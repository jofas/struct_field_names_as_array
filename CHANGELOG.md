# CHANGELOG

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0]

### Added

* `struct-field-names-as-array-derive` crate

* `FieldNamesAsArray` trait

* `FieldNamesAsSlice` trait

* `FieldNamesAsSlice` procedural macro

### Changed

* upgraded rust edition from 2018 to 2021

* `struct-field-names-as-array-derive`: `syn v1 -> v2`

## [0.2.0]

### Added

* `visibility` container attribute

### Changed

* default visibility of `FIELD_NAMES_AS_ARRAY` now private

* `FIELD_NAMES_AS_ARRAY` is now an array, not a slice


## [0.1.4]

### Added

* `rename_all` container attribute


## [0.1.3]

### Added

* documentation for the generated `FIELD_NAMES_AS_ARRAY` constant


## [0.1.2]

### Added

* `field_names_as_array(skip)` attribute to `FieldNamesAsArray`


## [0.1.1]

### Changed

* downgraded rust edition from `2021` to `2018`


## [0.1.0]

### Added

* `FieldNamesAsArray` procedural macro
