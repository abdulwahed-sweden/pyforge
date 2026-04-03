// Author: Abdulwahed Mansour
//! # pyforge-django
//!
//! High-performance Django integration layer built on PyForge.
//!
//! Provides Rust-accelerated serialization, validation, and field mapping
//! for Django 5.x+ projects. Designed for drop-in use with Django REST Framework.

pub mod async_bridge;
pub mod error;
pub mod field_types;
pub mod model;
pub mod serializer;
pub mod validator;

use pyforge::prelude::*;
use pyforge::types::{PyDict, PyFloat, PyInt, PyList, PyString};

use crate::error::DjangoError;
use crate::field_types::{DjangoFieldType, FieldDescriptor, FieldValue};

// ─── Python-exposed functions ───────────────────────────────────────────────

/// Extracts field descriptors from a Django model class.
///
/// Introspects the model's `_meta` API and returns a list of field definitions
/// as Python dicts, each containing: name, type, nullable, has_default.
#[pyfunction]
fn extract_model_fields<'py>(
    py: Python<'py>,
    model_class: &Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyList>> {
    let descriptors = model::extract_field_descriptors(py, model_class)
        .map_err(|e| -> pyforge::PyErr { e.into() })?;

    let list = PyList::empty(py);
    for desc in &descriptors {
        let dict = PyDict::new(py);
        dict.set_item("name", &desc.name)?;
        dict.set_item("type", format!("{:?}", desc.field_type))?;
        dict.set_item("nullable", desc.nullable)?;
        dict.set_item("has_default", desc.has_default)?;
        list.append(dict)?;
    }
    Ok(list)
}

/// Serializes field values according to their descriptors into a Python dict.
///
/// Handles Django-specific semantics: Decimal as string, DateTime as ISO 8601,
/// UUID as hyphenated string, None as null.
#[pyfunction]
fn serialize_fields<'py>(
    py: Python<'py>,
    field_descriptors: &Bound<'py, PyList>,
    values: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyDict>> {
    let descriptors = extract_descriptor_list(field_descriptors)?;

    let mut field_values = Vec::with_capacity(descriptors.len());
    for desc in &descriptors {
        let py_val = values.get_item(&desc.name)?;
        match py_val {
            Some(val) => {
                let fv = model::convert_python_value_to_field(&val, desc)
                    .map_err(|e| -> pyforge::PyErr { e.into() })?;
                field_values.push(fv);
            }
            None => {
                if desc.nullable || desc.has_default {
                    field_values.push(FieldValue::Null);
                } else {
                    return Err(DjangoError::NullField {
                        field: desc.name.clone(),
                    }
                    .into());
                }
            }
        }
    }

    let record = serializer::serialize_model_fields(&descriptors, &field_values)
        .map_err(|e| -> pyforge::PyErr { e.into() })?;

    let output = PyDict::new(py);
    for (key, val) in &record {
        let py_val = json_value_to_pyobject(py, val)?;
        output.set_item(key, py_val)?;
    }
    Ok(output)
}

/// Validates a batch of field entries and returns a structured report.
///
/// Each entry is a pair of (field_descriptor_dict, value). For batches above
/// 64 entries, validation runs in parallel across CPU cores.
#[pyfunction]
fn validate_fields<'py>(
    py: Python<'py>,
    descriptors: &Bound<'py, PyList>,
    values: &Bound<'py, PyList>,
) -> PyResult<Bound<'py, PyDict>> {
    let descs = extract_descriptor_list(descriptors)?;

    let mut batch = Vec::with_capacity(descs.len());
    for (i, desc) in descs.into_iter().enumerate() {
        let py_val = values.get_item(i)?;
        let fv = if py_val.is_none() {
            FieldValue::Null
        } else {
            model::convert_python_value_to_field(&py_val, &desc)
                .map_err(|e| -> pyforge::PyErr { e.into() })?
        };
        batch.push((desc, fv));
    }

    let report = validator::validate_field_batch(&batch);

    let result = PyDict::new(py);
    result.set_item("valid_count", report.valid_count)?;
    result.set_item("error_count", report.error_count)?;

    let errors = PyList::empty(py);
    for err in &report.field_errors {
        let err_dict = PyDict::new(py);
        err_dict.set_item("field", &err.field_name)?;
        err_dict.set_item("message", &err.message)?;
        err_dict.set_item("code", &err.code)?;
        let params = PyDict::new(py);
        for (k, v) in &err.params {
            params.set_item(k, v)?;
        }
        err_dict.set_item("params", params)?;
        errors.append(err_dict)?;
    }
    result.set_item("errors", errors)?;

    Ok(result)
}

/// Returns the pyforge-django version string.
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// The `pyforge_django` Python module.
#[pymodule]
fn pyforge_django(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract_model_fields, m)?)?;
    m.add_function(wrap_pyfunction!(serialize_fields, m)?)?;
    m.add_function(wrap_pyfunction!(validate_fields, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}

// ─── Internal helpers ───────────────────────────────────────────────────────

/// Extracts a list of `FieldDescriptor` from a Python list of dicts.
fn extract_descriptor_list(py_list: &Bound<'_, PyList>) -> PyResult<Vec<FieldDescriptor>> {
    let mut descriptors = Vec::with_capacity(py_list.len());

    for item in py_list.iter() {
        let name: String = item.get_item("name")?.extract()?;
        let field_type_str: String = item.get_item("type")?.extract()?;
        let nullable: bool = item.get_item("nullable")?.extract()?;
        let has_default: bool = item.get_item("has_default")?.extract()?;

        let max_length: Option<usize> = item
            .get_item("max_length")
            .ok()
            .and_then(|v| v.extract().ok());
        let max_digits: Option<u32> = item
            .get_item("max_digits")
            .ok()
            .and_then(|v| v.extract().ok());
        let decimal_places: Option<u32> = item
            .get_item("decimal_places")
            .ok()
            .and_then(|v| v.extract().ok());

        let field_type = match field_type_str.as_str() {
            "CharField" => DjangoFieldType::CharField {
                max_length: max_length.unwrap_or(255),
            },
            "TextField" => DjangoFieldType::TextField,
            "IntegerField" => DjangoFieldType::IntegerField,
            "BigIntegerField" => DjangoFieldType::BigIntegerField,
            "FloatField" => DjangoFieldType::FloatField,
            "DecimalField" => DjangoFieldType::DecimalField {
                max_digits: max_digits.unwrap_or(10),
                decimal_places: decimal_places.unwrap_or(2),
            },
            "BooleanField" => DjangoFieldType::BooleanField,
            "DateField" => DjangoFieldType::DateField,
            "TimeField" => DjangoFieldType::TimeField,
            "DateTimeField" => DjangoFieldType::DateTimeField,
            "UUIDField" => DjangoFieldType::UuidField,
            "JSONField" => DjangoFieldType::JsonField,
            "BinaryField" => DjangoFieldType::BinaryField { max_length },
            "EmailField" => DjangoFieldType::EmailField {
                max_length: max_length.unwrap_or(254),
            },
            "URLField" => DjangoFieldType::UrlField {
                max_length: max_length.unwrap_or(200),
            },
            "SlugField" => DjangoFieldType::SlugField {
                max_length: max_length.unwrap_or(50),
            },
            _ => DjangoFieldType::TextField,
        };

        descriptors.push(FieldDescriptor {
            name,
            field_type,
            nullable,
            has_default,
        });
    }

    Ok(descriptors)
}

/// Converts a `serde_json::Value` into a Python object.
///
/// Uses PyForge's primitive constructors to avoid ownership issues with
/// `IntoPyObject` trait's varying return types (Bound vs Borrowed).
fn json_value_to_pyobject<'py>(
    py: Python<'py>,
    value: &serde_json::Value,
) -> PyResult<Bound<'py, PyAny>> {
    match value {
        serde_json::Value::Null => Ok(py.None().into_bound(py)),
        serde_json::Value::Bool(b) => Ok(PyInt::new(py, if *b { 1i32 } else { 0i32 }).into_any()),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(PyInt::new(py, i).into_any())
            } else if let Some(f) = n.as_f64() {
                Ok(PyFloat::new(py, f).into_any())
            } else {
                Ok(PyString::new(py, &n.to_string()).into_any())
            }
        }
        serde_json::Value::String(s) => Ok(PyString::new(py, s).into_any()),
        serde_json::Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_value_to_pyobject(py, item)?)?;
            }
            Ok(list.into_any())
        }
        serde_json::Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_value_to_pyobject(py, v)?)?;
            }
            Ok(dict.into_any())
        }
    }
}
