# Author: Abdulwahed Mansour
"""
django_pyforge — High-performance Django integration powered by Rust.

Drop-in accelerators for Django REST Framework serializers and validators.
Requires pyforge_django native extension (built via maturin).
"""

try:
    from pyforge_django import (
        extract_model_fields,
        serialize_fields,
        validate_fields,
        version,
    )
except ImportError as exc:
    raise ImportError(
        "pyforge_django native extension not found. "
        "Install with: pip install pyforge-django"
    ) from exc

__version__ = version()
__all__ = [
    "extract_model_fields",
    "serialize_fields",
    "validate_fields",
]
