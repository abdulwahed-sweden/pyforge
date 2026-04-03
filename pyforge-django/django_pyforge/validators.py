# Author: Abdulwahed Mansour
"""
Drop-in Rust-accelerated validators for Django model fields.

These validators work as standard Django validators — they accept a value
and raise django.core.exceptions.ValidationError on failure. Internally,
they delegate to the Rust validation engine for batch processing.

Usage:
    from django_pyforge.validators import validate_field_batch

    errors = validate_field_batch(field_entries)
    if errors["error_count"] > 0:
        raise ValidationError(errors["errors"])
"""

from django_pyforge import validate_fields as _rust_validate


def validate_field_batch(descriptors, values):
    """
    Validate a batch of field values using the Rust validation engine.

    For batches above 64 entries, validation runs in parallel across CPU cores.

    Args:
        descriptors: A list of field descriptor dicts, each with keys:
            name (str), type (str), nullable (bool), has_default (bool),
            and optional max_length, max_digits, decimal_places.
        values: A list of Python values aligned by index with descriptors.

    Returns:
        A dict with keys:
            valid_count (int): Number of fields that passed validation.
            error_count (int): Number of fields that failed.
            errors (list[dict]): Each error has field, message, code, params.
    """
    return _rust_validate(descriptors, values)


def validate_model_instance(instance, field_descriptors=None):
    """
    Validate all fields of a Django model instance.

    Extracts field values from the instance and runs them through Rust
    validation. Returns a structured report suitable for conversion to
    Django's ValidationError.

    Args:
        instance: A Django model instance.
        field_descriptors: Optional pre-extracted field descriptors.
            If None, they are extracted from the model class.

    Returns:
        A dict with valid_count, error_count, and errors.
    """
    if field_descriptors is None:
        from django_pyforge import extract_model_fields
        field_descriptors = extract_model_fields(type(instance))

    values = []
    for desc in field_descriptors:
        try:
            values.append(getattr(instance, desc["name"], None))
        except Exception:
            values.append(None)

    return _rust_validate(field_descriptors, values)
