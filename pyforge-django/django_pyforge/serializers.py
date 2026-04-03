# Author: Abdulwahed Mansour
"""
Drop-in mixin for Django REST Framework serializers.

Accelerates the serialization phase using Rust-native field processing.
Works with any existing DRF ModelSerializer — just add the mixin.

Usage:
    from django_pyforge.serializers import RustSerializerMixin
    from rest_framework import serializers

    class UserSerializer(RustSerializerMixin, serializers.ModelSerializer):
        class Meta:
            model = User
            fields = '__all__'
"""

from django_pyforge import extract_model_fields, serialize_fields


class RustSerializerMixin:
    """
    Mixin that accelerates DRF serializer .to_representation() using Rust.

    Add this mixin BEFORE the base serializer class in the MRO:

        class MySerializer(RustSerializerMixin, serializers.ModelSerializer):
            ...

    The mixin overrides to_representation() to extract field values and
    serialize them through the Rust backend. Falls back to the standard
    DRF path if the model cannot be introspected.
    """

    _pyforge_field_cache = None

    def _get_pyforge_field_descriptors(self):
        """Extract and cache field descriptors from the model class."""
        if self._pyforge_field_cache is not None:
            return self._pyforge_field_cache

        model_class = self.Meta.model
        try:
            field_descriptors = extract_model_fields(model_class)
            self.__class__._pyforge_field_cache = field_descriptors
            return field_descriptors
        except (ValueError, AttributeError):
            return None

    def to_representation(self, instance):
        """
        Serialize a model instance using Rust acceleration.

        Extracts field values from the instance, passes them through the
        Rust serializer, and returns a dict. Falls back to the standard
        DRF serialization if Rust processing fails.
        """
        field_descriptors = self._get_pyforge_field_descriptors()

        if field_descriptors is None:
            return super().to_representation(instance)

        try:
            # Build a values dict from the model instance
            serializer_fields = self.fields
            values = {}
            for field_name in serializer_fields:
                try:
                    values[field_name] = getattr(instance, field_name, None)
                except Exception:
                    values[field_name] = None

            # Filter descriptors to only include fields present in the serializer
            active_descriptors = [
                desc for desc in field_descriptors
                if desc["name"] in serializer_fields
            ]

            if not active_descriptors:
                return super().to_representation(instance)

            serialized = serialize_fields(active_descriptors, values)
            return dict(serialized)

        except Exception:
            # Graceful fallback — never break existing behavior
            return super().to_representation(instance)
