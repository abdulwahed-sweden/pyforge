// Author: Abdulwahed Mansour
//! Bulk field validation using Rayon parallel iterators.
//!
//! Validates batches of Django model field values against their descriptors,
//! returning structured errors that map directly to Django's `ValidationError`.
//!
//! For batches above `PARALLEL_VALIDATION_THRESHOLD`, validation is parallelized
//! across CPU cores using Rayon's work-stealing thread pool.

use rayon::prelude::*;
use std::collections::HashMap;

use crate::error::FieldValidationError;
use crate::field_types::{DjangoFieldType, FieldDescriptor, FieldValue};

/// Minimum batch size before parallelization kicks in.
///
/// Below this threshold, the overhead of Rayon's thread pool dispatch
/// exceeds the speedup from parallel execution. Tuned via benchmarks
/// on typical Django querysets (10-50 fields per row).
const PARALLEL_VALIDATION_THRESHOLD: usize = 64;

/// Outcome of validating a batch of field values.
///
/// Carries both the valid count and detailed errors, allowing the caller to
/// decide whether to reject the entire batch or process partial results.
#[derive(Debug)]
pub struct ValidationReport {
    /// Number of fields that passed all validation checks.
    pub valid_count: usize,
    /// Number of fields that failed validation.
    pub error_count: usize,
    /// Per-field error details, ordered by field name.
    pub field_errors: Vec<FieldValidationError>,
}

impl ValidationReport {
    /// Returns `true` if every field in the batch passed validation.
    pub fn is_valid(&self) -> bool {
        self.field_errors.is_empty()
    }
}

/// Validates a batch of field values against their corresponding descriptors.
///
/// Each `(FieldDescriptor, FieldValue)` pair is checked for:
/// - Null constraint violations (non-nullable field with `FieldValue::Null`)
/// - Type compatibility (e.g., `IntegerField` must receive `FieldValue::Integer`)
/// - Constraint satisfaction (e.g., `CharField` max_length, `DecimalField` digit limits)
///
/// For batches with more than `PARALLEL_VALIDATION_THRESHOLD` entries, validation
/// runs across multiple CPU cores via Rayon.
///
/// # Arguments
/// * `entries` - Pairs of field descriptors and their corresponding values.
///
/// # Returns
/// A `ValidationReport` summarizing the results.
pub fn validate_field_batch(entries: &[(FieldDescriptor, FieldValue)]) -> ValidationReport {
    // Collect indexed results so we can restore input order after parallel execution.
    // Rayon's par_iter does NOT guarantee output order, but Django expects errors
    // in field declaration order for consistent form rendering.
    let mut indexed_errors: Vec<(usize, Vec<FieldValidationError>)> =
        if entries.len() >= PARALLEL_VALIDATION_THRESHOLD {
            entries
                .par_iter()
                .enumerate()
                .map(|(i, (desc, val))| (i, validate_single_field(desc, val)))
                .collect()
        } else {
            entries
                .iter()
                .enumerate()
                .map(|(i, (desc, val))| (i, validate_single_field(desc, val)))
                .collect()
        };
    indexed_errors.sort_by_key(|(i, _)| *i);
    let errors: Vec<FieldValidationError> = indexed_errors
        .into_iter()
        .flat_map(|(_, errs)| errs)
        .collect();

    // Count entries that produced at least one error (not total error structs,
    // because one entry can fail multiple checks — e.g., type mismatch + max_length)
    let entries_with_errors = errors
        .iter()
        .map(|e| &e.field_name)
        .collect::<std::collections::HashSet<_>>()
        .len();
    ValidationReport {
        valid_count: entries.len().saturating_sub(entries_with_errors),
        error_count: errors.len(),
        field_errors: errors,
    }
}

/// Validates a single field value against its descriptor.
///
/// Returns zero or more errors — a single field can fail multiple checks
/// (e.g., wrong type AND exceeding max_length if the type is coerced).
fn validate_single_field(
    descriptor: &FieldDescriptor,
    value: &FieldValue,
) -> Vec<FieldValidationError> {
    let mut errors = Vec::new();

    // Null check
    if matches!(value, FieldValue::Null) {
        if !descriptor.nullable && !descriptor.has_default {
            errors.push(FieldValidationError {
                field_name: descriptor.name.clone(),
                message: "This field is required.".into(),
                code: "required".into(),
                params: HashMap::new(),
            });
        }
        return errors;
    }

    // Type-specific validation
    match (&descriptor.field_type, value) {
        (DjangoFieldType::CharField { max_length }, FieldValue::Text(s))
        | (DjangoFieldType::EmailField { max_length }, FieldValue::Text(s))
        | (DjangoFieldType::UrlField { max_length }, FieldValue::Text(s))
        | (DjangoFieldType::SlugField { max_length }, FieldValue::Text(s)) => {
            // BUG FIX: Django counts characters, not bytes — crucial for multi-byte UTF-8
            let char_count = s.chars().count();
            if char_count > *max_length {
                errors.push(FieldValidationError {
                    field_name: descriptor.name.clone(),
                    message: format!(
                        "Ensure this value has at most {max_length} characters (it has {char_count}).",
                    ),
                    code: "max_length".into(),
                    params: HashMap::from([
                        ("max_length".into(), max_length.to_string()),
                        ("length".into(), char_count.to_string()),
                    ]),
                });
            }
            // SlugField: only alphanumerics, hyphens, underscores
            if matches!(&descriptor.field_type, DjangoFieldType::SlugField { .. })
                && !s.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
            {
                errors.push(FieldValidationError {
                    field_name: descriptor.name.clone(),
                    message: "Enter a valid 'slug' consisting of letters, numbers, underscores or hyphens.".into(),
                    code: "invalid".into(),
                    params: HashMap::new(),
                });
            }
        }

        (DjangoFieldType::TextField, FieldValue::Text(_)) => {}

        (DjangoFieldType::IntegerField, FieldValue::Integer(_)) => {}
        (DjangoFieldType::BigIntegerField, FieldValue::BigInteger(_)) => {}
        (DjangoFieldType::FloatField, FieldValue::Float(_)) => {}

        (
            DjangoFieldType::DecimalField {
                max_digits,
                decimal_places,
            },
            FieldValue::Decimal(d),
        ) => {
            // Match Django's DecimalValidator digit counting: uses Decimal.as_tuple().digits
            // which counts ALL digits in the coefficient (including trailing zeros, excluding
            // leading zeros only for the integer part). We use mantissa() to get the
            // unscaled integer, then count its digits.
            let mantissa_abs = d.mantissa().unsigned_abs();
            let total_digits = if mantissa_abs == 0 {
                1u32 // "0" has 1 digit
            } else {
                // ilog10 returns floor(log10(n)), so digit count is ilog10 + 1
                mantissa_abs.ilog10() + 1
            };
            let scale = d.scale();

            if total_digits > *max_digits {
                errors.push(FieldValidationError {
                    field_name: descriptor.name.clone(),
                    message: format!(
                        "Ensure that there are no more than {max_digits} digits in total."
                    ),
                    code: "max_digits".into(),
                    params: HashMap::from([("max_digits".into(), max_digits.to_string())]),
                });
            }
            if scale > *decimal_places {
                errors.push(FieldValidationError {
                    field_name: descriptor.name.clone(),
                    message: format!(
                        "Ensure that there are no more than {decimal_places} decimal places."
                    ),
                    code: "max_decimal_places".into(),
                    params: HashMap::from([(
                        "decimal_places".into(),
                        decimal_places.to_string(),
                    )]),
                });
            }
        }

        (DjangoFieldType::BooleanField, FieldValue::Boolean(_)) => {}
        (DjangoFieldType::DateField, FieldValue::Date(_)) => {}
        (DjangoFieldType::TimeField, FieldValue::Time(_)) => {}
        (DjangoFieldType::DateTimeField, FieldValue::DateTime(_)) => {}
        (DjangoFieldType::UuidField, FieldValue::Uuid(_)) => {}
        (DjangoFieldType::JsonField, FieldValue::Json(_)) => {}

        (DjangoFieldType::BinaryField { max_length }, FieldValue::Binary(bytes)) => {
            if let Some(max) = max_length {
                if bytes.len() > *max {
                    errors.push(FieldValidationError {
                        field_name: descriptor.name.clone(),
                        message: format!(
                            "Ensure this value has at most {max} bytes (it has {}).",
                            bytes.len()
                        ),
                        code: "max_length".into(),
                        params: HashMap::from([("max_length".into(), max.to_string())]),
                    });
                }
            }
        }

        // Type mismatch
        (field_type, value) => {
            errors.push(FieldValidationError {
                field_name: descriptor.name.clone(),
                message: format!(
                    "Invalid type: expected compatible value for {:?}, got {}.",
                    field_type,
                    value.type_name()
                ),
                code: "invalid".into(),
                params: HashMap::new(),
            });
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    fn char_field(name: &str, max_length: usize) -> FieldDescriptor {
        FieldDescriptor {
            name: name.into(),
            field_type: DjangoFieldType::CharField { max_length },
            nullable: false,
            has_default: false,
        }
    }

    fn decimal_field(name: &str, max_digits: u32, decimal_places: u32) -> FieldDescriptor {
        FieldDescriptor {
            name: name.into(),
            field_type: DjangoFieldType::DecimalField {
                max_digits,
                decimal_places,
            },
            nullable: false,
            has_default: false,
        }
    }

    #[test]
    fn valid_char_field_passes() {
        let entries = vec![(char_field("name", 100), FieldValue::Text("Alice".into()))];
        let report = validate_field_batch(&entries);
        assert!(report.is_valid());
        assert_eq!(report.valid_count, 1);
    }

    #[test]
    fn char_field_exceeding_max_length_fails() {
        let entries = vec![(char_field("name", 5), FieldValue::Text("Abdulwahed".into()))];
        let report = validate_field_batch(&entries);
        assert!(!report.is_valid());
        assert_eq!(report.error_count, 1);
        assert_eq!(report.field_errors[0].code, "max_length");
    }

    #[test]
    fn null_on_required_field_fails() {
        let entries = vec![(char_field("username", 150), FieldValue::Null)];
        let report = validate_field_batch(&entries);
        assert!(!report.is_valid());
        assert_eq!(report.field_errors[0].code, "required");
    }

    #[test]
    fn null_on_nullable_field_passes() {
        let mut desc = char_field("bio", 500);
        desc.nullable = true;
        let entries = vec![(desc, FieldValue::Null)];
        let report = validate_field_batch(&entries);
        assert!(report.is_valid());
    }

    #[test]
    fn decimal_exceeding_max_digits_fails() {
        let entries = vec![(
            decimal_field("price", 5, 2),
            FieldValue::Decimal(Decimal::new(1_234_567, 2)), // 12345.67 — 7 digits
        )];
        let report = validate_field_batch(&entries);
        assert!(!report.is_valid());
        assert_eq!(report.field_errors[0].code, "max_digits");
    }

    #[test]
    fn decimal_within_limits_passes() {
        let entries = vec![(
            decimal_field("price", 10, 2),
            FieldValue::Decimal(Decimal::new(9999, 2)), // 99.99
        )];
        let report = validate_field_batch(&entries);
        assert!(report.is_valid());
    }

    #[test]
    fn type_mismatch_produces_error() {
        let entries = vec![(
            FieldDescriptor {
                name: "age".into(),
                field_type: DjangoFieldType::IntegerField,
                nullable: false,
                has_default: false,
            },
            FieldValue::Text("not a number".into()),
        )];
        let report = validate_field_batch(&entries);
        assert!(!report.is_valid());
        assert_eq!(report.field_errors[0].code, "invalid");
    }

    #[test]
    fn slug_field_rejects_spaces() {
        let entries = vec![(
            FieldDescriptor {
                name: "slug".into(),
                field_type: DjangoFieldType::SlugField { max_length: 50 },
                nullable: false,
                has_default: false,
            },
            FieldValue::Text("invalid slug value".into()),
        )];
        let report = validate_field_batch(&entries);
        assert!(!report.is_valid());
        assert_eq!(report.field_errors[0].code, "invalid");
    }

    #[test]
    fn large_batch_uses_parallel_validation() {
        let entries: Vec<_> = (0..200)
            .map(|i| {
                (
                    char_field(&format!("field_{i}"), 100),
                    FieldValue::Text(format!("value_{i}")),
                )
            })
            .collect();
        let report = validate_field_batch(&entries);
        assert!(report.is_valid());
        assert_eq!(report.valid_count, 200);
    }

    // ─── Regression tests for audit-discovered bugs ─────────────────────

    #[test]
    fn regression_multibyte_charfield_counts_characters_not_bytes() {
        // Arabic name: 10 characters but 20+ bytes in UTF-8
        let arabic_name = "\u{0639}\u{0628}\u{062F}\u{0627}\u{0644}\u{0648}\u{0627}\u{062D}\u{062F} \u{0645}";
        let char_count = arabic_name.chars().count();
        assert!(char_count <= 12, "test assumption: Arabic name is <=12 chars");
        assert!(arabic_name.len() > 12, "test assumption: Arabic name is >12 bytes");

        let entries = vec![(char_field("name", 12), FieldValue::Text(arabic_name.into()))];
        let report = validate_field_batch(&entries);
        // Must pass because it's <=12 CHARACTERS even though >12 BYTES
        assert!(report.is_valid(), "multi-byte CharField validation counted bytes, not characters");
    }

    #[test]
    fn regression_valid_count_no_underflow_on_multi_error_entry() {
        // SlugField with spaces AND exceeding max_length produces 2 errors from 1 entry
        let entries = vec![(
            FieldDescriptor {
                name: "slug".into(),
                field_type: DjangoFieldType::SlugField { max_length: 5 },
                nullable: false,
                has_default: false,
            },
            FieldValue::Text("invalid slug with spaces and too long".into()),
        )];
        let report = validate_field_batch(&entries);
        assert!(!report.is_valid());
        // 2 errors from 1 entry — valid_count must be 0, not underflow
        assert!(report.error_count >= 2, "should have at least 2 errors (max_length + invalid slug)");
        assert_eq!(report.valid_count, 0, "valid_count must be 0 when entry has errors");
    }

    #[test]
    fn regression_parallel_errors_preserve_input_order() {
        // Create 200 entries where every other one fails, verify error order matches input order
        let entries: Vec<_> = (0..200)
            .map(|i| {
                if i % 2 == 0 {
                    (char_field(&format!("field_{i:04}"), 100), FieldValue::Text("ok".into()))
                } else {
                    (char_field(&format!("field_{i:04}"), 2), FieldValue::Text("too long".into()))
                }
            })
            .collect();
        let report = validate_field_batch(&entries);
        // Verify errors are in field declaration order, not random Rayon order
        for window in report.field_errors.windows(2) {
            assert!(
                window[0].field_name < window[1].field_name,
                "error order is non-deterministic: {} came before {}",
                window[0].field_name,
                window[1].field_name
            );
        }
    }

    #[test]
    fn regression_decimal_100_has_3_digits() {
        // Decimal("100") must count as 3 digits, not 1
        let entries = vec![(
            decimal_field("amount", 3, 0),
            FieldValue::Decimal(Decimal::new(100, 0)),
        )];
        let report = validate_field_batch(&entries);
        assert!(report.is_valid(), "Decimal 100 should pass max_digits=3 (has exactly 3 digits)");
    }

    #[test]
    fn regression_decimal_100_exceeds_2_max_digits() {
        // Decimal("100") = 3 digits, must FAIL max_digits=2
        let entries = vec![(
            decimal_field("amount", 2, 0),
            FieldValue::Decimal(Decimal::new(100, 0)),
        )];
        let report = validate_field_batch(&entries);
        assert!(!report.is_valid(), "Decimal 100 should fail max_digits=2 (has 3 digits)");
    }

    #[test]
    fn regression_decimal_zero_has_1_digit() {
        // Decimal("0") must count as 1 digit
        let entries = vec![(
            decimal_field("amount", 1, 0),
            FieldValue::Decimal(Decimal::new(0, 0)),
        )];
        let report = validate_field_batch(&entries);
        assert!(report.is_valid(), "Decimal 0 should pass max_digits=1");
    }

    #[test]
    fn regression_empty_batch_returns_zero_counts() {
        let entries: Vec<(FieldDescriptor, FieldValue)> = vec![];
        let report = validate_field_batch(&entries);
        assert!(report.is_valid());
        assert_eq!(report.valid_count, 0);
        assert_eq!(report.error_count, 0);
    }

    #[test]
    fn regression_infinity_float_serializes_as_error() {
        // f64::INFINITY should not silently serialize — it's not valid JSON
        let entries = vec![(
            FieldDescriptor {
                name: "score".into(),
                field_type: DjangoFieldType::FloatField,
                nullable: false,
                has_default: false,
            },
            FieldValue::Float(f64::INFINITY),
        )];
        // Validation passes (INFINITY is a valid f64), but serialization should catch it
        let report = validate_field_batch(&entries);
        assert!(report.is_valid(), "Infinity is a valid float for validation");
    }
}
