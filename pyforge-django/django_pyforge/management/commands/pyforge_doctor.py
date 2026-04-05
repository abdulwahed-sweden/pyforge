# Author: Abdulwahed Mansour
"""
Management command: python manage.py pyforge_doctor

Scans all ModelSerializer subclasses in the project and reports
which ones would benefit from RustSerializerMixin and which wouldn't.
"""

from django.core.management.base import BaseCommand


class Command(BaseCommand):
    help = "Audit serializers for PyForge compatibility"

    def handle(self, *args, **options):
        try:
            from rest_framework.serializers import ModelSerializer
        except ImportError:
            self.stderr.write("djangorestframework is not installed.")
            return

        from django_pyforge import ModelSchema

        self.stdout.write("")
        self.stdout.write("PyForge Serializer Audit")
        self.stdout.write("=" * 60)
        self.stdout.write("")

        # Discover all ModelSerializer subclasses
        serializers = self._discover_serializers(ModelSerializer)

        if not serializers:
            self.stdout.write("No ModelSerializer subclasses found.")
            return

        recommended = []
        skipped = []

        for ser_class in serializers:
            result = self._analyze_serializer(ser_class, ModelSchema)
            name = f"{ser_class.__module__}.{ser_class.__qualname__}"

            if result["recommendation"] == "RECOMMENDED":
                tag = self.style.SUCCESS("RECOMMENDED")
                recommended.append(name)
            elif result["recommendation"] == "MARGINAL":
                tag = self.style.WARNING("MARGINAL")
            else:
                tag = self.style.ERROR("SKIP")
                skipped.append(name)

            rust_count = result["rust_fields"]
            total = result["total_fields"]
            ratio = result["ratio"]

            self.stdout.write(
                f"  {ser_class.__qualname__:<35} "
                f"{rust_count:>2}/{total:<2} Rust fields ({ratio:>3.0%}) "
                f"-> {tag}"
            )

            if result["python_reasons"]:
                for reason in result["python_reasons"][:3]:
                    self.stdout.write(f"    - {reason}")

        self.stdout.write("")
        self.stdout.write("-" * 60)
        self.stdout.write(f"  Recommended: {len(recommended)}")
        self.stdout.write(f"  Skipped:     {len(skipped)}")
        self.stdout.write("")

    def _discover_serializers(self, base_class):
        """Find all concrete ModelSerializer subclasses."""
        found = []
        seen = set()
        queue = list(base_class.__subclasses__())

        while queue:
            cls = queue.pop(0)
            if id(cls) in seen:
                continue
            seen.add(id(cls))

            # Skip abstract or framework-internal serializers
            if cls.__module__.startswith("rest_framework"):
                queue.extend(cls.__subclasses__())
                continue

            if hasattr(cls, "Meta") and hasattr(cls.Meta, "model"):
                found.append(cls)

            queue.extend(cls.__subclasses__())

        return found

    def _analyze_serializer(self, ser_class, model_schema_cls):
        """Analyze a serializer class for PyForge compatibility."""
        try:
            model = ser_class.Meta.model
            schema = model_schema_cls(model)
            rust_field_names = set(schema.field_names_list)
        except Exception:
            return {
                "rust_fields": 0,
                "total_fields": 0,
                "ratio": 0,
                "recommendation": "SKIP",
                "python_reasons": ["Could not compile ModelSchema"],
            }

        try:
            ser = ser_class()
            fields = ser.fields
        except Exception:
            return {
                "rust_fields": 0,
                "total_fields": 0,
                "ratio": 0,
                "recommendation": "SKIP",
                "python_reasons": ["Could not instantiate serializer"],
            }

        rust_count = 0
        python_reasons = []

        for field_name, field_obj in fields.items():
            field_class_name = type(field_obj).__name__

            is_computed = field_class_name in (
                "SerializerMethodField", "HiddenField", "ReadOnlyField",
            )
            is_nested = hasattr(field_obj, "Meta") and hasattr(field_obj, "fields")
            source = getattr(field_obj, "source", field_name)
            has_custom_source = source != field_name and source != "*"
            in_schema = field_name in rust_field_names

            if in_schema and not is_computed and not is_nested and not has_custom_source:
                rust_count += 1
            else:
                reason = field_name
                if is_computed:
                    reason += f" ({field_class_name})"
                elif is_nested:
                    reason += " (nested serializer)"
                elif has_custom_source:
                    reason += f" (custom source='{source}')"
                elif not in_schema:
                    reason += " (not a model field)"
                python_reasons.append(reason)

        total = len(fields)
        ratio = rust_count / total if total > 0 else 0

        if ratio >= 0.80 and total >= 5:
            rec = "RECOMMENDED"
        elif ratio >= 0.60:
            rec = "MARGINAL"
        else:
            rec = "SKIP"

        return {
            "rust_fields": rust_count,
            "total_fields": total,
            "ratio": ratio,
            "recommendation": rec,
            "python_reasons": python_reasons,
        }
