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
    let errors: Vec<FieldValidationError> = if entries.len() >= PARALLEL_VALIDATION_THRESHOLD {
        entries
            .par_iter()
            .flat_map(|(desc, val)| validate_single_field(desc, val))
            .collect()
    } else {
        entries
            .iter()
            .flat_map(|(desc, val)| validate_single_field(desc, val))
            .collect()
    };

    let error_count = errors.len();
    ValidationReport {
        valid_count: entries.len() - error_count,
        error_count,
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
            if s.len() > *max_length {
                errors.push(FieldValidationError {
                    field_name: descriptor.name.clone(),
                    message: format!(
                        "Ensure this value has at most {max_length} characters (it has {}).",
                        s.len()
                    ),
                    code: "max_length".into(),
                    params: HashMap::from([
                        ("max_length".into(), max_length.to_string()),
                        ("length".into(), s.len().to_string()),
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
            // Count significant digits via string representation (matches Django's check)
            let abs_str = d.to_string().replace(['-', '.'], "");
            let total_digits = abs_str.trim_start_matches('0').len() as u32;
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
}
