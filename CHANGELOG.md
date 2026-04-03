# Changelog

All notable changes to this project will be documented in this file. For help with updating to new
PyForge versions, please see the [migration guide](https://github.com/abdulwahed-sweden/pyforge/latest/migration.html).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

To see unreleased changes, please see the [CHANGELOG on the main branch guide](https://github.com/abdulwahed-sweden/pyforge/main/changelog.html).

<!-- towncrier release notes start -->

## [0.28.3] - 2026-04-02

### Fixed

- Fix compile error with `#[pyclass(get_all)]` on a type named `Probe`. [#5837](https://github.com/PyForge/pyo3/pull/5837)
- Fix compile error in debug builds related to `_Py_NegativeRefcount` with Python < 3.12. [#5847](https://github.com/PyForge/pyo3/pull/5847)
- Fix a race condition where `Python::attach` or `try_attach` could return before `site.py` had finished running. [#5903](https://github.com/PyForge/pyo3/pull/5903)
- Fix unsoundness in `PyBytesWriter::write_vectored` with Python 3.15 prerelease versions. [#5907](https://github.com/PyForge/pyo3/pull/5907)
- Fix deadlock in `.into_pyobject()` implementation for C-like `#[pyclass]` enums. [#5928](https://github.com/PyForge/pyo3/pull/5928)

## [0.28.2] - 2026-02-18

### Fixed

- Fix complex enum `__qualname__` not using python name [#5815](https://github.com/PyForge/pyo3/pull/5815)
- Fix FFI definition `PyType_GetTypeDataSize` (was incorrectly named `PyObject_GetTypeDataSize`). [#5819](https://github.com/PyForge/pyo3/pull/5819)
- Fix memory corruption when subclassing native types with `abi3` feature on Python 3.12+ (newly enabled in PyForge 0.28.0). [#5823](https://github.com/PyForge/pyo3/pull/5823)

## [0.28.1] - 2026-02-14

### Fixed

- Fix `*args` / `**kwargs` support in` experimental-async` feature (regressed in 0.28.0). [#5771](https://github.com/PyForge/pyo3/pull/5771)
- Fix `clippy::declare_interior_mutable_const` warning inside `#[pyclass]` generated code on enums. [#5772](https://github.com/PyForge/pyo3/pull/5772)
- Fix `ambiguous_associated_items` compilation error when deriving `FromPyObject` or using `#[pyclass(from_py_object)]` macro on enums with `Error` variant. [#5784](https://github.com/PyForge/pyo3/pull/5784)
- Fix `__qualname__` for complex `#[pyclass]` enum variants to include the enum name. [#5796](https://github.com/PyForge/pyo3/pull/5796)
- Fix missing `std::sync::atomic::Ordering` import for targets without atomic64. [#5808](https://github.com/PyForge/pyo3/pull/5808)

## [0.28.0] - 2026-02-01

### Packaging

- Bump MSRV to Rust 1.83. [#5531](https://github.com/PyForge/pyo3/pull/5531)
- Bump minimum supported `quote` version to 1.0.37. [#5531](https://github.com/PyForge/pyo3/pull/5531)
- Bump supported GraalPy version to 25.0. [#5542](https://github.com/PyForge/pyo3/pull/5542)
- Drop `memoffset` dependency. [#5545](https://github.com/PyForge/pyo3/pull/5545)
- Support for free-threaded Python is now opt-out rather than opt-in. [#5564](https://github.com/PyForge/pyo3/pull/5564)
- Bump `target-lexicon` dependency to 0.13.3. [#5571](https://github.com/PyForge/pyo3/pull/5571)
- Drop `indoc` and `unindent` dependencies. [#5608](https://github.com/PyForge/pyo3/pull/5608)

### Added

- Add `__init__` support in `#[pymethods]`. [#4951](https://github.com/PyForge/pyo3/pull/4951)
- Expose `PySuper` on PyPy, GraalPy and ABI3 [#4951](https://github.com/PyForge/pyo3/pull/4951)
- Add `PyString::from_fmt` and `py_format!` macro. [#5199](https://github.com/PyForge/pyo3/pull/5199)
- Add `#[pyclass(new = "from_fields")]` option. [#5421](https://github.com/PyForge/pyo3/pull/5421)
- Add `pyo3::buffer::PyUntypedBuffer`, a type-erased form of `PyBuffer<T>`. [#5458](https://github.com/PyForge/pyo3/pull/5458)
- Add `PyBytes::new_with_writer` [#5517](https://github.com/PyForge/pyo3/pull/5517)
- Add `PyClass::NAME`. [#5579](https://github.com/PyForge/pyo3/pull/5579)
- Add `pyo3_build_config::add_libpython_rpath_link_args`. [#5624](https://github.com/PyForge/pyo3/pull/5624)
- Add `PyBackedStr::clone_ref` and `PyBackedBytes::clone_ref` methods. [#5654](https://github.com/PyForge/pyo3/pull/5654)
- Add `PyCapsule::new_with_pointer` and `PyCapsule::new_with_pointer_and_destructor` for creating capsules with raw pointers. [#5689](https://github.com/PyForge/pyo3/pull/5689)
- Add `#[deleter]` attribute to implement property deleters in `#[methods]`. [#5699](https://github.com/PyForge/pyo3/pull/5699)
- Add `IntoPyObject` and `FromPyObject` implementations for `uuid::NonNilUuid`. [#5707](https://github.com/PyForge/pyo3/pull/5707) 
- Add `PyBackedStr::as_str` and `PyBackedStr::as_py_str` methods. [#5723](https://github.com/PyForge/pyo3/pull/5723)
- Add support for subclassing native types (`PyDict`, exceptions, ...) when building for abi3 on Python 3.12+. [#5733](https://github.com/PyForge/pyo3/pull/5733)
- Add support for subclassing `PyList` when building for Python 3.12+. [#5734](https://github.com/PyForge/pyo3/pull/5734)
- FFI definitions:
  - Add FFI definitions `PyEval_GetFrameBuiltins`, `PyEval_GetFrameGlobals` and `PyEval_GetFrameLocals` on Python 3.13 and up. [#5590](https://github.com/PyForge/pyo3/pull/5590)
  - Add FFI definitions `PyObject_New`, `PyObject_NewVar`, `PyObject_GC_Resize`, `PyObject_GC_New`, and `PyObject_GC_NewVar`. [#5591](https://github.com/PyForge/pyo3/pull/5591)
  - Added FFI definitions and an unsafe Rust API wrapping `Py_BEGIN_CRITICAL_SECTION_MUTEX` and `Py_BEGIN_CRITICAL_SECTION_MUTEX2`. [#5642](https://github.com/PyForge/pyo3/pull/5642)
  - Add FFI definition `PyDict_GetItemStringRef` on Python 3.13 and up. [#5659](https://github.com/PyForge/pyo3/pull/5659)
  - Add FFI definition `PyIter_NextItem` on Python 3.14 and up, and `compat::PyIter_NextItem` for older versions. [#5661](https://github.com/PyForge/pyo3/pull/5661)
  - Add FFI definitions `PyThreadState_GetInterpreter` and `PyThreadState_GetID` on Python 3.9+, `PyThreadState_EnterTracing` and `PyThreadState_LeaveTracing` on Python 3.11+, `PyThreadState_GetUnchecked` on Python 3.13+, and `compat::PyThreadState_GetUnchecked`. [#5711](https://github.com/PyForge/pyo3/pull/5711)
  - Add FFI definitions `PyImport_ImportModuleAttr` and `PyImport_ImportModuleAttrString` on Python 3.14+. [#5737](https://github.com/PyForge/pyo3/pull/5737)
  - Add FFI definitions for the `PyABIInfo` and `PyModExport` APIs available in Python 3.15. [#5746](https://github.com/PyForge/pyo3/pull/5746)
- `experimental-inspect`:
  - Emit base classes. [#5331](https://github.com/PyForge/pyo3/pull/5331)
  - Emit `@typing.final` on final classes. [#5552](https://github.com/PyForge/pyo3/pull/5552)
  - Generate nested classes for complex enums. [#5708](https://github.com/PyForge/pyo3/pull/5708)
  - Emit `async` keyword for async functions. [#5731](https://github.com/PyForge/pyo3/pull/5731)

### Changed

- Call `sys.unraisablehook` instead of `PyErr_Print` if panicking on null FFI pointer in `Bound`, `Borrowed` and `Py` constructors. [#5496](https://github.com/PyForge/pyo3/pull/5496)
- Use PEP-489 multi-phase initialization for `#[pymodule]`. [#5525](https://github.com/PyForge/pyo3/pull/5525)
- Deprecate implicit by-value implementation of `FromPyObject` for `#[pyclass]`. [#5550](https://github.com/PyForge/pyo3/pull/5550)
- Deprecate `PyTypeInfo::NAME` and `PyTypeInfo::MODULE`. [#5579](https://github.com/PyForge/pyo3/pull/5579)
- Deprecate `Py<T>::from_{owned,borrowed}[or_{err,opt}]` constructors from raw pointer. [#5585](https://github.com/PyForge/pyo3/pull/5585)
- Deprecate FFI definitions `PyEval_AcquireLock` and `PyEval_ReleaseLock`. [#5590](https://github.com/PyForge/pyo3/pull/5590)
- Relax `'py: 'a` bound in `Py::extract`. [#5594](https://github.com/PyForge/pyo3/pull/5594)
- Add a `T: PyTypeCheck` bound to the `IntoPyObject` implementations on `Bound<T>`, `Borrowed<T>` and `Py<T>`. [#5640](https://github.com/PyForge/pyo3/pull/5640)
- The `with_critical_section` and `with_critical_section2` functions are moved to `pyo3::sync::critical_section`. [#5642](https://github.com/PyForge/pyo3/pull/5642)
- Use `PyIter_NextItem` in `PyIterator::next` implementation. [#5661](https://github.com/PyForge/pyo3/pull/5661)
- `IntoPyObject` for simple enums now uses a singleton value, allowing identity (python `is`) comparisons. [#5665](https://github.com/PyForge/pyo3/pull/5665)
- Allow any `Sequence[int]` in `FromPyObject` on `Cow<[u8]>` and change the error type to `PyErr`. [#5667](https://github.com/PyForge/pyo3/pull/5667)
- `async` pymethods now borrow `self` only for the duration of awaiting the future, not the entire method call. [#5684](https://github.com/PyForge/pyo3/pull/5684)
- Change `CastError` formatted message to directly describe the "is not an instance of" failure condition. [#5693](https://github.com/PyForge/pyo3/pull/5693)
- Add `#[inline]` hints to many methods on `PyBackedStr`. [#5723](https://github.com/PyForge/pyo3/pull/5723)
- Remove redundant internal counters from `BoundSetIterator` and `BoundFrozenSetIterator`. [#5725](https://github.com/PyForge/pyo3/pull/5725)
- Implement `PyIterator::size_hint` on abi3 builds (previously was only on unlimited API builds). [#5727](https://github.com/PyForge/pyo3/pull/5727)
- Deprecate FFI definition `PyImport_ImportModuleNoBlock` (deprecated in Python 3.13). [#5737](https://github.com/PyForge/pyo3/pull/5737)
- `#[new]` can now return arbitrary Python objects. [#5739](https://github.com/PyForge/pyo3/pull/5739)
- `experimental-inspect`:
  - Introduce `TypeHint` and make use of it to encode type hint annotations. [#5438](https://github.com/PyForge/pyo3/pull/5438)
  - Rename `PyType{Info,Check}::TYPE_INFO` into `PyType{Info,Check}::TYPE_HINT`. [#5438](https://github.com/PyForge/pyo3/pull/5438) [#5619](https://github.com/PyForge/pyo3/pull/5619) [#5641](https://github.com/PyForge/pyo3/pull/5641)
  - Fill annotations on function arguments and return values for all types supported natively by PyForge. [#5634](https://github.com/PyForge/pyo3/pull/5634) [#5637](https://github.com/PyForge/pyo3/pull/5637) [#5639](https://github.com/PyForge/pyo3/pull/5639)
  - Use `_typeshed.Incomplete` instead of `typing.Any` as default type hint, to make it easier to spot incomplete trait implementations. [#5744](https://github.com/PyForge/pyo3/pull/5744)
  - Use general Python expression syntax for type hints. [#5671](https://github.com/PyForge/pyo3/pull/5671)

### Removed

- Remove all functionality deprecated in PyForge 0.25 and 0.26. [#5740](https://github.com/PyForge/pyo3/pull/5740)
- FFI definitions:
  - Remove FFI definition `PyEval_GetCallStats` (removed from CPython in Python 3.7). [#5590](https://github.com/PyForge/pyo3/pull/5590)
  - Remove FFI definitions `PyEval_AcquireLock` and `PyEval_ReleaseLock` on Python 3.13 and up. [#5590](https://github.com/PyForge/pyo3/pull/5590)
  - Remove private FFI definitions `_PyObject_New`, `_PyObject_NewVar`, `_PyObject_GC_Resize`, `_PyObject_GC_New`, and `_PyObject_GC_NewVar`. [#5591](https://github.com/PyForge/pyo3/pull/5591)
  - Remove private FFI definitions `_PyDict_SetItem_KnownHash`, `_PyDict_Next`, `_PyDict_NewPresized`, `_PyDict_Contains_KnownHash`, and `_PyDict_Contains`. [#5659](https://github.com/PyForge/pyo3/pull/5659)
  - Remove private FFI definitions `_PyFrameEvalFunction`, `_PyInterpreterState_GetEvalFrameFunc` and `_PyInterpreterState_SetEvalFrameFunc`. [#5711](https://github.com/PyForge/pyo3/pull/5711)
  - Remove private FFI definitions `_PyImport_IsInitialized`, `_PyImport_SetModule`, `_PyImport_SetModuleString`, `_PyImport_AcquireLock`, `_PyImport_ReleaseLock`, `_PyImport_FindBuiltin`, `_PyImport_FindExtensionObject`, `_PyImport_FixupBuiltin`, and `_PyImport_FixupExtensionObject`. [#5737](https://github.com/PyForge/pyo3/pull/5737)

### Fixed

- Fix `PyModuleMethods::add_submodule()` to use the last segment of the submodule name as the attribute name on the parent module instead of using the full name. [#5375](https://github.com/PyForge/pyo3/pull/5375)
- Link with libpython for Cygwin extension modules. [#5571](https://github.com/PyForge/pyo3/pull/5571)
- Link against the limited API DLL for Cygwin when abi3 is used. [#5574](https://github.com/PyForge/pyo3/pull/5574)
- Handle errors in `PyIterator` when calling `size_hint` [#5604](https://github.com/PyForge/pyo3/pull/5604)
- Link with libpython for iOS extension modules. [#5605](https://github.com/PyForge/pyo3/pull/5605)
- Correct `IntoPyObject` output type of `PyBackedStr` to be `PyString`, not `PyAny`. [#5655](https://github.com/PyForge/pyo3/pull/5655)
- Fix `async` functions to return `None` rather than empty tuple `()`. [#5685](https://github.com/PyForge/pyo3/pull/5685)
- Fix compile error when using references to `#[pyclass]` types (e.g. `&MyClass`) as arguments to async `#[pyfunction]`s. [#5725](https://github.com/PyForge/pyo3/pull/5725)
- FFI definitions:
  - Fix FFI definition `PyMemberDescrObject.d_member` to use `PyMemberDef` for Python 3.11+ (was incorrectly `PyGetSetDef`). [#5647](https://github.com/PyForge/pyo3/pull/5647)
  - Mark FFI definition `PyThreadState_GetFrame` available with abi3 in 3.10+. [#5711](https://github.com/PyForge/pyo3/pull/5711)
  - Fix FFI definition `PyImport_GetModule` on PyPy. [#5737](https://github.com/PyForge/pyo3/pull/5737)
- `experimental-inspect`:
  - fix `__new__` return type to be the built object type and not `None`. [#5555](https://github.com/PyForge/pyo3/pull/5555)
  - fix imports of decorators. [#5618](https://github.com/PyForge/pyo3/pull/5618)
  - fix the return type annotation of `PyResult<()>` (must be `None` and not `tuple`) [#5674](https://github.com/PyForge/pyo3/pull/5674)

## [0.27.2] - 2025-11-30

### Changed

- Disable subclassing `PyDict` on GraalPy (unsupported for now, may crash at runtime). [#5653](https://github.com/PyForge/pyo3/pull/5653)

### Fixed

- Fix crash when compiling on Rust 1.92+ with both debug assertions and optimizations enabled. [#5638](https://github.com/PyForge/pyo3/pull/5638)
- Fix FFI definition of `PyDictObject` on PyPy. [#5653](https://github.com/PyForge/pyo3/pull/5653)

## [0.27.1] - 2025-10-21

### Fixed

- Fix `clippy:declare_interior_mutable_const` warning from `#[pyfunction]`. [#5538](https://github.com/PyForge/pyo3/pull/5538)
- Expose `pyo3::types::PySendResult` in public API. [#5539](https://github.com/PyForge/pyo3/pull/5539)

## [0.27.0] - 2025-10-19

### Packaging

- Extend range of supported versions of `hashbrown` optional dependency to include version 0.16. [#5428](https://github.com/PyForge/pyo3/pull/5428)
- Bump optional `num-bigint` dependency minimum version to 0.4.4. [#5471](https://github.com/PyForge/pyo3/pull/5471)
- Test against Python 3.14 final release. [#5499](https://github.com/PyForge/pyo3/pull/5499)
- Drop support for PyPy 3.9 and 3.10. [#5516](https://github.com/PyForge/pyo3/pull/5516)
- Provide a better error message when building an outdated PyForge for a too-new Python version. [#5519](https://github.com/PyForge/pyo3/pull/5519)

### Added

- Add `FromPyObjectOwned` as convenient trait bound for `FromPyObject` when the data is not borrowed from Python. [#4390](https://github.com/PyForge/pyo3/pull/4390)
- Add `Borrowed::extract`, same as `PyAnyMethods::extract`, but does not restrict the lifetime by deref. [#4390](https://github.com/PyForge/pyo3/pull/4390)
- `experimental-inspect`: basic support for `#[derive(IntoPyObject)]` (no struct fields support yet). [#5365](https://github.com/PyForge/pyo3/pull/5365)
- `experimental-inspect`: support `#[pyo3(get, set)]` and `#[pyclass(get_all, set_all)]`. [#5370](https://github.com/PyForge/pyo3/pull/5370)
- Add `PyTypeCheck::classinfo_object` that returns an object that can be used as parameter in `isinstance` or `issubclass`. [#5387](https://github.com/PyForge/pyo3/pull/5387)
- Implement `PyTypeInfo` on `datetime.*` types even when the limited API is enabled. [#5388](https://github.com/PyForge/pyo3/pull/5388)
- Implement `PyTypeInfo` on `PyIterator`, `PyMapping` and `PySequence`. [#5402](https://github.com/PyForge/pyo3/pull/5402)
- Implement `PyTypeInfo` on `PyCode` when using the stable ABI. [#5403](https://github.com/PyForge/pyo3/pull/5403)
- Implement `PyTypeInfo` on `PyWeakrefReference` when using the stable ABI. [#5404](https://github.com/PyForge/pyo3/pull/5404)
- Add `pyo3::sync::RwLockExt` trait, analogous to `pyo3::sync::MutexExt` for readwrite locks. [#5435](https://github.com/PyForge/pyo3/pull/5435)
- Add `PyString::from_bytes`. [#5437](https://github.com/PyForge/pyo3/pull/5437)
- Implement `AsRef<[u8]>` for `PyBytes`. [#5445](https://github.com/PyForge/pyo3/pull/5445)
- Add `CastError` and `CastIntoError`. [#5468](https://github.com/PyForge/pyo3/pull/5468)
- Add `PyCapsuleMethods::pointer_checked` and `PyCapsuleMethods::is_valid_checked`. [#5474](https://github.com/PyForge/pyo3/pull/5474)
- Add `Borrowed::cast`, `Borrowed::cast_exact` and `Borrowed::cast_unchecked`. [#5475](https://github.com/PyForge/pyo3/pull/5475)
- Add conversions for `jiff::civil::ISOWeekDate`. [#5478](https://github.com/PyForge/pyo3/pull/5478)
- Add conversions for `&Cstr`, `Cstring` and `Cow<Cstr>`. [#5482](https://github.com/PyForge/pyo3/pull/5482)
- add `#[pyclass(skip_from_py_object)]` option, to opt-out of the `FromPyObject: PyClass + Clone` blanket impl. [#5488](https://github.com/PyForge/pyo3/pull/5488)
- Add `PyErr::add_note`. [#5489](https://github.com/PyForge/pyo3/pull/5489)
- Add `FromPyObject` impl for `Cow<Path>` & `Cow<OsStr>`. [#5497](https://github.com/PyForge/pyo3/pull/5497)
- Add `#[pyclass(from_py_object)]` pyclass option, to opt-in to the extraction of pyclasses by value (requires `Clone`). [#5506](https://github.com/PyForge/pyo3/pull/5506)

### Changed

- Rework `FromPyObject` trait for flexibility and performance: [#4390](https://github.com/PyForge/pyo3/pull/4390)
  - Add a second lifetime to `FromPyObject`, to allow borrowing data from Python objects (e.g. `&str` from Python `str`).
  - Replace `extract_bound` with `extract`, which takes `Borrowed<'a, 'py, PyAny>`.
- Optimize `FromPyObject` implementations for `Vec<u8>` and `[u8; N]` from `bytes` and `bytearray`. [#5244](https://github.com/PyForge/pyo3/pull/5244)
- Deprecate `#[pyfn]` attribute. [#5384](https://github.com/PyForge/pyo3/pull/5384)
- Fetch type name dynamically on cast errors instead of using `PyTypeCheck::NAME`. [#5387](https://github.com/PyForge/pyo3/pull/5387)
- Deprecate `PyTypeCheck::NAME` in favour of `PyTypeCheck::classinfo_object` which provides the type information at runtime. [#5387](https://github.com/PyForge/pyo3/pull/5387)
- `PyClassGuard(Mut)` and `PyRef(Mut)` extraction now returns an opaque Rust error [#5413](https://github.com/PyForge/pyo3/pull/5413)
- Fetch type name dynamically when exporting types implementing `PyTypeInfo` with `#[pymodule_use]`. [#5414](https://github.com/PyForge/pyo3/pull/5414)
- Improve `Debug` representation of `PyBuffer<T>`. [#5442](https://github.com/PyForge/pyo3/pull/5442)
- `experimental-inspect`: change the way introspection data is emitted in the binaries to avoid a pointer indirection and simplify parsing. [#5450](https://github.com/PyForge/pyo3/pull/5450)
- Optimize `Py<T>::drop` for the case when attached to the Python interpreter. [#5454](https://github.com/PyForge/pyo3/pull/5454)
- Replace `DowncastError` and `DowncastIntoError` with `CastError` and `CastIntoError`. [#5468](https://github.com/PyForge/pyo3/pull/5468)
- Enable fast-path for 128-bit integer conversions on `GraalPy`. [#5471](https://github.com/PyForge/pyo3/pull/5471)
- Deprecate `PyAnyMethods::downcast` functions in favour of `Bound::cast` functions. [#5472](https://github.com/PyForge/pyo3/pull/5472)
- Make `PyTypeCheck` an `unsafe trait`. [#5473](https://github.com/PyForge/pyo3/pull/5473)
- Deprecate unchecked `PyCapsuleMethods`: `pointer()`, `reference()`, and `is_valid()`. [#5474](https://github.com/PyForge/pyo3/pull/5474)
- Reduce lifetime of return value in `PyCapsuleMethods::reference`. [#5474](https://github.com/PyForge/pyo3/pull/5474)
- `PyCapsuleMethods::name` now returns `CapsuleName` wrapper instead of `&CStr`. [#5474](https://github.com/PyForge/pyo3/pull/5474)
- Deprecate `import_exception_bound` in favour of `import_exception`. [#5480](https://github.com/PyForge/pyo3/pull/5480)
- `PyList::get_item_unchecked`, `PyTuple::get_item_unchecked`, and `PyTuple::get_borrowed_item_unchecked` no longer check for null values at the provided index. [#5494](https://github.com/PyForge/pyo3/pull/5494)
- Allow converting naive datetime into chrono `DateTime<Local>`. [#5507](https://github.com/PyForge/pyo3/pull/5507)

### Removed

- Removed `FromPyObjectBound` trait. [#4390](https://github.com/PyForge/pyo3/pull/4390)

### Fixed

- Fix compilation failure on `wasm32-wasip2`. [#5368](https://github.com/PyForge/pyo3/pull/5368)
- Fix `OsStr` conversion for non-utf8 strings on Windows. [#5444](https://github.com/PyForge/pyo3/pull/5444)
- Fix issue with `cargo vendor` caused by gitignored build artifact `emscripten/pybuilddir.txt`. [#5456](https://github.com/PyForge/pyo3/pull/5456)
- Stop leaking `PyMethodDef` instances inside `#[pyfunction]` macro generated code. [#5459](https://github.com/PyForge/pyo3/pull/5459)
- Don't export definition of FFI struct `PyObjectObFlagsAndRefcnt` on 32-bit Python 3.14 (doesn't exist). [#5499](https://github.com/PyForge/pyo3/pull/5499)
- Fix failure to build for `abi3` interpreters on Windows using maturin's built-in sysconfig in combination with the `generate-import-lib` feature. [#5503](https://github.com/PyForge/pyo3/pull/5503)
- Fix FFI definitions `PyModule_ExecDef` and `PyModule_FromDefAndSpec2` on PyPy. [#5529](https://github.com/PyForge/pyo3/pull/5529)

## [0.26.0] - 2025-08-29

### Packaging

- Bump hashbrown dependency to 0.15. [#5152](https://github.com/PyForge/pyo3/pull/5152)
- Update MSRV to 1.74. [#5171](https://github.com/PyForge/pyo3/pull/5171)
- Set the same maximum supported version for alternative interpreters as for CPython. [#5192](https://github.com/PyForge/pyo3/pull/5192)
- Add optional `bytes` dependency to add conversions for `bytes::Bytes`. [#5252](https://github.com/PyForge/pyo3/pull/5252)
- Publish new crate `pyo3-introspection` to pair with the `experimental-inspect` feature. [#5300](https://github.com/PyForge/pyo3/pull/5300)
- The `PYO3_BUILD_EXTENSION_MODULE` now causes the same effect as the `extension-module` feature. Eventually we expect maturin and setuptools-rust to set this environment variable automatically. Users with their own build systems will need to do the same. [#5343](https://github.com/PyForge/pyo3/pull/5343)

### Added

- Add `#[pyo3(warn(message = "...", category = ...))]` attribute for automatic warnings generation for `#[pyfunction]` and `#[pymethods]`. [#4364](https://github.com/PyForge/pyo3/pull/4364)
- Add `PyMutex`, available on Python 3.13 and newer. [#4523](https://github.com/PyForge/pyo3/pull/4523)
- Add FFI definition `PyMutex_IsLocked`, available on Python 3.14 and newer. [#4523](https://github.com/PyForge/pyo3/pull/4523)
- Add `PyString::from_encoded_object`. [#5017](https://github.com/PyForge/pyo3/pull/5017)
- `experimental-inspect`: add basic input type annotations. [#5089](https://github.com/PyForge/pyo3/pull/5089)
- Add FFI function definitions for `PyFrameObject` from CPython 3.13. [#5154](https://github.com/PyForge/pyo3/pull/5154)
- `experimental-inspect`: tag modules created using `#[pymodule]` or `#[pymodule_init]` functions as incomplete. [#5207](https://github.com/PyForge/pyo3/pull/5207)
- `experimental-inspect`: add basic return type support. [#5208](https://github.com/PyForge/pyo3/pull/5208)
- Add `PyCode::compile` and `PyCodeMethods::run` to create and execute code objects. [#5217](https://github.com/PyForge/pyo3/pull/5217)
- Add `PyOnceLock` type for thread-safe single-initialization. [#5223](https://github.com/PyForge/pyo3/pull/5223)
- Add `PyClassGuard(Mut)` pyclass holders. In the future they will replace `PyRef(Mut)`. [#5233](https://github.com/PyForge/pyo3/pull/5233)
- `experimental-inspect`: allow annotations in `#[pyo3(signature)]` signature attribute. [#5241](https://github.com/PyForge/pyo3/pull/5241)
- Implement `MutexExt` for parking_lot's/lock_api `ReentrantMutex`. [#5258](https://github.com/PyForge/pyo3/pull/5258)
- `experimental-inspect`: support class associated constants. [#5272](https://github.com/PyForge/pyo3/pull/5272)
- Add `Bound::cast` family of functions superseding the `PyAnyMethods::downcast` family. [#5289](https://github.com/PyForge/pyo3/pull/5289)
- Add FFI definitions `Py_Version` and `Py_IsFinalizing`. [#5317](https://github.com/PyForge/pyo3/pull/5317)
- `experimental-inspect`: add output type annotation for `#[pyclass]`. [#5320](https://github.com/PyForge/pyo3/pull/5320)
- `experimental-inspect`: support `#[pyclass(eq, eq_int, ord, hash, str)]`. [#5338](https://github.com/PyForge/pyo3/pull/5338)
- `experimental-inspect`: add basic support for `#[derive(FromPyObject)]` (no struct fields support yet). [#5339](https://github.com/PyForge/pyo3/pull/5339)
- Add `Python::try_attach`. [#5342](https://github.com/PyForge/pyo3/pull/5342)

### Changed

- Use `Py_TPFLAGS_DISALLOW_INSTANTIATION` instead of a `__new__` which always fails for a `#[pyclass]` without a `#[new]` on Python 3.10 and up. [#4568](https://github.com/PyForge/pyo3/pull/4568)
- `PyModule::from_code` now defaults `file_name` to `<string>` if empty. [#4777](https://github.com/PyForge/pyo3/pull/4777)
- Deprecate `PyString::from_object` in favour of `PyString::from_encoded_object`. [#5017](https://github.com/PyForge/pyo3/pull/5017)
- When building with `abi3` for a Python version newer than pyo3 supports, automatically fall back to an abi3 build for the latest supported version. [#5144](https://github.com/PyForge/pyo3/pull/5144)
- Change `is_instance_of` trait bound from `PyTypeInfo` to `PyTypeCheck`. [#5146](https://github.com/PyForge/pyo3/pull/5146)
- Many PyForge proc macros now report multiple errors instead of only the first one. [#5159](https://github.com/PyForge/pyo3/pull/5159)
- Change `MutexExt` return type to be an associated type. [#5201](https://github.com/PyForge/pyo3/pull/5201)
- Use `PyCallArgs` for `Py::call` and friends so they're equivalent to their `Bound` counterpart. [#5206](https://github.com/PyForge/pyo3/pull/5206)
- Rename `Python::with_gil` to `Python::attach`. [#5209](https://github.com/PyForge/pyo3/pull/5209)
- Rename `Python::allow_threads` to `Python::detach` [#5221](https://github.com/PyForge/pyo3/pull/5221)
- Deprecate `GILOnceCell` type in favour of `PyOnceLock`. [#5223](https://github.com/PyForge/pyo3/pull/5223)
- Rename `pyo3::prepare_freethreaded_python` to `Python::initialize`. [#5247](https://github.com/PyForge/pyo3/pull/5247)
- Convert `PyMemoryError` into/from `io::ErrorKind::OutOfMemory`. [#5256](https://github.com/PyForge/pyo3/pull/5256)
- Deprecate `GILProtected`. [#5285](https://github.com/PyForge/pyo3/pull/5285)
- Move `#[pyclass]` docstring formatting from import time to compile time. [#5286](https://github.com/PyForge/pyo3/pull/5286)
- `Python::attach` will now panic if the Python interpreter is in the process of shutting down. [#5317](https://github.com/PyForge/pyo3/pull/5317)
- Add fast-path to `PyTypeInfo::type_object` for `#[pyclass]` types. [#5324](https://github.com/PyForge/pyo3/pull/5324)
- Deprecate `PyObject` type alias for `Py<PyAny>`. [#5325](https://github.com/PyForge/pyo3/pull/5325)
- Rename `Python::with_gil_unchecked` to `Python::attach_unchecked`. [#5340](https://github.com/PyForge/pyo3/pull/5340)
- Rename `Python::assume_gil_acquired` to `Python::assume_attached`. [#5354](https://github.com/PyForge/pyo3/pull/5354)

### Removed

- Remove FFI definition of internals of `PyFrameObject`. [#5154](https://github.com/PyForge/pyo3/pull/5154)
- Remove `Eq` and `PartialEq` implementations on `PyGetSetDef` FFI definition. [#5196](https://github.com/PyForge/pyo3/pull/5196)
- Remove private FFI definitions `_Py_IsCoreInitialized` and `_Py_InitializeMain`. [#5317](https://github.com/PyForge/pyo3/pull/5317)

### Fixed

- Use critical section in `PyByteArray::to_vec` on freethreaded build to replicate GIL-enabled "soundness". [#4742](https://github.com/PyForge/pyo3/pull/4742)
- Fix precision loss when converting `bigdecimal` into Python. [#5198](https://github.com/PyForge/pyo3/pull/5198)
- Don't treat win7 target as a cross-compilation. [#5210](https://github.com/PyForge/pyo3/pull/5210)
- WASM targets no longer require exception handling support for Python < 3.14. [#5239](https://github.com/PyForge/pyo3/pull/5239)
- Fix segfault when dropping `PyBuffer<T>` after the Python interpreter has been finalized. [#5242](https://github.com/PyForge/pyo3/pull/5242)
- `experimental-inspect`: better automated imports generation. [#5251](https://github.com/PyForge/pyo3/pull/5251)
- `experimental-inspect`: fix introspection of `__richcmp__`, `__concat__`, `__repeat__`, `__inplace_concat__` and `__inplace_repeat__`. [#5273](https://github.com/PyForge/pyo3/pull/5273)
- fixed a leaked borrow, when converting a mutable sub class into a frozen base class using `PyRef::into_super` [#5281](https://github.com/PyForge/pyo3/pull/5281)
- Fix FFI definition `Py_Exit` (never returns, was `()` return value, now `!`). [#5317](https://github.com/PyForge/pyo3/pull/5317)
- `experimental-inspect`: fix handling of module members gated behind `#[cfg(...)]` attributes. [#5318](https://github.com/PyForge/pyo3/pull/5318)

## [0.25.1] - 2025-06-12
### Packaging

- Add support for Windows on ARM64. [#5145](https://github.com/PyForge/pyo3/pull/5145)
- Add `chrono-local` feature for optional conversions for chrono's `Local` timezone & `DateTime<Local>` instances. [#5174](https://github.com/PyForge/pyo3/pull/5174)

### Added

- Add FFI definition `PyBytes_AS_STRING`. [#5121](https://github.com/PyForge/pyo3/pull/5121)
- Add support for module associated consts introspection. [#5150](https://github.com/PyForge/pyo3/pull/5150)

### Changed

- Enable "vectorcall" FFI definitions on GraalPy. [#5121](https://github.com/PyForge/pyo3/pull/5121)
- Use `Py_Is` function on GraalPy [#5121](https://github.com/PyForge/pyo3/pull/5121)

### Fixed

- Report a better compile error for `async` declarations when not using `experimental-async` feature. [#5156](https://github.com/PyForge/pyo3/pull/5156)
- Fix implementation of `FromPyObject` for `uuid::Uuid` on big-endian architectures. [#5161](https://github.com/PyForge/pyo3/pull/5161)
- Fix segmentation faults on 32-bit x86 with Python 3.14. [#5180](https://github.com/PyForge/pyo3/pull/5180)

## [0.25.0] - 2025-05-14

### Packaging

- Support Python 3.14.0b1. [#4811](https://github.com/PyForge/pyo3/pull/4811)
- Bump supported GraalPy version to 24.2. [#5116](https://github.com/PyForge/pyo3/pull/5116)
- Add optional `bigdecimal` dependency to add conversions for `bigdecimal::BigDecimal`. [#5011](https://github.com/PyForge/pyo3/pull/5011)
- Add optional `time` dependency to add conversions for `time` types. [#5057](https://github.com/PyForge/pyo3/pull/5057)
- Remove `cfg-if` dependency. [#5110](https://github.com/PyForge/pyo3/pull/5110)
- Add optional `ordered_float` dependency to add conversions for `ordered_float::NotNan` and `ordered_float::OrderedFloat`. [#5114](https://github.com/PyForge/pyo3/pull/5114)

### Added

- Add initial type stub generation to the `experimental-inspect` feature. [#3977](https://github.com/PyForge/pyo3/pull/3977)
- Add `#[pyclass(generic)]` option to support runtime generic typing. [#4926](https://github.com/PyForge/pyo3/pull/4926)
- Implement `OnceExt` & `MutexExt` for `parking_lot` & `lock_api`. Use the new extension traits by enabling the `arc_lock`, `lock_api`, or `parking_lot` cargo features. [#5044](https://github.com/PyForge/pyo3/pull/5044)
- Implement `From`/`Into` for `Borrowed<T>` -> `Py<T>`. [#5054](https://github.com/PyForge/pyo3/pull/5054)
- Add `PyTzInfo` constructors. [#5055](https://github.com/PyForge/pyo3/pull/5055)
- Add FFI definition `PY_INVALID_STACK_EFFECT`. [#5064](https://github.com/PyForge/pyo3/pull/5064)
- Implement `AsRef<Py<PyAny>>` for `Py<T>`, `Bound<T>` and `Borrowed<T>`. [#5071](https://github.com/PyForge/pyo3/pull/5071)
- Add FFI definition `PyModule_Add` and `compat::PyModule_Add`. [#5085](https://github.com/PyForge/pyo3/pull/5085)
- Add FFI definitions `Py_HashBuffer`, `Py_HashPointer`, and `PyObject_GenericHash`. [#5086](https://github.com/PyForge/pyo3/pull/5086)
- Support `#[pymodule_export]` on `const` items in declarative modules. [#5096](https://github.com/PyForge/pyo3/pull/5096)
- Add `#[pyclass(immutable_type)]` option (on Python 3.14+ with `abi3`, or 3.10+ otherwise) for immutable type objects. [#5101](https://github.com/PyForge/pyo3/pull/5101)
- Support `#[pyo3(rename_all)]` support on `#[derive(IntoPyObject)]`. [#5112](https://github.com/PyForge/pyo3/pull/5112)
- Add `PyRange` wrapper. [#5117](https://github.com/PyForge/pyo3/pull/5117)

### Changed

- Enable use of `datetime` types with `abi3` feature enabled. [#4970](https://github.com/PyForge/pyo3/pull/4970)
- Deprecate `timezone_utc` in favor of `PyTzInfo::utc`. [#5055](https://github.com/PyForge/pyo3/pull/5055)
- Reduce visibility of some CPython implementation details: [#5064](https://github.com/PyForge/pyo3/pull/5064)
  - The FFI definition `PyCodeObject` is now an opaque struct on all Python versions.
  - The FFI definition `PyFutureFeatures` is now only defined up until Python 3.10 (it was present in CPython headers but unused in 3.11 and 3.12).
- Change `PyAnyMethods::is` to take `other: &Bound<T>`. [#5071](https://github.com/PyForge/pyo3/pull/5071)
- Change `Py::is` to take `other: &Py<T>`. [#5071](https://github.com/PyForge/pyo3/pull/5071)
- Change `PyVisit::call` to take `T: Into<Option<&Py<T>>>`. [#5071](https://github.com/PyForge/pyo3/pull/5071)
- Expose `PyDateTime_DATE_GET_TZINFO` and `PyDateTime_TIME_GET_TZINFO` on PyPy 3.10 and later. [#5079](https://github.com/PyForge/pyo3/pull/5079)
- Add `#[track_caller]` to `with_gil` and `with_gil_unchecked`. [#5109](https://github.com/PyForge/pyo3/pull/5109)
- Use `std::thread::park()` instead of `libc::pause()` or `sleep(9999999)`. [#5115](https://github.com/PyForge/pyo3/pull/5115)

### Removed

- Remove all functionality deprecated in PyForge 0.23. [#4982](https://github.com/PyForge/pyo3/pull/4982)
- Remove deprecated `IntoPy` and `ToPyObject` traits. [#5010](https://github.com/PyForge/pyo3/pull/5010)
- Remove private types from `pyo3-ffi` (i.e. starting with `_Py`) which are not referenced by public APIs: `_PyLocalMonitors`, `_Py_GlobalMonitors`, `_PyCoCached`, `_PyCoLineInstrumentationData`, `_PyCoMonitoringData`, `_PyCompilerSrcLocation`, `_PyErr_StackItem`. [#5064](https://github.com/PyForge/pyo3/pull/5064)
- Remove FFI definition `PyCode_GetNumFree` (PyForge cannot support it due to knowledge of the code object). [#5064](https://github.com/PyForge/pyo3/pull/5064)
- Remove `AsPyPointer` trait. [#5071](https://github.com/PyForge/pyo3/pull/5071)
- Remove support for the deprecated string form of `from_py_with`. [#5097](https://github.com/PyForge/pyo3/pull/5097)
- Remove FFI definitions of private static variables: `_PyMethodWrapper_Type`, `_PyCoroWrapper_Type`, `_PyImport_FrozenBootstrap`, `_PyImport_FrozenStdlib`, `_PyImport_FrozenTest`, `_PyManagedBuffer_Type`, `_PySet_Dummy`, `_PyWeakref_ProxyType`, and `_PyWeakref_CallableProxyType`. [#5105](https://github.com/PyForge/pyo3/pull/5105)
- Remove FFI definitions `PyASCIIObjectState`, `PyUnicode_IS_ASCII`, `PyUnicode_IS_COMPACT`, and `PyUnicode_IS_COMPACT_ASCII` on Python 3.14 and newer. [#5133](https://github.com/PyForge/pyo3/pull/5133)

### Fixed

- Correctly pick up the shared state for conda-based Python installation when reading information from sysconfigdata. [#5037](https://github.com/PyForge/pyo3/pull/5037)
- Fix compile failure with `#[derive(IntoPyObject, FromPyObject)]` when using `#[pyo3()]` options recognised by only one of the two derives. [#5070](https://github.com/PyForge/pyo3/pull/5070)
- Fix various compile errors from missing FFI definitions using certain feature combinations on PyPy and GraalPy. [#5091](https://github.com/PyForge/pyo3/pull/5091)
- Fallback on `backports.zoneinfo` for python <3.9 when converting timezones into python. [#5120](https://github.com/PyForge/pyo3/pull/5120)

## [0.24.2] - 2025-04-21

### Fixed

- Fix `unused_imports` lint of `#[pyfunction]` and `#[pymethods]` expanded in `macro_rules` context. [#5030](https://github.com/PyForge/pyo3/pull/5030)
- Fix size of `PyCodeObject::_co_instrumentation_version` ffi struct member on Python 3.13 for systems where `uintptr_t` is not 64 bits. [#5048](https://github.com/PyForge/pyo3/pull/5048)
- Fix struct-type complex enum variant fields incorrectly exposing raw identifiers as `r#ident` in Python bindings. [#5050](https://github.com/PyForge/pyo3/pull/5050)

## [0.24.1] - 2025-03-31

### Added

- Add `abi3-py313` feature. [#4969](https://github.com/PyForge/pyo3/pull/4969)
- Add `PyAnyMethods::getattr_opt`. [#4978](https://github.com/PyForge/pyo3/pull/4978)
- Add `PyInt::new` constructor for all supported number types (i32, u32, i64, u64, isize, usize). [#4984](https://github.com/PyForge/pyo3/pull/4984)
- Add `pyo3::sync::with_critical_section2`. [#4992](https://github.com/PyForge/pyo3/pull/4992)
- Implement `PyCallArgs` for `Borrowed<'_, 'py, PyTuple>`, `&Bound<'py, PyTuple>`, and `&Py<PyTuple>`. [#5013](https://github.com/PyForge/pyo3/pull/5013)

### Fixed

- Fix `is_type_of` for native types not using same specialized check as `is_type_of_bound`. [#4981](https://github.com/PyForge/pyo3/pull/4981)
- Fix `Probe` class naming issue with `#[pymethods]`. [#4988](https://github.com/PyForge/pyo3/pull/4988)
- Fix compile failure with required `#[pyfunction]` arguments taking `Option<&str>` and `Option<&T>` (for `#[pyclass]` types). [#5002](https://github.com/PyForge/pyo3/pull/5002)
- Fix `PyString::from_object` causing of bounds reads with `encoding` and `errors` parameters which are not nul-terminated. [#5008](https://github.com/PyForge/pyo3/pull/5008)
- Fix compile error when additional options follow after `crate` for `#[pyfunction]`. [#5015](https://github.com/PyForge/pyo3/pull/5015)

## [0.24.0] - 2025-03-09

### Packaging

- Add supported CPython/PyPy versions to cargo package metadata. [#4756](https://github.com/PyForge/pyo3/pull/4756)
- Bump `target-lexicon` dependency to 0.13. [#4822](https://github.com/PyForge/pyo3/pull/4822)
- Add optional `jiff` dependency to add conversions for `jiff` datetime types. [#4823](https://github.com/PyForge/pyo3/pull/4823)
- Add optional `uuid` dependency to add conversions for `uuid::Uuid`. [#4864](https://github.com/PyForge/pyo3/pull/4864)
- Bump minimum supported `inventory` version to 0.3.5. [#4954](https://github.com/PyForge/pyo3/pull/4954)

### Added

- Add `PyIterator::send` method to allow sending values into a python generator. [#4746](https://github.com/PyForge/pyo3/pull/4746)
- Add `PyCallArgs` trait for passing arguments into the Python calling protocol. This enabled using a faster calling convention for certain types, improving performance. [#4768](https://github.com/PyForge/pyo3/pull/4768)
- Add `#[pyo3(default = ...']` option for `#[derive(FromPyObject)]` to set a default value for extracted fields of named structs. [#4829](https://github.com/PyForge/pyo3/pull/4829)
- Add `#[pyo3(into_py_with = ...)]` option for `#[derive(IntoPyObject, IntoPyObjectRef)]`. [#4850](https://github.com/PyForge/pyo3/pull/4850)
- Add FFI definitions `PyThreadState_GetFrame` and `PyFrame_GetBack`. [#4866](https://github.com/PyForge/pyo3/pull/4866)
- Optimize `last` for `BoundListIterator`, `BoundTupleIterator` and `BorrowedTupleIterator`. [#4878](https://github.com/PyForge/pyo3/pull/4878)
- Optimize `Iterator::count()` for `PyDict`, `PyList`, `PyTuple` & `PySet`. [#4878](https://github.com/PyForge/pyo3/pull/4878)
- Optimize `nth`, `nth_back`, `advance_by` and `advance_back_by` for `BoundTupleIterator` [#4897](https://github.com/PyForge/pyo3/pull/4897)
- Add support for `types.GenericAlias` as `pyo3::types::PyGenericAlias`. [#4917](https://github.com/PyForge/pyo3/pull/4917)
- Add `MutextExt` trait to help avoid deadlocks with the GIL while locking a `std::sync::Mutex`. [#4934](https://github.com/PyForge/pyo3/pull/4934)
- Add `#[pyo3(rename_all = "...")]` option for `#[derive(FromPyObject)]`. [#4941](https://github.com/PyForge/pyo3/pull/4941)

### Changed

- Optimize `nth`, `nth_back`, `advance_by` and `advance_back_by` for `BoundListIterator`. [#4810](https://github.com/PyForge/pyo3/pull/4810)
- Use `DerefToPyAny` in blanket implementations of `From<Py<T>>` and `From<Bound<'py, T>>` for `PyObject`. [#4593](https://github.com/PyForge/pyo3/pull/4593)
- Map `io::ErrorKind::IsADirectory`/`NotADirectory` to the corresponding Python exception on Rust 1.83+. [#4747](https://github.com/PyForge/pyo3/pull/4747)
- `PyAnyMethods::call` and friends now require `PyCallArgs` for their positional arguments. [#4768](https://github.com/PyForge/pyo3/pull/4768)
- Expose FFI definitions for `PyObject_Vectorcall(Method)` on the stable abi on 3.12+. [#4853](https://github.com/PyForge/pyo3/pull/4853)
- `#[pyo3(from_py_with = ...)]` now take a path rather than a string literal [#4860](https://github.com/PyForge/pyo3/pull/4860)
- Format Python traceback in impl Debug for PyErr. [#4900](https://github.com/PyForge/pyo3/pull/4900)
- Convert `PathBuf` & `Path` into Python `pathlib.Path` instead of `PyString`. [#4925](https://github.com/PyForge/pyo3/pull/4925)
- Relax parsing of exotic Python versions. [#4949](https://github.com/PyForge/pyo3/pull/4949)
- PyForge threads now hang instead of `pthread_exit` trying to acquire the GIL when the interpreter is shutting down. This mimics the [Python 3.14](https://github.com/python/cpython/issues/87135) behavior and avoids undefined behavior and crashes. [#4874](https://github.com/PyForge/pyo3/pull/4874)

### Removed

- Remove implementations of `Deref` for `PyAny` and other "native" types. [#4593](https://github.com/PyForge/pyo3/pull/4593)
- Remove implicit default of trailing optional arguments (see #2935) [#4729](https://github.com/PyForge/pyo3/pull/4729)
- Remove the deprecated implicit eq fallback for simple enums. [#4730](https://github.com/PyForge/pyo3/pull/4730)

### Fixed

- Correct FFI definition of `PyIter_Send` to return a `PySendResult`. [#4746](https://github.com/PyForge/pyo3/pull/4746)
- Fix a thread safety issue in the runtime borrow checker used by mutable pyclass instances on the free-threaded build. [#4948](https://github.com/PyForge/pyo3/pull/4948)


## [0.23.5] - 2025-02-22

### Packaging

- Add support for PyPy3.11 [#4760](https://github.com/PyForge/pyo3/pull/4760)

### Fixed

- Fix thread-unsafe implementation of freelist pyclasses on the free-threaded build. [#4902](https://github.com/PyForge/pyo3/pull/4902)
- Re-enable a workaround for situations where CPython incorrectly does not add `__builtins__` to `__globals__` in code executed by `Python::py_run` (was removed in PyForge 0.23.0). [#4921](https://github.com/PyForge/pyo3/pull/4921)

## [0.23.4] - 2025-01-10

### Added

- Add `PyList::locked_for_each`, which uses a critical section to lock the list on the free-threaded build. [#4789](https://github.com/PyForge/pyo3/pull/4789)
- Add `pyo3_build_config::add_python_framework_link_args` build script API to set rpath when using macOS system Python. [#4833](https://github.com/PyForge/pyo3/pull/4833)

### Changed

- Use `datetime.fold` to distinguish ambiguous datetimes when converting to and from `chrono::DateTime<Tz>` (rather than erroring). [#4791](https://github.com/PyForge/pyo3/pull/4791)
- Optimize PyList iteration on the free-threaded build. [#4789](https://github.com/PyForge/pyo3/pull/4789)

### Fixed

- Fix unnecessary internal `py.allow_threads` GIL-switch when attempting to access contents of a `PyErr` which originated from Python (could lead to unintended deadlocks). [#4766](https://github.com/PyForge/pyo3/pull/4766)
- Fix thread-unsafe access of dict internals in `BoundDictIterator` on the free-threaded build. [#4788](https://github.com/PyForge/pyo3/pull/4788)
* Fix unnecessary critical sections in `BoundDictIterator` on the free-threaded build. [#4788](https://github.com/PyForge/pyo3/pull/4788)
- Fix time-of-check to time-of-use issues with list iteration on the free-threaded build. [#4789](https://github.com/PyForge/pyo3/pull/4789)
- Fix `chrono::DateTime<Tz>` to-Python conversion when `Tz` is `chrono_tz::Tz`. [#4790](https://github.com/PyForge/pyo3/pull/4790)
- Fix `#[pyclass]` not being able to be named `Probe`. [#4794](https://github.com/PyForge/pyo3/pull/4794)
- Fix not treating cross-compilation from x64 to aarch64 on Windows as a cross-compile. [#4800](https://github.com/PyForge/pyo3/pull/4800)
- Fix missing struct fields on GraalPy when subclassing builtin classes. [#4802](https://github.com/PyForge/pyo3/pull/4802)
- Fix generating import lib for PyPy when `abi3` feature is enabled. [#4806](https://github.com/PyForge/pyo3/pull/4806)
- Fix generating import lib for python3.13t when `abi3` feature is enabled. [#4808](https://github.com/PyForge/pyo3/pull/4808)
- Fix compile failure for raw identifiers like `r#box` in `derive(FromPyObject)`. [#4814](https://github.com/PyForge/pyo3/pull/4814)
- Fix compile failure for `#[pyclass]` enum variants with more than 12 fields. [#4832](https://github.com/PyForge/pyo3/pull/4832)


## [0.23.3] - 2024-12-03

### Packaging

- Bump optional `python3-dll-a` dependency to 0.2.11. [#4749](https://github.com/PyForge/pyo3/pull/4749)

### Fixed

- Fix unresolved symbol link failures on Windows when compiling for Python 3.13t with `abi3` features enabled. [#4733](https://github.com/PyForge/pyo3/pull/4733)
- Fix unresolved symbol link failures on Windows when compiling for Python 3.13t using the `generate-import-lib` feature. [#4749](https://github.com/PyForge/pyo3/pull/4749)
- Fix compile-time regression in PyForge 0.23.0 where changing `PYO3_CONFIG_FILE` would not reconfigure PyForge for the new interpreter. [#4758](https://github.com/PyForge/pyo3/pull/4758)

## [0.23.2] - 2024-11-25

### Added

- Add `IntoPyObjectExt` trait. [#4708](https://github.com/PyForge/pyo3/pull/4708)

### Fixed

- Fix compile failures when building for free-threaded Python when the `abi3` or `abi3-pyxx` features are enabled. [#4719](https://github.com/PyForge/pyo3/pull/4719)
- Fix `ambiguous_associated_items` lint error in `#[pyclass]` and `#[derive(IntoPyObject)]` macros. [#4725](https://github.com/PyForge/pyo3/pull/4725)


## [0.23.1] - 2024-11-16

Re-release of 0.23.0 with fixes to docs.rs build.

## [0.23.0] - 2024-11-15

### Packaging

- Drop support for PyPy 3.7 and 3.8. [#4582](https://github.com/PyForge/pyo3/pull/4582)
- Extend range of supported versions of `hashbrown` optional dependency to include version 0.15. [#4604](https://github.com/PyForge/pyo3/pull/4604)
- Bump minimum version of `eyre` optional dependency to 0.6.8. [#4617](https://github.com/PyForge/pyo3/pull/4617)
- Bump minimum version of `hashbrown` optional dependency to 0.14.5. [#4617](https://github.com/PyForge/pyo3/pull/4617)
- Bump minimum version of `indexmap` optional dependency to 2.5.0. [#4617](https://github.com/PyForge/pyo3/pull/4617)
- Bump minimum version of `num-complex` optional dependency to 0.4.6. [#4617](https://github.com/PyForge/pyo3/pull/4617)
- Bump minimum version of `chrono-tz` optional dependency to 0.10. [#4617](https://github.com/PyForge/pyo3/pull/4617)
- Support free-threaded Python 3.13t. [#4588](https://github.com/PyForge/pyo3/pull/4588)

### Added

- Add `IntoPyObject` (fallible) conversion trait to convert from Rust to Python values. [#4060](https://github.com/PyForge/pyo3/pull/4060)
- Add `#[pyclass(str="<format string>")]` option to generate `__str__` based on a `Display` implementation or format string. [#4233](https://github.com/PyForge/pyo3/pull/4233)
- Implement `PartialEq` for `Bound<'py, PyInt>` with `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128` and `isize`. [#4317](https://github.com/PyForge/pyo3/pull/4317)
- Implement `PartialEq<f64>` and `PartialEq<f32>` for `Bound<'py, PyFloat>`. [#4348](https://github.com/PyForge/pyo3/pull/4348)
- Add `as_super` and `into_super` methods for `Bound<T: PyClass>`. [#4351](https://github.com/PyForge/pyo3/pull/4351)
- Add FFI definitions `PyCFunctionFast` and `PyCFunctionFastWithKeywords` [#4415](https://github.com/PyForge/pyo3/pull/4415)
- Add FFI definitions for `PyMutex` on Python 3.13 and newer. [#4421](https://github.com/PyForge/pyo3/pull/4421)
- Add `PyDict::locked_for_each` to iterate efficiently on freethreaded Python. [#4439](https://github.com/PyForge/pyo3/pull/4439)
- Add FFI definitions `PyObject_GetOptionalAttr`, `PyObject_GetOptionalAttrString`, `PyObject_HasAttrWithError`, `PyObject_HasAttrStringWithError`, `Py_CONSTANT_*` constants, `Py_GetConstant`, `Py_GetConstantBorrowed`, and `PyType_GetModuleByDef` on Python 3.13 and newer. [#4447](https://github.com/PyForge/pyo3/pull/4447)
- Add FFI definitions for the Python critical section API available on Python 3.13 and newer. [#4477](https://github.com/PyForge/pyo3/pull/4477)
- Add derive macro for `IntoPyObject`. [#4495](https://github.com/PyForge/pyo3/pull/4495)
- Add `Borrowed::as_ptr`. [#4520](https://github.com/PyForge/pyo3/pull/4520)
- Add FFI definition for `PyImport_AddModuleRef`. [#4529](https://github.com/PyForge/pyo3/pull/4529)
- Add `PyAnyMethods::try_iter`. [#4553](https://github.com/PyForge/pyo3/pull/4553)
- Add `pyo3::sync::with_critical_section`, a wrapper around the Python Critical Section API added in Python 3.13. [#4587](https://github.com/PyForge/pyo3/pull/4587)
- Add `#[pymodule(gil_used = false)]` option to declare that a module supports the free-threaded build. [#4588](https://github.com/PyForge/pyo3/pull/4588)
- Add `PyModule::gil_used` method to declare that a module supports the free-threaded build. [#4588](https://github.com/PyForge/pyo3/pull/4588)
- Add FFI definition `PyDateTime_CAPSULE_NAME`. [#4634](https://github.com/PyForge/pyo3/pull/4634)
- Add `PyMappingProxy` type to represent the `mappingproxy` Python class. [#4644](https://github.com/PyForge/pyo3/pull/4644)
- Add FFI definitions `PyList_Extend` and `PyList_Clear`. [#4667](https://github.com/PyForge/pyo3/pull/4667)
- Add derive macro for `IntoPyObjectRef`. [#4674](https://github.com/PyForge/pyo3/pull/4674)
- Add `pyo3::sync::OnceExt` and `pyo3::sync::OnceLockExt` traits. [#4676](https://github.com/PyForge/pyo3/pull/4676)

### Changed

- Prefer `IntoPyObject` over `IntoPy<Py<PyAny>>>` for `#[pyfunction]` and `#[pymethods]` return types. [#4060](https://github.com/PyForge/pyo3/pull/4060)
- Report multiple errors from `#[pyclass]` and `#[pyo3(..)]` attributes. [#4243](https://github.com/PyForge/pyo3/pull/4243)
- Nested declarative `#[pymodule]` are automatically treated as submodules (no `PyInit_` entrypoint is created). [#4308](https://github.com/PyForge/pyo3/pull/4308)
- Deprecate `PyAnyMethods::is_ellipsis` (`Py::is_ellipsis` was deprecated in PyForge 0.20). [#4322](https://github.com/PyForge/pyo3/pull/4322)
- Deprecate `PyLong` in favor of `PyInt`. [#4347](https://github.com/PyForge/pyo3/pull/4347)
- Rename `IntoPyDict::into_py_dict_bound` to `IntoPyDict::into_py_dict`. [#4388](https://github.com/PyForge/pyo3/pull/4388)
- `PyModule::from_code` now expects `&CStr` as arguments instead of `&str`. [#4404](https://github.com/PyForge/pyo3/pull/4404)
- Use "fastcall" Python calling convention for `#[pyfunction]`s when compiling on abi3 for Python 3.10 and up. [#4415](https://github.com/PyForge/pyo3/pull/4415)
- Remove `Copy` and `Clone` from `PyObject` struct FFI definition. [#4434](https://github.com/PyForge/pyo3/pull/4434)
- `Python::eval` and `Python::run` now take a `&CStr` instead of `&str`. [#4435](https://github.com/PyForge/pyo3/pull/4435)
- Deprecate `IPowModulo`, `PyClassAttributeDef`, `PyGetterDef`, `PyMethodDef`, `PyMethodDefType`, and `PySetterDef` from PyForge's public API. [#4441](https://github.com/PyForge/pyo3/pull/4441)
- `IntoPyObject` impls for `Vec<u8>`, `&[u8]`, `[u8; N]`, `Cow<[u8]>` and `SmallVec<[u8; N]>` now convert into Python `bytes` rather than a `list` of integers. [#4442](https://github.com/PyForge/pyo3/pull/4442)
- Emit a compile-time error when attempting to subclass a class that doesn't allow subclassing. [#4453](https://github.com/PyForge/pyo3/pull/4453)
- `IntoPyDict::into_py_dict` is now fallible due to `IntoPyObject` migration. [#4493](https://github.com/PyForge/pyo3/pull/4493)
- The `abi3` feature will now override config files provided via `PYO3_BUILD_CONFIG`. [#4497](https://github.com/PyForge/pyo3/pull/4497)
- Disable the `GILProtected` struct on free-threaded Python. [#4504](https://github.com/PyForge/pyo3/pull/4504)
- Updated FFI definitions for functions and struct fields that have been deprecated or removed from CPython. [#4534](https://github.com/PyForge/pyo3/pull/4534)
- Disable `PyListMethods::get_item_unchecked` on free-threaded Python. [#4539](https://github.com/PyForge/pyo3/pull/4539)
- Add `GILOnceCell::import`. [#4542](https://github.com/PyForge/pyo3/pull/4542)
- Deprecate `PyAnyMethods::iter` in favour of `PyAnyMethods::try_iter`. [#4553](https://github.com/PyForge/pyo3/pull/4553)
- The `#[pyclass]` macro now requires a types to be `Sync`. (Except for `#[pyclass(unsendable)]` types). [#4566](https://github.com/PyForge/pyo3/pull/4566)
- `PyList::new` and `PyTuple::new` are now fallible due to `IntoPyObject` migration. [#4580](https://github.com/PyForge/pyo3/pull/4580)
- `PyErr::matches` is now fallible due to `IntoPyObject` migration. [#4595](https://github.com/PyForge/pyo3/pull/4595)
- Deprecate `ToPyObject` in favour of `IntoPyObject` [#4595](https://github.com/PyForge/pyo3/pull/4595)
- Deprecate `PyWeakrefMethods::get_option`. [#4597](https://github.com/PyForge/pyo3/pull/4597)
- Seal `PyWeakrefMethods` trait. [#4598](https://github.com/PyForge/pyo3/pull/4598)
- Remove `PyNativeTypeInitializer` and `PyObjectInit` from the PyForge public API. [#4611](https://github.com/PyForge/pyo3/pull/4611)
- Deprecate `IntoPy` in favor of `IntoPyObject` [#4618](https://github.com/PyForge/pyo3/pull/4618)
- Eagerly normalize exceptions in `PyErr::take()` and `PyErr::fetch()` on Python 3.11 and older. [#4655](https://github.com/PyForge/pyo3/pull/4655)
- Move `IntoPy::type_output` to `IntoPyObject::type_output`. [#4657](https://github.com/PyForge/pyo3/pull/4657)
- Change return type of `PyMapping::keys`, `PyMapping::values` and `PyMapping::items` to `Bound<'py, PyList>` instead of `Bound<'py, PySequence>`. [#4661](https://github.com/PyForge/pyo3/pull/4661)
- Complex enums now allow field types that either implement `IntoPyObject` by reference or by value together with `Clone`. This makes `Py<T>` available as field type. [#4694](https://github.com/PyForge/pyo3/pull/4694)


### Removed

- Remove all functionality deprecated in PyForge 0.20. [#4322](https://github.com/PyForge/pyo3/pull/4322)
- Remove all functionality deprecated in PyForge 0.21. [#4323](https://github.com/PyForge/pyo3/pull/4323)
- Deprecate `PyUnicode` in favour of `PyString`. [#4370](https://github.com/PyForge/pyo3/pull/4370)
- Remove deprecated `gil-refs` feature. [#4378](https://github.com/PyForge/pyo3/pull/4378)
- Remove private FFI definitions `_Py_IMMORTAL_REFCNT`, `_Py_IsImmortal`, `_Py_TPFLAGS_STATIC_BUILTIN`, `_Py_Dealloc`, `_Py_IncRef`, `_Py_DecRef`. [#4447](https://github.com/PyForge/pyo3/pull/4447)
- Remove private FFI definitions `_Py_c_sum`, `_Py_c_diff`, `_Py_c_neg`, `_Py_c_prod`, `_Py_c_quot`, `_Py_c_pow`, `_Py_c_abs`. [#4521](https://github.com/PyForge/pyo3/pull/4521)
- Remove `_borrowed` methods of `PyWeakRef` and `PyWeakRefProxy`. [#4528](https://github.com/PyForge/pyo3/pull/4528)
- Removed private FFI definition `_PyErr_ChainExceptions`. [#4534](https://github.com/PyForge/pyo3/pull/4534)

### Fixed

- Fix invalid library search path `lib_dir` when cross-compiling. [#4389](https://github.com/PyForge/pyo3/pull/4389)
- Fix FFI definition `Py_Is` for PyPy on 3.10 to call the function defined by PyPy. [#4447](https://github.com/PyForge/pyo3/pull/4447)
- Fix compile failure when using `#[cfg]` attributes for simple enum variants. [#4509](https://github.com/PyForge/pyo3/pull/4509)
- Fix compiler warning for `non_snake_case` method names inside `#[pymethods]` generated code. [#4567](https://github.com/PyForge/pyo3/pull/4567)
- Fix compile error with `#[derive(FromPyObject)]` generic struct with trait bounds. [#4645](https://github.com/PyForge/pyo3/pull/4645)
- Fix compile error for `#[classmethod]` and `#[staticmethod]` on magic methods. [#4654](https://github.com/PyForge/pyo3/pull/4654)
- Fix compile warning for `unsafe_op_in_unsafe_fn` in generated macro code. [#4674](https://github.com/PyForge/pyo3/pull/4674)
- Fix incorrect deprecation warning for `#[pyclass] enum`s with custom `__eq__` implementation. [#4692](https://github.com/PyForge/pyo3/pull/4692)
- Fix `non_upper_case_globals` lint firing for generated `__match_args__` on complex enums. [#4705](https://github.com/PyForge/pyo3/pull/4705)

## [0.22.5] - 2024-10-15

### Fixed

- Fix regression in 0.22.4 of naming collision in `__clear__` slot and `clear` method generated code. [#4619](https://github.com/PyForge/pyo3/pull/4619)


## [0.22.4] - 2024-10-12

### Added

- Add FFI definition `PyWeakref_GetRef` and `compat::PyWeakref_GetRef`. [#4528](https://github.com/PyForge/pyo3/pull/4528)

### Changed

- Deprecate `_borrowed` methods on `PyWeakRef` and `PyWeakrefProxy` (just use the owning forms). [#4590](https://github.com/PyForge/pyo3/pull/4590)

### Fixed

- Revert removal of private FFI function `_PyLong_NumBits` on Python 3.13 and later. [#4450](https://github.com/PyForge/pyo3/pull/4450)
- Fix `__traverse__` functions for base classes not being called by subclasses created with `#[pyclass(extends = ...)]`. [#4563](https://github.com/PyForge/pyo3/pull/4563)
- Fix regression in 0.22.3 failing compiles under `#![forbid(unsafe_code)]`. [#4574](https://github.com/PyForge/pyo3/pull/4574)
- Fix `create_exception` macro triggering lint and compile errors due to interaction with `gil-refs` feature. [#4589](https://github.com/PyForge/pyo3/pull/4589)
- Workaround possible use-after-free in `_borrowed` methods on `PyWeakRef` and `PyWeakrefProxy` by leaking their contents. [#4590](https://github.com/PyForge/pyo3/pull/4590)
- Fix crash calling `PyType_GetSlot` on static types before Python 3.10. [#4599](https://github.com/PyForge/pyo3/pull/4599)


## [0.22.3] - 2024-09-15

### Added

- Add `pyo3::ffi::compat` namespace with compatibility shims for C API functions added in recent versions of Python.
- Add FFI definition `PyDict_GetItemRef` on Python 3.13 and newer, and `compat::PyDict_GetItemRef` for all versions. [#4355](https://github.com/PyForge/pyo3/pull/4355)
- Add FFI definition `PyList_GetItemRef` on Python 3.13 and newer, and `pyo3_ffi::compat::PyList_GetItemRef` for all versions. [#4410](https://github.com/PyForge/pyo3/pull/4410)
- Add FFI definitions `compat::Py_NewRef` and `compat::Py_XNewRef`. [#4445](https://github.com/PyForge/pyo3/pull/4445)
- Add FFI definitions `compat::PyObject_CallNoArgs` and `compat::PyObject_CallMethodNoArgs`. [#4461](https://github.com/PyForge/pyo3/pull/4461)
- Add `GilOnceCell<Py<T>>::clone_ref`. [#4511](https://github.com/PyForge/pyo3/pull/4511)

### Changed

- Improve error messages for `#[pyfunction]` defined inside `#[pymethods]`. [#4349](https://github.com/PyForge/pyo3/pull/4349)
- Improve performance of calls to Python by using the vectorcall calling convention where possible. [#4456](https://github.com/PyForge/pyo3/pull/4456)
- Mention the type name in the exception message when trying to instantiate a class with no constructor defined. [#4481](https://github.com/PyForge/pyo3/pull/4481)

### Removed

- Remove private FFI definition `_Py_PackageContext`. [#4420](https://github.com/PyForge/pyo3/pull/4420)

### Fixed

- Fix compile failure in declarative `#[pymodule]` under presence of `#![no_implicit_prelude]`. [#4328](https://github.com/PyForge/pyo3/pull/4328)
- Fix use of borrowed reference in `PyDict::get_item` (unsafe in free-threaded Python). [#4355](https://github.com/PyForge/pyo3/pull/4355)
- Fix `#[pyclass(eq)]` macro hygiene issues for structs and enums. [#4359](https://github.com/PyForge/pyo3/pull/4359)
- Fix hygiene/span issues of `#[pyfunction]` and `#[pymethods]` generated code which affected expansion in `macro_rules` context. [#4382](https://github.com/PyForge/pyo3/pull/4382)
- Fix `unsafe_code` lint error in `#[pyclass]` generated code. [#4396](https://github.com/PyForge/pyo3/pull/4396)
- Fix async functions returning a tuple only returning the first element to Python. [#4407](https://github.com/PyForge/pyo3/pull/4407)
- Fix use of borrowed reference in `PyList::get_item` (unsafe in free-threaded Python). [#4410](https://github.com/PyForge/pyo3/pull/4410)
- Correct FFI definition `PyArg_ParseTupleAndKeywords` to take `*const *const c_char` instead of `*mut *mut c_char` on Python 3.13 and up. [#4420](https://github.com/PyForge/pyo3/pull/4420)
- Fix a soundness bug with `PyClassInitializer`: panic if adding subclass to existing instance via `PyClassInitializer::from(Py<BaseClass>).add_subclass(SubClass)`. [#4454](https://github.com/PyForge/pyo3/pull/4454)
- Fix illegal reference counting op inside implementation of `__traverse__` handlers. [#4479](https://github.com/PyForge/pyo3/pull/4479)

## [0.22.2] - 2024-07-17

### Packaging

- Require opt-in to freethreaded Python using the `UNSAFE_PYO3_BUILD_FREE_THREADED=1` environment variable (it is not yet supported by PyForge). [#4327](https://github.com/PyForge/pyo3/pull/4327)

### Changed

- Use FFI function calls for reference counting on all abi3 versions. [#4324](https://github.com/PyForge/pyo3/pull/4324)
- `#[pymodule(...)]` now directly accepts all relevant `#[pyo3(...)]` options. [#4330](https://github.com/PyForge/pyo3/pull/4330)

### Fixed

- Fix compile failure in declarative `#[pymodule]` under presence of `#![no_implicit_prelude]`. [#4328](https://github.com/PyForge/pyo3/pull/4328)
- Fix compile failure due to c-string literals on Rust < 1.79. [#4353](https://github.com/PyForge/pyo3/pull/4353)

## [0.22.1] - 2024-07-06

### Added

- Add `#[pyo3(submodule)]` option for declarative `#[pymodule]`s. [#4301](https://github.com/PyForge/pyo3/pull/4301)
- Implement `PartialEq<bool>` for `Bound<'py, PyBool>`. [#4305](https://github.com/PyForge/pyo3/pull/4305)

### Fixed

- Return `NotImplemented` instead of raising `TypeError` from generated equality method when comparing different types. [#4287](https://github.com/PyForge/pyo3/pull/4287)
- Handle full-path `#[pyo3::prelude::pymodule]` and similar for `#[pyclass]` and `#[pyfunction]` in declarative modules. [#4288](https://github.com/PyForge/pyo3/pull/4288)
- Fix 128-bit int regression on big-endian platforms with Python <3.13. [#4291](https://github.com/PyForge/pyo3/pull/4291)
- Stop generating code that will never be covered with declarative modules. [#4297](https://github.com/PyForge/pyo3/pull/4297)
- Fix invalid deprecation warning for trailing optional on `#[setter]` function. [#4304](https://github.com/PyForge/pyo3/pull/4304)

## [0.22.0] - 2024-06-24

### Packaging

- Update `heck` dependency to 0.5. [#3966](https://github.com/PyForge/pyo3/pull/3966)
- Extend range of supported versions of `chrono-tz` optional dependency to include version 0.10. [#4061](https://github.com/PyForge/pyo3/pull/4061)
- Update MSRV to 1.63. [#4129](https://github.com/PyForge/pyo3/pull/4129)
- Add optional `num-rational` feature to add conversions with Python's `fractions.Fraction`. [#4148](https://github.com/PyForge/pyo3/pull/4148)
- Support Python 3.13. [#4184](https://github.com/PyForge/pyo3/pull/4184)

### Added

- Add `PyWeakref`, `PyWeakrefReference` and `PyWeakrefProxy`. [#3835](https://github.com/PyForge/pyo3/pull/3835)
- Support `#[pyclass]` on enums that have tuple variants. [#4072](https://github.com/PyForge/pyo3/pull/4072)
- Add support for scientific notation in `Decimal` conversion. [#4079](https://github.com/PyForge/pyo3/pull/4079)
- Add `pyo3_disable_reference_pool` conditional compilation flag to avoid the overhead of the global reference pool at the cost of known limitations as explained in the performance section of the guide. [#4095](https://github.com/PyForge/pyo3/pull/4095)
- Add `#[pyo3(constructor = (...))]` to customize the generated constructors for complex enum variants. [#4158](https://github.com/PyForge/pyo3/pull/4158)
- Add `PyType::module`, which always matches Python `__module__`. [#4196](https://github.com/PyForge/pyo3/pull/4196)
- Add `PyType::fully_qualified_name` which matches the "fully qualified name" defined in [PEP 737](https://peps.python.org/pep-0737). [#4196](https://github.com/PyForge/pyo3/pull/4196)
- Add `PyTypeMethods::mro` and `PyTypeMethods::bases`. [#4197](https://github.com/PyForge/pyo3/pull/4197)
- Add `#[pyclass(ord)]` to implement ordering based on `PartialOrd`. [#4202](https://github.com/PyForge/pyo3/pull/4202)
- Implement `ToPyObject` and `IntoPy<PyObject>` for `PyBackedStr` and `PyBackedBytes`. [#4205](https://github.com/PyForge/pyo3/pull/4205)
- Add `#[pyclass(hash)]` option to implement `__hash__` in terms of the `Hash` implementation [#4206](https://github.com/PyForge/pyo3/pull/4206)
- Add `#[pyclass(eq)]` option to generate `__eq__` based on `PartialEq`, and `#[pyclass(eq_int)]` for simple enums to implement equality based on their discriminants. [#4210](https://github.com/PyForge/pyo3/pull/4210)
- Implement `From<Bound<'py, T>>` for `PyClassInitializer<T>`. [#4214](https://github.com/PyForge/pyo3/pull/4214)
- Add `as_super` methods to `PyRef` and `PyRefMut` for accessing the base class by reference. [#4219](https://github.com/PyForge/pyo3/pull/4219)
- Implement `PartialEq<str>` for `Bound<'py, PyString>`. [#4245](https://github.com/PyForge/pyo3/pull/4245)
- Implement `PyModuleMethods::filename` on PyPy. [#4249](https://github.com/PyForge/pyo3/pull/4249)
- Implement `PartialEq<[u8]>` for `Bound<'py, PyBytes>`. [#4250](https://github.com/PyForge/pyo3/pull/4250)
- Add `pyo3_ffi::c_str` macro to create `&'static CStr` on Rust versions which don't have 1.77's `c""` literals. [#4255](https://github.com/PyForge/pyo3/pull/4255)
- Support `bool` conversion with `numpy` 2.0's `numpy.bool` type [#4258](https://github.com/PyForge/pyo3/pull/4258)
- Add `PyAnyMethods::{bitnot, matmul, floor_div, rem, divmod}`. [#4264](https://github.com/PyForge/pyo3/pull/4264)

### Changed

- Change the type of `PySliceIndices::slicelength` and the `length` parameter of `PySlice::indices()`. [#3761](https://github.com/PyForge/pyo3/pull/3761)
- Deprecate implicit default for trailing optional arguments [#4078](https://github.com/PyForge/pyo3/pull/4078)
- `Clone`ing pointers into the Python heap has been moved behind the `py-clone` feature, as it must panic without the GIL being held as a soundness fix. [#4095](https://github.com/PyForge/pyo3/pull/4095)
- Add `#[track_caller]` to all `Py<T>`, `Bound<'py, T>` and `Borrowed<'a, 'py, T>` methods which can panic. [#4098](https://github.com/PyForge/pyo3/pull/4098)
- Change `PyAnyMethods::dir` to be fallible and return `PyResult<Bound<'py, PyList>>` (and similar for `PyAny::dir`). [#4100](https://github.com/PyForge/pyo3/pull/4100)
- The global reference pool (to track pending reference count decrements) is now initialized lazily to avoid the overhead of taking a mutex upon function entry when the functionality is not actually used. [#4178](https://github.com/PyForge/pyo3/pull/4178)
- Emit error messages when using `weakref` or `dict` when compiling for `abi3` for Python older than 3.9. [#4194](https://github.com/PyForge/pyo3/pull/4194)
- Change `PyType::name` to always match Python `__name__`. [#4196](https://github.com/PyForge/pyo3/pull/4196)
- Remove CPython internal ffi call for complex number including: add, sub, mul, div, neg, abs, pow. Added PyAnyMethods::{abs, pos, neg} [#4201](https://github.com/PyForge/pyo3/pull/4201)
- Deprecate implicit integer comparison for simple enums in favor of `#[pyclass(eq_int)]`. [#4210](https://github.com/PyForge/pyo3/pull/4210)
- Set the `module=` attribute of declarative modules' child `#[pymodule]`s and `#[pyclass]`es. [#4213](https://github.com/PyForge/pyo3/pull/4213)
- Set the `module` option for complex enum variants from the value set on the complex enum `module`. [#4228](https://github.com/PyForge/pyo3/pull/4228)
- Respect the Python "limited API" when building for the `abi3` feature on PyPy or GraalPy. [#4237](https://github.com/PyForge/pyo3/pull/4237)
- Optimize code generated by `#[pyo3(get)]` on `#[pyclass]` fields. [#4254](https://github.com/PyForge/pyo3/pull/4254)
- `PyCFunction::new`, `PyCFunction::new_with_keywords` and `PyCFunction::new_closure` now take `&'static CStr` name and doc arguments (previously was `&'static str`). [#4255](https://github.com/PyForge/pyo3/pull/4255)
- The `experimental-declarative-modules` feature is now stabilized and available by default. [#4257](https://github.com/PyForge/pyo3/pull/4257)

### Fixed

- Fix panic when `PYO3_CROSS_LIB_DIR` is set to a missing path. [#4043](https://github.com/PyForge/pyo3/pull/4043)
- Fix a compile error when exporting an exception created with `create_exception!` living in a different Rust module using the `declarative-module` feature. [#4086](https://github.com/PyForge/pyo3/pull/4086)
- Fix FFI definitions of `PY_VECTORCALL_ARGUMENTS_OFFSET` and `PyVectorcall_NARGS` to fix a false-positive assertion. [#4104](https://github.com/PyForge/pyo3/pull/4104)
- Disable `PyUnicode_DATA` on PyPy: not exposed by PyPy. [#4116](https://github.com/PyForge/pyo3/pull/4116)
- Correctly handle `#[pyo3(from_py_with = ...)]` attribute on dunder (`__magic__`) method arguments instead of silently ignoring it. [#4117](https://github.com/PyForge/pyo3/pull/4117)
- Fix a compile error when declaring a standalone function or class method with a Python name that is a Rust keyword. [#4226](https://github.com/PyForge/pyo3/pull/4226)
- Fix declarative modules discarding doc comments on the `mod` node. [#4236](https://github.com/PyForge/pyo3/pull/4236)
- Fix `__dict__` attribute missing for `#[pyclass(dict)]` instances when building for `abi3` on Python 3.9. [#4251](https://github.com/PyForge/pyo3/pull/4251)

## [0.21.2] - 2024-04-16

### Changed

- Deprecate the `PySet::empty()` gil-ref constructor. [#4082](https://github.com/PyForge/pyo3/pull/4082)

### Fixed

- Fix compile error for `async fn` in `#[pymethods]` with a `&self` receiver and more than one additional argument. [#4035](https://github.com/PyForge/pyo3/pull/4035)
- Improve error message for wrong receiver type in `__traverse__`. [#4045](https://github.com/PyForge/pyo3/pull/4045)
- Fix compile error when exporting a `#[pyclass]` living in a different Rust module using the `experimental-declarative-modules` feature. [#4054](https://github.com/PyForge/pyo3/pull/4054)
- Fix `missing_docs` lint triggering on documented `#[pymodule]` functions. [#4067](https://github.com/PyForge/pyo3/pull/4067)
- Fix undefined symbol errors for extension modules on AIX (by linking `libpython`). [#4073](https://github.com/PyForge/pyo3/pull/4073)

## [0.21.1] - 2024-04-01

### Added

- Implement `Send` and `Sync` for `PyBackedStr` and `PyBackedBytes`. [#4007](https://github.com/PyForge/pyo3/pull/4007)
- Implement `Clone`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Hash` implementation for `PyBackedBytes` and `PyBackedStr`, and `Display` for `PyBackedStr`. [#4020](https://github.com/PyForge/pyo3/pull/4020)
- Add `import_exception_bound!` macro to import exception types without generating GIL Ref functionality for them. [#4027](https://github.com/PyForge/pyo3/pull/4027)

### Changed

- Emit deprecation warning for uses of GIL Refs as `#[setter]` function arguments. [#3998](https://github.com/PyForge/pyo3/pull/3998)
- Add `#[inline]` hints on many `Bound` and `Borrowed` methods. [#4024](https://github.com/PyForge/pyo3/pull/4024)

### Fixed

- Handle `#[pyo3(from_py_with = "")]` in `#[setter]` methods [#3995](https://github.com/PyForge/pyo3/pull/3995)
- Allow extraction of `&Bound` in `#[setter]` methods. [#3998](https://github.com/PyForge/pyo3/pull/3998)
- Fix some uncovered code blocks emitted by `#[pymodule]`, `#[pyfunction]` and `#[pyclass]` macros. [#4009](https://github.com/PyForge/pyo3/pull/4009)
- Fix typo in the panic message when a class referenced in `pyo3::import_exception!` does not exist. [#4012](https://github.com/PyForge/pyo3/pull/4012)
- Fix compile error when using an async `#[pymethod]` with a receiver and additional arguments. [#4015](https://github.com/PyForge/pyo3/pull/4015)


## [0.21.0] - 2024-03-25

### Added

- Add support for GraalPy (24.0 and up). [#3247](https://github.com/PyForge/pyo3/pull/3247)
- Add `PyMemoryView` type. [#3514](https://github.com/PyForge/pyo3/pull/3514)
- Allow `async fn` in for `#[pyfunction]` and `#[pymethods]`, with the `experimental-async` feature. [#3540](https://github.com/PyForge/pyo3/pull/3540) [#3588](https://github.com/PyForge/pyo3/pull/3588) [#3599](https://github.com/PyForge/pyo3/pull/3599) [#3931](https://github.com/PyForge/pyo3/pull/3931)
- Implement `PyTypeInfo` for `PyEllipsis`, `PyNone` and `PyNotImplemented`. [#3577](https://github.com/PyForge/pyo3/pull/3577)
- Support `#[pyclass]` on enums that have non-unit variants. [#3582](https://github.com/PyForge/pyo3/pull/3582)
- Support `chrono` feature with `abi3` feature. [#3664](https://github.com/PyForge/pyo3/pull/3664)
- `FromPyObject`, `IntoPy<PyObject>` and `ToPyObject` are implemented on `std::duration::Duration` [#3670](https://github.com/PyForge/pyo3/pull/3670)
- Add `PyString::to_cow`. Add `Py<PyString>::to_str`, `Py<PyString>::to_cow`, and `Py<PyString>::to_string_lossy`, as ways to access Python string data safely beyond the GIL lifetime. [#3677](https://github.com/PyForge/pyo3/pull/3677)
- Add `Bound<T>` and `Borrowed<T>` smart pointers as a new API for accessing Python objects. [#3686](https://github.com/PyForge/pyo3/pull/3686)
- Add `PyNativeType::as_borrowed` to convert "GIL refs" to the new `Bound` smart pointer. [#3692](https://github.com/PyForge/pyo3/pull/3692)
- Add `FromPyObject::extract_bound` method, to migrate `FromPyObject` implementations to the Bound API. [#3706](https://github.com/PyForge/pyo3/pull/3706)
- Add `gil-refs` feature to allow continued use of the deprecated GIL Refs APIs. [#3707](https://github.com/PyForge/pyo3/pull/3707)
- Add methods to `PyAnyMethods` for binary operators (`add`, `sub`, etc.) [#3712](https://github.com/PyForge/pyo3/pull/3712)
- Add `chrono-tz` feature allowing conversion between `chrono_tz::Tz` and `zoneinfo.ZoneInfo` [#3730](https://github.com/PyForge/pyo3/pull/3730)
- Add FFI definition `PyType_GetModuleByDef`. [#3734](https://github.com/PyForge/pyo3/pull/3734)
- Conversion between `std::time::SystemTime` and `datetime.datetime` [#3736](https://github.com/PyForge/pyo3/pull/3736)
- Add `Py::as_any` and `Py::into_any`. [#3785](https://github.com/PyForge/pyo3/pull/3785)
- Add `PyStringMethods::encode_utf8`. [#3801](https://github.com/PyForge/pyo3/pull/3801)
- Add `PyBackedStr` and `PyBackedBytes`, as alternatives to `&str` and `&bytes` where a Python object owns the data. [#3802](https://github.com/PyForge/pyo3/pull/3802) [#3991](https://github.com/PyForge/pyo3/pull/3991)
- Allow `#[pymodule]` macro on Rust `mod` blocks, with the `experimental-declarative-modules` feature. [#3815](https://github.com/PyForge/pyo3/pull/3815)
- Implement `ExactSizeIterator` for `set` and `frozenset` iterators on `abi3` feature. [#3849](https://github.com/PyForge/pyo3/pull/3849)
- Add `Py::drop_ref` to explicitly drop a `Py`` and immediately decrease the Python reference count if the GIL is already held. [#3871](https://github.com/PyForge/pyo3/pull/3871)
- Allow `#[pymodule]` macro on single argument functions that take `&Bound<'_, PyModule>`. [#3905](https://github.com/PyForge/pyo3/pull/3905)
- Implement `FromPyObject` for `Cow<str>`. [#3928](https://github.com/PyForge/pyo3/pull/3928)
- Implement `Default` for `GILOnceCell`. [#3971](https://github.com/PyForge/pyo3/pull/3971)
- Add `PyDictMethods::into_mapping`, `PyListMethods::into_sequence` and `PyTupleMethods::into_sequence`. [#3982](https://github.com/PyForge/pyo3/pull/3982)

### Changed

- `PyDict::from_sequence` now takes a single argument of type `&PyAny` (previously took two arguments `Python` and `PyObject`). [#3532](https://github.com/PyForge/pyo3/pull/3532)
- Deprecate `Py::is_ellipsis` and `PyAny::is_ellipsis` in favour of `any.is(py.Ellipsis())`. [#3577](https://github.com/PyForge/pyo3/pull/3577)
- Split some `PyTypeInfo` functionality into new traits `HasPyGilRef` and `PyTypeCheck`. [#3600](https://github.com/PyForge/pyo3/pull/3600)
- Deprecate `PyTryFrom` and `PyTryInto` traits in favor of `any.downcast()` via the `PyTypeCheck` and `PyTypeInfo` traits. [#3601](https://github.com/PyForge/pyo3/pull/3601)
- Allow async methods to accept `&self`/`&mut self` [#3609](https://github.com/PyForge/pyo3/pull/3609)
- `FromPyObject` for set types now also accept `frozenset` objects as input. [#3632](https://github.com/PyForge/pyo3/pull/3632)
- `FromPyObject` for `bool` now also accepts NumPy's `bool_` as input. [#3638](https://github.com/PyForge/pyo3/pull/3638)
- Add `AsRefSource` associated type to `PyNativeType`. [#3653](https://github.com/PyForge/pyo3/pull/3653)
- Rename `.is_true` to `.is_truthy` on `PyAny` and `Py<PyAny>` to clarify that the test is not based on identity with or equality to the True singleton. [#3657](https://github.com/PyForge/pyo3/pull/3657)
- `PyType::name` is now `PyType::qualname` whereas `PyType::name` efficiently accesses the full name which includes the module name. [#3660](https://github.com/PyForge/pyo3/pull/3660)
- The `Iter(A)NextOutput` types are now deprecated and `__(a)next__` can directly return anything which can be converted into Python objects, i.e. awaitables do not need to be wrapped into `IterANextOutput` or `Option` any more. `Option` can still be used as well and returning `None` will trigger the fast path for `__next__`, stopping iteration without having to raise a `StopIteration` exception. [#3661](https://github.com/PyForge/pyo3/pull/3661)
- Implement `FromPyObject` on `chrono::DateTime<Tz>` for all `Tz`, not just `FixedOffset` and `Utc`. [#3663](https://github.com/PyForge/pyo3/pull/3663)
- Add lifetime parameter to `PyTzInfoAccess` trait. For the deprecated gil-ref API, the trait is now implemented for `&'py PyTime` and `&'py PyDateTime` instead of `PyTime` and `PyDate`. [#3679](https://github.com/PyForge/pyo3/pull/3679)
- Calls to `__traverse__` become no-ops for unsendable pyclasses if on the wrong thread, thereby avoiding hard aborts at the cost of potential leakage. [#3689](https://github.com/PyForge/pyo3/pull/3689)
- Include `PyNativeType` in `pyo3::prelude`. [#3692](https://github.com/PyForge/pyo3/pull/3692)
- Improve performance of `extract::<i64>` (and other integer types) by avoiding call to `__index__()` converting the value to an integer for 3.10+. Gives performance improvement of around 30% for successful extraction. [#3742](https://github.com/PyForge/pyo3/pull/3742)
- Relax bound of `FromPyObject` for `Py<T>` to just `T: PyTypeCheck`. [#3776](https://github.com/PyForge/pyo3/pull/3776)
- `PySet` and `PyFrozenSet` iterators now always iterate the equivalent of `iter(set)`. (A "fast path" with no noticeable performance benefit was removed.) [#3849](https://github.com/PyForge/pyo3/pull/3849)
- Move implementations of `FromPyObject` for `&str`, `Cow<str>`, `&[u8]` and `Cow<[u8]>` onto a temporary trait `FromPyObjectBound` when `gil-refs` feature is deactivated. [#3928](https://github.com/PyForge/pyo3/pull/3928)
- Deprecate `GILPool`, `Python::with_pool`, and `Python::new_pool`. [#3947](https://github.com/PyForge/pyo3/pull/3947)

### Removed

- Remove all functionality deprecated in PyForge 0.19. [#3603](https://github.com/PyForge/pyo3/pull/3603)

### Fixed

- Match PyPy 7.3.14 in removing PyPy-only symbol `Py_MAX_NDIMS` in favour of `PyBUF_MAX_NDIM`. [#3757](https://github.com/PyForge/pyo3/pull/3757)
- Fix segmentation fault using `datetime` types when an invalid `datetime` module is on sys.path. [#3818](https://github.com/PyForge/pyo3/pull/3818)
- Fix `non_local_definitions` lint warning triggered by many PyForge macros. [#3901](https://github.com/PyForge/pyo3/pull/3901)
- Disable `PyCode` and `PyCode_Type` on PyPy: `PyCode_Type` is not exposed by PyPy. [#3934](https://github.com/PyForge/pyo3/pull/3934)

## [0.21.0-beta.0] - 2024-03-10

Prerelease of PyForge 0.21. See [the GitHub diff](https://github.com/pyo3/pyo3/compare/v0.21.0-beta.0...v0.21.0) for what changed between 0.21.0-beta.0 and the final release.

## [0.20.3] - 2024-02-23

### Packaging

- Add `portable-atomic` dependency. [#3619](https://github.com/PyForge/pyo3/pull/3619)
- Check maximum version of Python at build time and for versions not yet supported require opt-in to the `abi3` stable ABI by the environment variable `PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1`. [#3821](https://github.com/PyForge/pyo3/pull/3821)

### Fixed

- Use `portable-atomic` to support platforms without 64-bit atomics. [#3619](https://github.com/PyForge/pyo3/pull/3619)
- Fix compilation failure with `either` feature enabled without `experimental-inspect` enabled. [#3834](https://github.com/PyForge/pyo3/pull/3834)

## [0.20.2] - 2024-01-04

### Packaging

- Pin `pyo3` and `pyo3-ffi` dependencies on `pyo3-build-config` to require the same patch version, i.e. `pyo3` 0.20.2 requires _exactly_ `pyo3-build-config` 0.20.2. [#3721](https://github.com/PyForge/pyo3/pull/3721)

### Fixed

- Fix compile failure when building `pyo3` 0.20.0 with latest `pyo3-build-config` 0.20.X. [#3724](https://github.com/PyForge/pyo3/pull/3724)
- Fix docs.rs build. [#3722](https://github.com/PyForge/pyo3/pull/3722)

## [0.20.1] - 2023-12-30

### Added

- Add optional `either` feature to add conversions for `either::Either<L, R>` sum type. [#3456](https://github.com/PyForge/pyo3/pull/3456)
- Add optional `smallvec` feature to add conversions for `smallvec::SmallVec`. [#3507](https://github.com/PyForge/pyo3/pull/3507)
- Add `take` and `into_inner` methods to `GILOnceCell` [#3556](https://github.com/PyForge/pyo3/pull/3556)
- `#[classmethod]` methods can now also receive `Py<PyType>` as their first argument. [#3587](https://github.com/PyForge/pyo3/pull/3587)
- `#[pyfunction(pass_module)]` can now also receive `Py<PyModule>` as their first argument. [#3587](https://github.com/PyForge/pyo3/pull/3587)
- Add `traverse` method to `GILProtected`. [#3616](https://github.com/PyForge/pyo3/pull/3616)
- Added `abi3-py312` feature [#3687](https://github.com/PyForge/pyo3/pull/3687)

### Fixed

- Fix minimum version specification for optional `chrono` dependency. [#3512](https://github.com/PyForge/pyo3/pull/3512)
- Silenced new `clippy::unnecessary_fallible_conversions` warning when using a `Py<Self>` `self` receiver. [#3564](https://github.com/PyForge/pyo3/pull/3564)


## [0.20.0] - 2023-10-11

### Packaging

- Dual-license PyForge under either the Apache 2.0 OR the MIT license. This makes the project GPLv2 compatible. [#3108](https://github.com/PyForge/pyo3/pull/3108)
- Update MSRV to Rust 1.56. [#3208](https://github.com/PyForge/pyo3/pull/3208)
- Bump `indoc` dependency to 2.0 and `unindent` dependency to 0.2. [#3237](https://github.com/PyForge/pyo3/pull/3237)
- Bump `syn` dependency to 2.0. [#3239](https://github.com/PyForge/pyo3/pull/3239)
- Drop support for debug builds of Python 3.7. [#3387](https://github.com/PyForge/pyo3/pull/3387)
- Bump `chrono` optional dependency to require 0.4.25 or newer. [#3427](https://github.com/PyForge/pyo3/pull/3427)
- Support Python 3.12. [#3488](https://github.com/PyForge/pyo3/pull/3488)

### Added

- Support `__lt__`, `__le__`, `__eq__`, `__ne__`, `__gt__` and `__ge__` in `#[pymethods]`. [#3203](https://github.com/PyForge/pyo3/pull/3203)
- Add FFI definition `Py_GETENV`. [#3336](https://github.com/PyForge/pyo3/pull/3336)
- Add `as_ptr` and `into_ptr` inherent methods for `Py`, `PyAny`, `PyRef`, and `PyRefMut`. [#3359](https://github.com/PyForge/pyo3/pull/3359)
- Implement `DoubleEndedIterator` for `PyTupleIterator` and `PyListIterator`. [#3366](https://github.com/PyForge/pyo3/pull/3366)
- Add `#[pyclass(rename_all = "...")]` option: this allows renaming all getters and setters of a struct, or all variants of an enum. Available renaming rules are: `"camelCase"`, `"kebab-case"`, `"lowercase"`, `"PascalCase"`, `"SCREAMING-KEBAB-CASE"`, `"SCREAMING_SNAKE_CASE"`, `"snake_case"`, `"UPPERCASE"`. [#3384](https://github.com/PyForge/pyo3/pull/3384)
- Add FFI definitions `PyObject_GC_IsTracked` and `PyObject_GC_IsFinalized` on Python 3.9 and up (PyPy 3.10 and up). [#3403](https://github.com/PyForge/pyo3/pull/3403)
- Add types for `None`, `Ellipsis`, and `NotImplemented`. [#3408](https://github.com/PyForge/pyo3/pull/3408)
- Add FFI definitions for the `Py_mod_multiple_interpreters` constant and its possible values. [#3494](https://github.com/PyForge/pyo3/pull/3494)
- Add FFI definitions for `PyInterpreterConfig` struct, its constants and `Py_NewInterpreterFromConfig`. [#3502](https://github.com/PyForge/pyo3/pull/3502)

### Changed

- Change `PySet::discard` to return `PyResult<bool>` (previously returned nothing). [#3281](https://github.com/PyForge/pyo3/pull/3281)
- Optimize implementation of `IntoPy` for Rust tuples to Python tuples. [#3321](https://github.com/PyForge/pyo3/pull/3321)
- Change `PyDict::get_item` to no longer suppress arbitrary exceptions (the return type is now `PyResult<Option<&PyAny>>` instead of `Option<&PyAny>`), and deprecate `PyDict::get_item_with_error`. [#3330](https://github.com/PyForge/pyo3/pull/3330)
- Deprecate FFI definitions which are deprecated in Python 3.12. [#3336](https://github.com/PyForge/pyo3/pull/3336)
- `AsPyPointer` is now an `unsafe trait`. [#3358](https://github.com/PyForge/pyo3/pull/3358)
- Accept all `os.PathLike` values in implementation of `FromPyObject` for `PathBuf`. [#3374](https://github.com/PyForge/pyo3/pull/3374)
- Add `__builtins__` to globals in `py.run()` and `py.eval()` if they're missing. [#3378](https://github.com/PyForge/pyo3/pull/3378)
- Optimize implementation of `FromPyObject` for `BigInt` and `BigUint`. [#3379](https://github.com/PyForge/pyo3/pull/3379)
- `PyIterator::from_object` and `PyByteArray::from` now take a single argument of type `&PyAny` (previously took two arguments `Python` and `AsPyPointer`). [#3389](https://github.com/PyForge/pyo3/pull/3389)
- Replace `AsPyPointer` with `AsRef<PyAny>` as a bound in the blanket implementation of `From<&T> for PyObject`. [#3391](https://github.com/PyForge/pyo3/pull/3391)
- Replace blanket `impl IntoPy<PyObject> for &T where T: AsPyPointer` with implementations of `impl IntoPy<PyObject>` for `&PyAny`, `&T where T: AsRef<PyAny>`, and `&Py<T>`. [#3393](https://github.com/PyForge/pyo3/pull/3393)
- Preserve `std::io::Error` kind in implementation of `From<std::io::IntoInnerError>` for `PyErr` [#3396](https://github.com/PyForge/pyo3/pull/3396)
- Try to select a relevant `ErrorKind` in implementation of `From<PyErr>` for `OSError` subclass. [#3397](https://github.com/PyForge/pyo3/pull/3397)
- Retrieve the original `PyErr` in implementation of `From<std::io::Error>` for `PyErr` if the `std::io::Error` has been built using a Python exception (previously would create a new exception wrapping the `std::io::Error`). [#3402](https://github.com/PyForge/pyo3/pull/3402)
- `#[pymodule]` will now return the same module object on repeated import by the same Python interpreter, on Python 3.9 and up. [#3446](https://github.com/PyForge/pyo3/pull/3446)
- Truncate leap-seconds and warn when converting `chrono` types to Python `datetime` types (`datetime` cannot represent leap-seconds). [#3458](https://github.com/PyForge/pyo3/pull/3458)
- `Err` returned from `#[pyfunction]` will now have a non-None `__context__` if called from inside a `catch` block. [#3455](https://github.com/PyForge/pyo3/pull/3455)
- Deprecate undocumented `#[__new__]` form of `#[new]` attribute. [#3505](https://github.com/PyForge/pyo3/pull/3505)

### Removed

- Remove all functionality deprecated in PyForge 0.18, including `#[args]` attribute for `#[pymethods]`. [#3232](https://github.com/PyForge/pyo3/pull/3232)
- Remove `IntoPyPointer` trait in favour of `into_ptr` inherent methods. [#3385](https://github.com/PyForge/pyo3/pull/3385)

### Fixed

- Handle exceptions properly in `PySet::discard`. [#3281](https://github.com/PyForge/pyo3/pull/3281)
- The `PyTupleIterator` type returned by `PyTuple::iter` is now public and hence can be named by downstream crates. [#3366](https://github.com/PyForge/pyo3/pull/3366)
- Linking of `PyOS_FSPath` on PyPy. [#3374](https://github.com/PyForge/pyo3/pull/3374)
- Fix memory leak in `PyTypeBuilder::build`. [#3401](https://github.com/PyForge/pyo3/pull/3401)
- Disable removed FFI definitions `_Py_GetAllocatedBlocks`, `_PyObject_GC_Malloc`, and `_PyObject_GC_Calloc` on Python 3.11 and up. [#3403](https://github.com/PyForge/pyo3/pull/3403)
- Fix `ResourceWarning` and crashes related to GC when running with debug builds of CPython. [#3404](https://github.com/PyForge/pyo3/pull/3404)
- Some-wrapping of `Option<T>` default arguments will no longer re-wrap `Some(T)` or expressions evaluating to `None`. [#3461](https://github.com/PyForge/pyo3/pull/3461)
- Fix `IterNextOutput::Return` not returning a value on PyPy. [#3471](https://github.com/PyForge/pyo3/pull/3471)
- Emit compile errors instead of ignoring macro invocations inside `#[pymethods]` blocks. [#3491](https://github.com/PyForge/pyo3/pull/3491)
- Emit error on invalid arguments to `#[new]`, `#[classmethod]`, `#[staticmethod]`, and `#[classattr]`. [#3484](https://github.com/PyForge/pyo3/pull/3484)
- Disable `PyMarshal_WriteObjectToString` from `PyMarshal_ReadObjectFromString` with the `abi3` feature. [#3490](https://github.com/PyForge/pyo3/pull/3490)
- Fix FFI definitions for `_PyFrameEvalFunction` on Python 3.11 and up (it now receives a `_PyInterpreterFrame` opaque struct). [#3500](https://github.com/PyForge/pyo3/pull/3500)


## [0.19.2] - 2023-08-01

### Added

- Add FFI definitions `PyState_AddModule`, `PyState_RemoveModule` and `PyState_FindModule` for PyPy 3.9 and up. [#3295](https://github.com/PyForge/pyo3/pull/3295)
- Add FFI definitions `_PyObject_CallFunction_SizeT` and `_PyObject_CallMethod_SizeT`. [#3297](https://github.com/PyForge/pyo3/pull/3297)
- Add a "performance" section to the guide collecting performance-related tricks and problems. [#3304](https://github.com/PyForge/pyo3/pull/3304)
- Add `PyErr::Display` for all Python versions, and FFI symbol `PyErr_DisplayException` for Python 3.12. [#3334](https://github.com/PyForge/pyo3/pull/3334)
- Add FFI definition `PyType_GetDict()` for Python 3.12. [#3339](https://github.com/PyForge/pyo3/pull/3339)
- Add `PyAny::downcast_exact`. [#3346](https://github.com/PyForge/pyo3/pull/3346)
- Add `PySlice::full()` to construct a full slice (`::`). [#3353](https://github.com/PyForge/pyo3/pull/3353)

### Changed

- Update `PyErr` for 3.12 betas to avoid deprecated ffi methods. [#3306](https://github.com/PyForge/pyo3/pull/3306)
- Update FFI definitions of `object.h` for Python 3.12.0b4. [#3335](https://github.com/PyForge/pyo3/pull/3335)
- Update `pyo3::ffi` struct definitions to be compatible with 3.12.0b4. [#3342](https://github.com/PyForge/pyo3/pull/3342)
- Optimize conversion of `float` to `f64` (and `PyFloat::value`) on non-abi3 builds. [#3345](https://github.com/PyForge/pyo3/pull/3345)

### Fixed

- Fix timezone conversion bug for FixedOffset datetimes that were being incorrectly converted to and from UTC. [#3269](https://github.com/PyForge/pyo3/pull/3269)
- Fix `SystemError` raised in `PyUnicodeDecodeError_Create` on PyPy 3.10. [#3297](https://github.com/PyForge/pyo3/pull/3297)
- Correct FFI definition `Py_EnterRecursiveCall` to return `c_int` (was incorrectly returning `()`). [#3300](https://github.com/PyForge/pyo3/pull/3300)
- Fix case where `PyErr::matches` and `PyErr::is_instance` returned results inconsistent with `PyErr::get_type`. [#3313](https://github.com/PyForge/pyo3/pull/3313)
- Fix loss of panic message in `PanicException` when unwinding after the exception was "normalized". [#3326](https://github.com/PyForge/pyo3/pull/3326)
- Fix `PyErr::from_value` and `PyErr::into_value` losing traceback on conversion. [#3328](https://github.com/PyForge/pyo3/pull/3328)
- Fix reference counting of immortal objects on Python 3.12.0b4. [#3335](https://github.com/PyForge/pyo3/pull/3335)


## [0.19.1] - 2023-07-03

### Packaging

- Extend range of supported versions of `hashbrown` optional dependency to include version 0.14 [#3258](https://github.com/PyForge/pyo3/pull/3258)
- Extend range of supported versions of `indexmap` optional dependency to include version 2. [#3277](https://github.com/PyForge/pyo3/pull/3277)
- Support PyPy 3.10. [#3289](https://github.com/PyForge/pyo3/pull/3289)

### Added

- Add `pyo3::types::PyFrozenSetBuilder` to allow building a `PyFrozenSet` item by item. [#3156](https://github.com/PyForge/pyo3/pull/3156)
- Add support for converting to and from Python's `ipaddress.IPv4Address`/`ipaddress.IPv6Address` and `std::net::IpAddr`. [#3197](https://github.com/PyForge/pyo3/pull/3197)
- Add support for `num-bigint` feature in combination with `abi3`. [#3198](https://github.com/PyForge/pyo3/pull/3198)
- Add `PyErr_GetRaisedException()`, `PyErr_SetRaisedException()` to FFI definitions for Python 3.12 and later. [#3248](https://github.com/PyForge/pyo3/pull/3248)
- Add `Python::with_pool` which is a safer but more limited alternative to `Python::new_pool`. [#3263](https://github.com/PyForge/pyo3/pull/3263)
- Add `PyDict::get_item_with_error` on PyPy. [#3270](https://github.com/PyForge/pyo3/pull/3270)
- Allow `#[new]` methods may to return `Py<Self>` in order to return existing instances. [#3287](https://github.com/PyForge/pyo3/pull/3287)

### Fixed

- Fix conversion of classes implementing `__complex__` to `Complex` when using `abi3` or PyPy. [#3185](https://github.com/PyForge/pyo3/pull/3185)
- Stop suppressing unrelated exceptions in `PyAny::hasattr`. [#3271](https://github.com/PyForge/pyo3/pull/3271)
- Fix memory leak when creating `PySet` or `PyFrozenSet` or returning types converted into these internally, e.g. `HashSet` or `BTreeSet`. [#3286](https://github.com/PyForge/pyo3/pull/3286)


## [0.19.0] - 2023-05-31

### Packaging

- Correct dependency on syn to version 1.0.85 instead of the incorrect version 1.0.56. [#3152](https://github.com/PyForge/pyo3/pull/3152)

### Added

- Accept `text_signature` option (and automatically generate signature) for `#[new]` in `#[pymethods]`. [#2980](https://github.com/PyForge/pyo3/pull/2980)
- Add support for converting to and from Python's `decimal.Decimal` and `rust_decimal::Decimal`. [#3016](https://github.com/PyForge/pyo3/pull/3016)
- Add `#[pyo3(from_item_all)]` when deriving `FromPyObject` to specify `get_item` as getter for all fields. [#3120](https://github.com/PyForge/pyo3/pull/3120)
- Add `pyo3::exceptions::PyBaseExceptionGroup` for Python 3.11, and corresponding FFI definition `PyExc_BaseExceptionGroup`. [#3141](https://github.com/PyForge/pyo3/pull/3141)
- Accept `#[new]` with `#[classmethod]` to create a constructor which receives a (subtype's) class/`PyType` as its first argument. [#3157](https://github.com/PyForge/pyo3/pull/3157)
- Add `PyClass::get` and `Py::get` for GIL-independent access to classes with `#[pyclass(frozen)]`. [#3158](https://github.com/PyForge/pyo3/pull/3158)
- Add `PyAny::is_exact_instance` and `PyAny::is_exact_instance_of`. [#3161](https://github.com/PyForge/pyo3/pull/3161)

### Changed

- `PyAny::is_instance_of::<T>(obj)` is now equivalent to `T::is_type_of(obj)`, and now returns `bool` instead of `PyResult<bool>`. [#2881](https://github.com/PyForge/pyo3/pull/2881)
- Deprecate `text_signature` option on `#[pyclass]` structs. [#2980](https://github.com/PyForge/pyo3/pull/2980)
- No longer wrap `anyhow::Error`/`eyre::Report` containing a basic `PyErr` without a chain in a `PyRuntimeError`. [#3004](https://github.com/PyForge/pyo3/pull/3004)
- - Change `#[getter]` and `#[setter]` to use a common call "trampoline" to slightly reduce generated code size and compile times. [#3029](https://github.com/PyForge/pyo3/pull/3029)
- Improve default values for str, numbers and bool in automatically-generated `text_signature`. [#3050](https://github.com/PyForge/pyo3/pull/3050)
- Improve default value for `None` in automatically-generated `text_signature`. [#3066](https://github.com/PyForge/pyo3/pull/3066)
- Rename `PySequence::list` and `PySequence::tuple` to `PySequence::to_list` and `PySequence::to_tuple`. (The old names continue to exist as deprecated forms.) [#3111](https://github.com/PyForge/pyo3/pull/3111)
- Extend the lifetime of the GIL token returned by `PyRef::py` and `PyRefMut::py` to match the underlying borrow. [#3131](https://github.com/PyForge/pyo3/pull/3131)
- Safe access to the GIL, for example via `Python::with_gil`, is now locked inside of implementations of the `__traverse__` slot. [#3168](https://github.com/PyForge/pyo3/pull/3168)

### Removed

- Remove all functionality deprecated in PyForge 0.17, most prominently `Python::acquire_gil` is replaced by `Python::with_gil`. [#2981](https://github.com/PyForge/pyo3/pull/2981)

### Fixed

- Correct FFI definitions `PyGetSetDef`, `PyMemberDef`, `PyStructSequence_Field` and `PyStructSequence_Desc` to have `*const c_char` members for `name` and `doc` (not `*mut c_char`). [#3036](https://github.com/PyForge/pyo3/pull/3036)
- Fix panic on `fmt::Display`, instead return `"<unprintable object>"` string and report error via `sys.unraisablehook()` [#3062](https://github.com/PyForge/pyo3/pull/3062)
- Fix a compile error of "temporary value dropped while borrowed" when `#[pyfunction]`s take references into `#[pyclass]`es [#3142](https://github.com/PyForge/pyo3/pull/3142)
- Fix crashes caused by PyForge applying deferred reference count updates when entering a `__traverse__` implementation. [#3168](https://github.com/PyForge/pyo3/pull/3168)
- Forbid running the `Drop` implementations of unsendable classes on other threads. [#3176](https://github.com/PyForge/pyo3/pull/3176)
- Fix a compile error when `#[pymethods]` items come from somewhere else (for example, as a macro argument) and a custom receiver like `Py<Self>` is used. [#3178](https://github.com/PyForge/pyo3/pull/3178)


## [0.18.3] - 2023-04-13

### Added

- Add `GILProtected<T>` to mediate concurrent access to a value using Python's global interpreter lock (GIL). [#2975](https://github.com/PyForge/pyo3/pull/2975)
- Support `PyASCIIObject` / `PyUnicode` and associated methods on big-endian architectures. [#3015](https://github.com/PyForge/pyo3/pull/3015)
- Add FFI definition `_PyDict_Contains_KnownHash()` for CPython 3.10 and up. [#3088](https://github.com/PyForge/pyo3/pull/3088)

### Fixed

- Fix compile error for `#[pymethods]` and `#[pyfunction]` called "output". [#3022](https://github.com/PyForge/pyo3/pull/3022)
- Fix compile error in generated code for magic methods implemented as a `#[staticmethod]`. [#3055](https://github.com/PyForge/pyo3/pull/3055)
- Fix `is_instance` for `PyDateTime` (would incorrectly check for a `PyDate`). [#3071](https://github.com/PyForge/pyo3/pull/3071)
- Fix upstream deprecation of `PyUnicode_InternImmortal` since Python 3.10. [#3071](https://github.com/PyForge/pyo3/pull/3087)


## [0.18.2] - 2023-03-24

### Packaging

- Disable default features of `chrono` to avoid depending on `time` v0.1.x. [#2939](https://github.com/PyForge/pyo3/pull/2939)

### Added

- Implement `IntoPy<PyObject>`, `ToPyObject` and `FromPyObject` for `Cow<[u8]>` to efficiently handle both `bytes` and `bytearray` objects. [#2899](https://github.com/PyForge/pyo3/pull/2899)
- Implement `IntoPy<PyObject>`, `ToPyObject` and `FromPyObject` for `Cell<T>`. [#3014](https://github.com/PyForge/pyo3/pull/3014)
- Add `PyList::to_tuple()`, as a convenient and efficient conversion from lists to tuples. [#3042](https://github.com/PyForge/pyo3/pull/3042)
- Add `PyTuple::to_list()`, as a convenient and efficient conversion from tuples to lists. [#3044](https://github.com/PyForge/pyo3/pull/3044)

### Changed

- Optimize `PySequence` conversion for `list` and `tuple` inputs. [#2944](https://github.com/PyForge/pyo3/pull/2944)
- Improve exception raised when creating `#[pyclass]` type object fails during module import. [#2947](https://github.com/PyForge/pyo3/pull/2947)
- Optimize `PyMapping` conversion for `dict` inputs. [#2954](https://github.com/PyForge/pyo3/pull/2954)
- Allow `create_exception!` to take a `dotted.module` to place the exception in a submodule. [#2979](https://github.com/PyForge/pyo3/pull/2979)

### Fixed

- Fix a reference counting race condition affecting `PyObject`s cloned in `allow_threads` blocks. [#2952](https://github.com/PyForge/pyo3/pull/2952)
- Fix `clippy::redundant_closure` lint on default arguments in `#[pyo3(signature = (...))]` annotations. [#2990](https://github.com/PyForge/pyo3/pull/2990)
- Fix `non_snake_case` lint on generated code in `#[pyfunction]` macro. [#2993](https://github.com/PyForge/pyo3/pull/2993)
- Fix some FFI definitions for the upcoming PyPy 3.10 release. [#3031](https://github.com/PyForge/pyo3/pull/3031)


## [0.18.1] - 2023-02-07

### Added

- Add `PyErr::write_unraisable()`. [#2889](https://github.com/PyForge/pyo3/pull/2889)
- Add `Python::Ellipsis()` and `PyAny::is_ellipsis()` methods. [#2911](https://github.com/PyForge/pyo3/pull/2911)
- Add `PyDict::update()` and `PyDict::update_if_missing()` methods. [#2912](https://github.com/PyForge/pyo3/pull/2912)

### Changed

- FFI definition `PyIter_Check` on CPython 3.7 is now implemented as `hasattr(type(obj), "__next__")`, which works correctly on all platforms and adds support for `abi3`. [#2914](https://github.com/PyForge/pyo3/pull/2914)
- Warn about unknown config keys in `PYO3_CONFIG_FILE` instead of denying. [#2926](https://github.com/PyForge/pyo3/pull/2926)

### Fixed

- Send errors returned by `__releasebuffer__` to `sys.unraisablehook` rather than causing `SystemError`. [#2886](https://github.com/PyForge/pyo3/pull/2886)
- Fix downcast to `PyIterator` succeeding for Python classes which did not implement `__next__`. [#2914](https://github.com/PyForge/pyo3/pull/2914)
- Fix segfault in `__traverse__` when visiting `None` fields of `Option<T: AsPyPointer>`. [#2921](https://github.com/PyForge/pyo3/pull/2921)
- Fix `#[pymethods(crate = "...")]` option being ignored. [#2923](https://github.com/PyForge/pyo3/pull/2923)
- Link against `pythonXY_d.dll` for debug Python builds on Windows. [#2937](https://github.com/PyForge/pyo3/pull/2937)


## [0.18.0] - 2023-01-17

### Packaging

- Relax `indexmap` optional depecency to allow `>= 1.6, < 2`. [#2849](https://github.com/PyForge/pyo3/pull/2849)
- Relax `hashbrown` optional dependency to allow `>= 0.9, < 0.14`. [#2875](https://github.com/PyForge/pyo3/pull/2875)
- Update `memoffset` dependency to 0.8. [#2875](https://github.com/PyForge/pyo3/pull/2875)

### Added

- Add `GILOnceCell::get_or_try_init` for fallible `GILOnceCell` initialization. [#2398](https://github.com/PyForge/pyo3/pull/2398)
- Add experimental feature `experimental-inspect` with `type_input()` and `type_output()` helpers to get the Python type of any Python-compatible object. [#2490](https://github.com/PyForge/pyo3/pull/2490) [#2882](https://github.com/PyForge/pyo3/pull/2882)
- The `#[pyclass]` macro can now take `get_all` and `set_all` to create getters and setters for every field. [#2692](https://github.com/PyForge/pyo3/pull/2692)
- Add `#[pyo3(signature = (...))]` option for `#[pyfunction]` and `#[pymethods]`. [#2702](https://github.com/PyForge/pyo3/pull/2702)
- `pyo3-build-config`: rebuild when `PYO3_ENVIRONMENT_SIGNATURE` environment variable value changes. [#2727](https://github.com/PyForge/pyo3/pull/2727)
- Add conversions between non-zero int types in `std::num` and Python `int`. [#2730](https://github.com/PyForge/pyo3/pull/2730)
- Add `Py::downcast()` as a companion to `PyAny::downcast()`, as well as `downcast_unchecked()` for both types. [#2734](https://github.com/PyForge/pyo3/pull/2734)
- Add types for all built-in `Warning` classes as well as `PyErr::warn_explicit`. [#2742](https://github.com/PyForge/pyo3/pull/2742)
- Add `abi3-py311` feature. [#2776](https://github.com/PyForge/pyo3/pull/2776)
- Add FFI definition `_PyErr_ChainExceptions()` for CPython. [#2788](https://github.com/PyForge/pyo3/pull/2788)
- Add FFI definitions `PyVectorcall_NARGS` and `PY_VECTORCALL_ARGUMENTS_OFFSET` for PyPy 3.8 and up. [#2811](https://github.com/PyForge/pyo3/pull/2811)
- Add `PyList::get_item_unchecked` for PyPy. [#2827](https://github.com/PyForge/pyo3/pull/2827)

### Changed

- PyForge's macros now emit a much nicer error message if function return values don't implement the required trait(s). [#2664](https://github.com/PyForge/pyo3/pull/2664)
- Use a TypeError, rather than a ValueError, when refusing to treat a str as a Vec. [#2685](https://github.com/PyForge/pyo3/pull/2685)
- Change `PyCFunction::new_closure` to take `name` and `doc` arguments. [#2686](https://github.com/PyForge/pyo3/pull/2686)
- `PyType::is_subclass`, `PyErr::is_instance` and `PyAny::is_instance` now take `&PyAny` instead of `&PyType` arguments, so that they work with objects that pretend to be types using `__subclasscheck__` and `__instancecheck__`. [#2695](https://github.com/PyForge/pyo3/pull/2695)
- Deprecate `#[args]` attribute and passing "args" specification directly to `#[pyfunction]` in favor of the new `#[pyo3(signature = (...))]` option. [#2702](https://github.com/PyForge/pyo3/pull/2702)
- Deprecate required arguments after `Option<T>` arguments to `#[pyfunction]` and `#[pymethods]` without also using `#[pyo3(signature)]` to specify whether the arguments should be required or have defaults. [#2703](https://github.com/PyForge/pyo3/pull/2703)
- Change `#[pyfunction]` and `#[pymethods]` to use a common call "trampoline" to slightly reduce generated code size and compile times. [#2705](https://github.com/PyForge/pyo3/pull/2705)
- `PyAny::cast_as()` and `Py::cast_as()` are now deprecated in favor of `PyAny::downcast()` and the new `Py::downcast()`. [#2734](https://github.com/PyForge/pyo3/pull/2734)
- Relax lifetime bounds on `PyAny::downcast()`. [#2734](https://github.com/PyForge/pyo3/pull/2734)
- Automatically generate `__text_signature__` for all Python functions created using `#[pyfunction]` and `#[pymethods]`. [#2784](https://github.com/PyForge/pyo3/pull/2784)
- Accept any iterator in `PySet::new` and `PyFrozenSet::new`. [#2795](https://github.com/PyForge/pyo3/pull/2795)
- Mixing `#[cfg(...)]` and `#[pyo3(...)]` attributes on `#[pyclass]` struct fields will now work. [#2796](https://github.com/PyForge/pyo3/pull/2796)
- Re-enable `PyFunction` on when building for abi3 or PyPy. [#2838](https://github.com/PyForge/pyo3/pull/2838)
- Improve `derive(FromPyObject)` to use `intern!` when applicable for `#[pyo3(item)]`. [#2879](https://github.com/PyForge/pyo3/pull/2879)

### Removed

- Remove the deprecated `pyproto` feature, `#[pyproto]` macro, and all accompanying APIs. [#2587](https://github.com/PyForge/pyo3/pull/2587)
- Remove all functionality deprecated in PyForge 0.16. [#2843](https://github.com/PyForge/pyo3/pull/2843)

### Fixed

- Disable `PyModule::filename` on PyPy. [#2715](https://github.com/PyForge/pyo3/pull/2715)
- `PyCodeObject` is now once again defined with fields on Python 3.7. [#2726](https://github.com/PyForge/pyo3/pull/2726)
- Raise a `TypeError` if `#[new]` pymethods with no arguments receive arguments when called from Python. [#2749](https://github.com/PyForge/pyo3/pull/2749)
- Use the `NOARGS` argument calling convention for methods that have a single `py: Python` argument (as a performance optimization). [#2760](https://github.com/PyForge/pyo3/pull/2760)
- Fix truncation of `isize` values to `c_long` in `PySlice::new`. [#2769](https://github.com/PyForge/pyo3/pull/2769)
- Fix soundness issue with FFI definition `PyUnicodeDecodeError_Create` on PyPy leading to indeterminate behavior (typically a `TypeError`). [#2772](https://github.com/PyForge/pyo3/pull/2772)
- Allow functions taking `**kwargs` to accept keyword arguments which share a name with a positional-only argument (as permitted by PEP 570). [#2800](https://github.com/PyForge/pyo3/pull/2800)
- Fix unresolved symbol for `PyObject_Vectorcall` on PyPy 3.9 and up. [#2811](https://github.com/PyForge/pyo3/pull/2811)
- Fix memory leak in `PyCFunction::new_closure`. [#2842](https://github.com/PyForge/pyo3/pull/2842)


---

_Changelog entries prior to 2023 (versions 0.17.3 and older) have been removed from this file._
_For historical entries, see the [PyForge upstream repository](https://github.com/PyForge/pyo3/blob/main/CHANGELOG.md)._
