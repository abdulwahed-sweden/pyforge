// Author: Abdulwahed Mansour
//! Rust-accelerated serialization for Django model instances and querysets.
//!
//! Converts `FieldValue` collections into JSON-compatible output, handling
//! Django-specific semantics like Decimal precision preservation and
//! timezone-aware datetime formatting.

use serde_json::{Map, Value as JsonValue};

use crate::error::DjangoError;
use crate::field_types::{FieldDescriptor, FieldValue};

/// A single serialized record — a map of field names to JSON values.
///
/// This is the Rust-side output that gets returned to Python as a dict.
/// Using `serde_json::Map` allows zero-copy conversion to Python dicts
/// via PyForge's serde integration.
pub type SerializedRecord = Map<String, JsonValue>;

/// Serializes a set of field values into a JSON-compatible record.
///
/// Handles Django-specific type conversions:
/// - `Decimal` is serialized as a string to preserve precision (matching DRF behavior).
/// - `DateTime` is serialized in ISO 8601 format with timezone suffix.
/// - `UUID` is serialized as its hyphenated string representation.
/// - `Null` becomes `serde_json::Value::Null`.
///
/// # Arguments
/// * `descriptors` - Field metadata extracted from the Django model.
/// * `values` - Corresponding field values, aligned by index with `descriptors`.
///
/// # Returns
/// A `SerializedRecord` (JSON object), or `DjangoError` if any field value
/// cannot be represented in the target format.
pub fn serialize_model_fields(
    descriptors: &[FieldDescriptor],
    values: &[FieldValue],
) -> Result<SerializedRecord, DjangoError> {
    if descriptors.len() != values.len() {
        return Err(DjangoError::Serialization {
            field: "<batch>".into(),
            message: format!(
                "descriptor count ({}) does not match value count ({})",
                descriptors.len(),
                values.len()
            ),
        });
    }

    let mut record = Map::with_capacity(descriptors.len());

    for (desc, val) in descriptors.iter().zip(values.iter()) {
        let json_val = field_value_to_json(val, &desc.name)?;
        record.insert(desc.name.clone(), json_val);
    }

    Ok(record)
}

/// Serializes multiple rows (e.g., a Django QuerySet) into a vector of records.
///
/// Each row is a slice of `FieldValue` aligned with the shared `descriptors`.
/// Suitable for serializing `Model.objects.all()` results in a single call.
///
/// # Arguments
/// * `descriptors` - Shared field metadata for all rows.
/// * `rows` - Each inner slice contains values for one model instance.
///
/// # Returns
/// A `Vec<SerializedRecord>`, or `DjangoError` on the first serialization failure.
pub fn serialize_queryset_rows(
    descriptors: &[FieldDescriptor],
    rows: &[Vec<FieldValue>],
) -> Result<Vec<SerializedRecord>, DjangoError> {
    rows.iter()
        .enumerate()
        .map(|(idx, values)| {
            serialize_model_fields(descriptors, values).map_err(|e| match e {
                DjangoError::Serialization { field, message } => DjangoError::Serialization {
                    field: format!("row[{idx}].{field}"),
                    message,
                },
                other => other,
            })
        })
        .collect()
}

/// Converts a single `FieldValue` into a `serde_json::Value`.
///
/// Decimal values are serialized as strings (not numbers) to match
/// Django REST Framework's default behavior and avoid precision loss.
fn field_value_to_json(value: &FieldValue, field_name: &str) -> Result<JsonValue, DjangoError> {
    match value {
        FieldValue::Text(s) => Ok(JsonValue::String(s.clone())),
        FieldValue::Integer(n) => Ok(JsonValue::Number((*n).into())),
        FieldValue::BigInteger(n) => Ok(JsonValue::Number((*n).into())),
        FieldValue::Float(f) => serde_json::Number::from_f64(*f)
            .map(JsonValue::Number)
            .ok_or_else(|| DjangoError::Serialization {
                field: field_name.into(),
                message: format!("float value {f} is not representable in JSON (NaN/Infinity)"),
            }),
        // Decimal → string to preserve precision (matches DRF DecimalField behavior)
        FieldValue::Decimal(d) => Ok(JsonValue::String(d.to_string())),
        FieldValue::Boolean(b) => Ok(JsonValue::Bool(*b)),
        FieldValue::Date(d) => Ok(JsonValue::String(d.format("%Y-%m-%d").to_string())),
        FieldValue::Time(t) => Ok(JsonValue::String(t.format("%H:%M:%S%.f").to_string())),
        FieldValue::DateTime(dt) => Ok(JsonValue::String(dt.to_rfc3339())),
        FieldValue::Uuid(u) => Ok(JsonValue::String(u.to_string())),
        FieldValue::Json(v) => Ok(v.clone()),
        FieldValue::Binary(bytes) => {
            // Base64 encode binary data for JSON transport
            use base64_encode::encode_to_string;
            Ok(JsonValue::String(encode_to_string(bytes)))
        }
        FieldValue::Null => Ok(JsonValue::Null),
    }
}

/// Minimal base64 encoder — avoids pulling in a full base64 crate for a single use.
mod base64_encode {
    const ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    pub fn encode_to_string(input: &[u8]) -> String {
        let mut output = String::with_capacity(input.len().div_ceil(3) * 4);
        for chunk in input.chunks(3) {
            let b0 = chunk[0] as u32;
            let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
            let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
            let triple = (b0 << 16) | (b1 << 8) | b2;
            output.push(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
            output.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
            if chunk.len() > 1 {
                output.push(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
            } else {
                output.push('=');
            }
            if chunk.len() > 2 {
                output.push(ALPHABET[(triple & 0x3F) as usize] as char);
            } else {
                output.push('=');
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_types::DjangoFieldType;
    use chrono::{NaiveDate, Utc};
    use rust_decimal::Decimal;
    use uuid::Uuid;

    fn make_descriptor(name: &str, field_type: DjangoFieldType) -> FieldDescriptor {
        FieldDescriptor {
            name: name.into(),
            field_type,
            nullable: false,
            has_default: false,
        }
    }

    #[test]
    fn serialize_text_field() {
        let descs = vec![make_descriptor("name", DjangoFieldType::TextField)];
        let vals = vec![FieldValue::Text("Alice".into())];
        let record = serialize_model_fields(&descs, &vals).unwrap();
        assert_eq!(record["name"], JsonValue::String("Alice".into()));
    }

    #[test]
    fn serialize_decimal_preserves_precision() {
        let descs = vec![make_descriptor(
            "price",
            DjangoFieldType::DecimalField {
                max_digits: 10,
                decimal_places: 2,
            },
        )];
        let vals = vec![FieldValue::Decimal(Decimal::new(19999, 2))];
        let record = serialize_model_fields(&descs, &vals).unwrap();
        // Must be a string "199.99", not a number
        assert_eq!(record["price"], JsonValue::String("199.99".into()));
    }

    #[test]
    fn serialize_null_becomes_json_null() {
        let descs = vec![make_descriptor("bio", DjangoFieldType::TextField)];
        let vals = vec![FieldValue::Null];
        let record = serialize_model_fields(&descs, &vals).unwrap();
        assert_eq!(record["bio"], JsonValue::Null);
    }

    #[test]
    fn serialize_datetime_uses_rfc3339() {
        let descs = vec![make_descriptor("created_at", DjangoFieldType::DateTimeField)];
        let dt = chrono::NaiveDate::from_ymd_opt(2025, 1, 15)
            .unwrap()
            .and_hms_opt(12, 30, 0)
            .unwrap()
            .and_utc();
        let vals = vec![FieldValue::DateTime(dt)];
        let record = serialize_model_fields(&descs, &vals).unwrap();
        let s = record["created_at"].as_str().unwrap();
        assert!(s.contains("2025-01-15"));
        assert!(s.contains("12:30:00"));
    }

    #[test]
    fn serialize_uuid_as_hyphenated_string() {
        let descs = vec![make_descriptor("id", DjangoFieldType::UuidField)];
        let id = Uuid::new_v4();
        let vals = vec![FieldValue::Uuid(id)];
        let record = serialize_model_fields(&descs, &vals).unwrap();
        assert_eq!(record["id"], JsonValue::String(id.to_string()));
    }

    #[test]
    fn serialize_queryset_multiple_rows() {
        let descs = vec![
            make_descriptor("name", DjangoFieldType::TextField),
            make_descriptor("active", DjangoFieldType::BooleanField),
        ];
        let rows = vec![
            vec![FieldValue::Text("Alice".into()), FieldValue::Boolean(true)],
            vec![FieldValue::Text("Bob".into()), FieldValue::Boolean(false)],
        ];
        let results = serialize_queryset_rows(&descs, &rows).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["name"], JsonValue::String("Alice".into()));
        assert_eq!(results[1]["active"], JsonValue::Bool(false));
    }

    #[test]
    fn serialize_mismatched_lengths_returns_error() {
        let descs = vec![make_descriptor("name", DjangoFieldType::TextField)];
        let vals = vec![]; // empty — mismatch
        let result = serialize_model_fields(&descs, &vals);
        assert!(result.is_err());
    }

    #[test]
    fn serialize_nan_float_returns_error() {
        let descs = vec![make_descriptor("score", DjangoFieldType::FloatField)];
        let vals = vec![FieldValue::Float(f64::NAN)];
        let result = serialize_model_fields(&descs, &vals);
        assert!(result.is_err());
    }

    #[test]
    fn base64_encode_binary_field() {
        let descs = vec![make_descriptor(
            "data",
            DjangoFieldType::BinaryField { max_length: None },
        )];
        let vals = vec![FieldValue::Binary(b"hello".to_vec())];
        let record = serialize_model_fields(&descs, &vals).unwrap();
        assert_eq!(record["data"], JsonValue::String("aGVsbG8=".into()));
    }
}
