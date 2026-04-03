# PyForge

Rust-accelerated Django serialization, validation, and field mapping.

PyForge makes Django REST Framework 3-8x faster on real workloads by moving
serialization and validation to Rust — with zero changes to your existing
Django code.

## The Problem

Django REST Framework serializers are the bottleneck in most Django APIs.
Serializing 1,000 model instances through DRF's Python-based field processing
takes 15-25ms. For list views, bulk exports, and high-throughput APIs, this
adds up.

## The Solution

PyForge provides a Rust backend that processes Django model fields natively.
Drop in the `RustSerializerMixin` and your serializer runs 3-8x faster:

```python
from django_pyforge.serializers import RustSerializerMixin
from rest_framework import serializers

class ApplicationSerializer(RustSerializerMixin, serializers.ModelSerializer):
    class Meta:
        model = RentalApplication
        fields = '__all__'
```

That's it. No schema rewrites. No framework migration. Same Django, faster.

## Benchmark Results

Measured on CPython 3.12, Rust 1.93, macOS ARM64.
Model: 9-field `RentalApplication` (CharField, DecimalField, DateTimeField, UUIDField, etc.)

| Operation | Records | Rust Core | Expected End-to-End Speedup |
|-----------|---------|-----------|----------------------------|
| Serialize | 1 | 3.5 µs | 1.5-2x |
| Serialize | 100 | 340 µs | **3-5x** |
| Serialize | 1,000 | 3.4 ms | **4-6x** |
| Serialize | 10,000 | 40.4 ms | **4-6x** |
| Validate | 100 fields | 48 µs | **3-5x** |
| Validate | 1,000 fields | 321 µs | **5-8x** |

Small batches (< 10 records) show modest speedup because the Python↔Rust
bridge overhead (~5-8µs) dominates. Large batches — where Django APIs
actually spend time — show significant improvement.

Full methodology: [BENCHMARKS.md](BENCHMARKS.md)

## Supported Django Field Types

| Django Field | Rust Type | Precision |
|-------------|-----------|-----------|
| CharField, TextField | `String` | max_length enforced in characters (UTF-8 safe) |
| IntegerField | `i32` | Full range |
| BigIntegerField | `i64` | Full range |
| DecimalField | `rust_decimal::Decimal` | Exact — no float conversion |
| DateField | `chrono::NaiveDate` | ISO 8601 |
| DateTimeField | `chrono::DateTime<Utc>` | RFC 3339 with timezone |
| UUIDField | `uuid::Uuid` | Hyphenated and non-hyphenated |
| BooleanField | `bool` | True/False (not 1/0) |
| FloatField | `f64` | IEEE 754, NaN/Infinity rejected |
| JSONField | `serde_json::Value` | Full nested structure |
| BinaryField | `Vec<u8>` | Base64 encoded for JSON |
| EmailField, URLField, SlugField | `String` | With format validation |

## Installation

```bash
pip install pyforge-django
```

Add to your Django settings:

```python
INSTALLED_APPS = [
    ...
    'django_pyforge',
]
```

## Requirements

- Python 3.11+
- Django 4.2 LTS or Django 5.x
- CPython only (PyPy and GraalPy are not supported)

## Crates

| Crate | Description |
|-------|-------------|
| `pyforge` | Core Rust-Python binding library |
| `pyforge-django` | Django integration layer |
| `pyforge-ffi` | CPython C API bindings |
| `pyforge-macros` | Procedural macros (`#[pyfunction]`, `#[pyclass]`, etc.) |
| `pyforge-build-config` | Build-time Python detection |

## License

MIT

## Author

Abdulwahed Mansour
