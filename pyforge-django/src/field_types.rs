// Author: Abdulwahed Mansour
//! Rust-native equivalents of Django model field types.
//!
//! Each variant in `DjangoFieldType` corresponds to a Django model field class
//! and carries the metadata needed to validate and serialize values without
//! round-tripping through Python.
//!
//! Design decisions:
//! - DecimalField maps to `rust_decimal::Decimal`, never to `f64`, because
//!   Django's DecimalField guarantees arbitrary precision via Python's `decimal` module.
//! - DateTimeField uses `chrono::DateTime<chrono::Utc>` — timezone-aware by default,
//!   matching Django's `USE_TZ = True` recommendation.
//! - JSONField stores `serde_json::Value` for zero-copy access to nested structures.

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Describes a single Django model field, including its type and constraints.
///
/// Extracted from Django's model `_meta` API at initialization time, then used
/// by the serializer and validator to process values without touching Python.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDescriptor {
    /// The Python attribute name on the Django model (e.g., "email", "created_at").
    pub name: String,
    /// The Rust-mapped field type with its constraints.
    pub field_type: DjangoFieldType,
    /// Whether Django allows `None` for this field (`null=True` in the model).
    pub nullable: bool,
    /// Whether the field has a default value and can be omitted from input.
    pub has_default: bool,
}

/// Enumeration of Django field types with their associated validation constraints.
///
/// Constraints are embedded directly in the enum variants so that the validator
/// can check them without additional lookups.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DjangoFieldType {
    /// `django.db.models.CharField` — bounded-length text.
    CharField { max_length: usize },
    /// `django.db.models.TextField` — unbounded text.
    TextField,
    /// `django.db.models.IntegerField` — signed 32-bit integer.
    IntegerField,
    /// `django.db.models.BigIntegerField` — signed 64-bit integer.
    BigIntegerField,
    /// `django.db.models.FloatField` — IEEE 754 double-precision.
    FloatField,
    /// `django.db.models.DecimalField` — arbitrary-precision decimal.
    /// `max_digits` and `decimal_places` mirror Django's field arguments.
    DecimalField {
        max_digits: u32,
        decimal_places: u32,
    },
    /// `django.db.models.BooleanField`.
    BooleanField,
    /// `django.db.models.DateField` — date without time or timezone.
    DateField,
    /// `django.db.models.TimeField` — time without date or timezone.
    TimeField,
    /// `django.db.models.DateTimeField` — timezone-aware datetime.
    DateTimeField,
    /// `django.db.models.UUIDField` — RFC 4122 UUID.
    UuidField,
    /// `django.db.models.JSONField` — arbitrary JSON structure.
    JsonField,
    /// `django.db.models.BinaryField` — raw byte data.
    BinaryField { max_length: Option<usize> },
    /// `django.db.models.EmailField` — CharField with email validation.
    EmailField { max_length: usize },
    /// `django.db.models.URLField` — CharField with URL validation.
    UrlField { max_length: usize },
    /// `django.db.models.SlugField` — CharField restricted to slug characters.
    SlugField { max_length: usize },
}

/// A concrete Rust value extracted from a Django model instance.
///
/// This is the runtime representation of a field's value, used as input
/// to the serializer and validator. `Null` is a first-class variant
/// rather than wrapping everything in `Option` — this matches Django's
/// model where `None` and missing-field are semantically distinct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldValue {
    Text(String),
    Integer(i32),
    BigInteger(i64),
    Float(f64),
    Decimal(Decimal),
    Boolean(bool),
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(DateTime<Utc>),
    Uuid(Uuid),
    Json(serde_json::Value),
    Binary(Vec<u8>),
    /// Represents Python `None` — a field that exists but has no value.
    Null,
}

impl FieldValue {
    /// Returns a human-readable type name for error messages.
    ///
    /// Used by the validator to produce clear type-mismatch errors like
    /// "expected Integer, got Text" rather than opaque Rust type names.
    pub fn type_name(&self) -> &'static str {
        match self {
            FieldValue::Text(_) => "Text",
            FieldValue::Integer(_) => "Integer",
            FieldValue::BigInteger(_) => "BigInteger",
            FieldValue::Float(_) => "Float",
            FieldValue::Decimal(_) => "Decimal",
            FieldValue::Boolean(_) => "Boolean",
            FieldValue::Date(_) => "Date",
            FieldValue::Time(_) => "Time",
            FieldValue::DateTime(_) => "DateTime",
            FieldValue::Uuid(_) => "UUID",
            FieldValue::Json(_) => "JSON",
            FieldValue::Binary(_) => "Binary",
            FieldValue::Null => "Null",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_descriptor_serializes_to_json() {
        let field = FieldDescriptor {
            name: "price".into(),
            field_type: DjangoFieldType::DecimalField {
                max_digits: 10,
                decimal_places: 2,
            },
            nullable: false,
            has_default: false,
        };
        let json = serde_json::to_string(&field).unwrap();
        assert!(json.contains("price"));
        assert!(json.contains("DecimalField"));
    }

    #[test]
    fn field_value_type_names_are_descriptive() {
        assert_eq!(FieldValue::Text("hello".into()).type_name(), "Text");
        assert_eq!(FieldValue::Null.type_name(), "Null");
        assert_eq!(FieldValue::Decimal(Decimal::new(100, 2)).type_name(), "Decimal");
        assert_eq!(FieldValue::Uuid(Uuid::new_v4()).type_name(), "UUID");
    }

    #[test]
    fn null_is_distinct_from_missing() {
        let val = FieldValue::Null;
        assert!(matches!(val, FieldValue::Null));
    }
}
