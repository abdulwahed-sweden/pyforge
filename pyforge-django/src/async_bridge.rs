// Author: Abdulwahed Mansour
//! ASGI-compatible async context handling for Django.
//!
//! Provides utilities for running Rust-accelerated operations within Django's
//! async view layer (ASGI) without blocking the event loop. This module bridges
//! PyForge's coroutine support with Django's async view dispatch.

use pyforge::prelude::*;

use crate::error::DjangoError;
use crate::field_types::{FieldDescriptor, FieldValue};
use crate::serializer::{serialize_model_fields, SerializedRecord};
use crate::validator::{validate_field_batch, ValidationReport};

/// Serializes model fields while releasing the Python GIL during computation.
///
/// Intended for use from Django async views where holding the GIL would block
/// other coroutines. The caller must provide pre-extracted `FieldValue` data
/// (Python-to-Rust conversion happens before this call, while holding the GIL).
///
/// # Arguments
/// * `py` - The Python GIL token, released during the Rust computation phase.
/// * `descriptors` - Field metadata extracted from the Django model.
/// * `values` - Field values already converted from Python objects.
///
/// # Returns
/// A `SerializedRecord` (JSON-compatible map), or `DjangoError` on failure.
pub fn serialize_fields_release_gil(
    py: Python<'_>,
    descriptors: Vec<FieldDescriptor>,
    values: Vec<FieldValue>,
) -> Result<SerializedRecord, DjangoError> {
    py.detach(|| serialize_model_fields(&descriptors, &values))
}

/// Validates a field batch while releasing the Python GIL.
///
/// For large batches, this enables Rayon's parallel validation to run across
/// multiple CPU cores without being bottlenecked by the GIL.
///
/// # Arguments
/// * `py` - The Python GIL token, released during validation.
/// * `entries` - Pairs of field descriptors and values to validate.
///
/// # Returns
/// A `ValidationReport` with per-field error details.
pub fn validate_batch_release_gil(
    py: Python<'_>,
    entries: Vec<(FieldDescriptor, FieldValue)>,
) -> ValidationReport {
    py.detach(|| validate_field_batch(&entries))
}
