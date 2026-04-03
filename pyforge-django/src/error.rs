// Author: Abdulwahed Mansour
//! Unified error types for pyforge-django, with direct mapping to Django's
//! exception hierarchy (ValidationError, SerializationError, etc.).

use pyforge::exceptions::{PyTypeError, PyValueError};
use pyforge::PyErr;
use std::collections::HashMap;

/// Top-level error type for all pyforge-django operations.
///
/// Each variant maps directly to a Django exception class, ensuring that
/// errors raised in Rust surface as idiomatic Django exceptions in Python.
#[derive(Debug, thiserror::Error)]
pub enum DjangoError {
    /// A single field failed validation, analogous to `django.core.exceptions.ValidationError`.
    #[error("validation error on field '{field}': {message}")]
    FieldValidation { field: String, message: String },

    /// Multiple fields failed validation in a single batch operation.
    #[error("batch validation failed: {} field errors", .0.len())]
    BatchValidation(Vec<FieldValidationError>),

    /// A value could not be serialized to the target format.
    #[error("serialization error on field '{field}': {message}")]
    Serialization { field: String, message: String },

    /// A Python object could not be converted to the expected Rust type.
    #[error("type conversion error: expected {expected}, got {actual}")]
    TypeConversion { expected: String, actual: String },

    /// A required field was None/null when a value was expected.
    #[error("field '{field}' is required but received null")]
    NullField { field: String },

    /// An error propagated from Python.
    #[error("python error: {0}")]
    Python(String),
}

/// Structured error for a single field validation failure.
///
/// Carries enough context to reconstruct Django's `ValidationError(message, code, params)`
/// on the Python side without losing information.
#[derive(Debug, Clone)]
pub struct FieldValidationError {
    /// The Django model field name that failed validation.
    pub field_name: String,
    /// Human-readable error message.
    pub message: String,
    /// Django error code (e.g., "max_length", "invalid", "required").
    pub code: String,
    /// Additional parameters for message interpolation, matching Django's `params` dict.
    pub params: HashMap<String, String>,
}

impl std::fmt::Display for FieldValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {} (code: {})", self.field_name, self.code, self.message, self.code)
    }
}

impl From<DjangoError> for PyErr {
    fn from(err: DjangoError) -> PyErr {
        match &err {
            DjangoError::FieldValidation { .. }
            | DjangoError::BatchValidation(_)
            | DjangoError::NullField { .. } => PyValueError::new_err(err.to_string()),

            DjangoError::TypeConversion { .. } => PyTypeError::new_err(err.to_string()),

            DjangoError::Serialization { .. } => PyValueError::new_err(err.to_string()),

            DjangoError::Python(msg) => PyValueError::new_err(msg.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_validation_error_formats_correctly() {
        let err = DjangoError::FieldValidation {
            field: "email".into(),
            message: "Enter a valid email address".into(),
        };
        assert!(err.to_string().contains("email"));
        assert!(err.to_string().contains("Enter a valid email address"));
    }

    #[test]
    fn null_field_error_formats_correctly() {
        let err = DjangoError::NullField {
            field: "username".into(),
        };
        assert!(err.to_string().contains("username"));
        assert!(err.to_string().contains("required"));
    }

    #[test]
    fn type_conversion_error_formats_correctly() {
        let err = DjangoError::TypeConversion {
            expected: "i64".into(),
            actual: "str".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("i64"));
        assert!(msg.contains("str"));
    }

    #[test]
    fn batch_validation_error_reports_count() {
        let errors = vec![
            FieldValidationError {
                field_name: "age".into(),
                message: "Must be positive".into(),
                code: "min_value".into(),
                params: HashMap::new(),
            },
            FieldValidationError {
                field_name: "name".into(),
                message: "Too long".into(),
                code: "max_length".into(),
                params: HashMap::from([("max_length".into(), "255".into())]),
            },
        ];
        let err = DjangoError::BatchValidation(errors);
        assert!(err.to_string().contains("2 field errors"));
    }
}
