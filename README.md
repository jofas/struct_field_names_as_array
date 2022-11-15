# struct-field-names-as-array

[![Build Status](https://github.com/jofas/struct_field_names_as_array/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/struct_field_names_as_array/actions/workflows/build.yml)
[![Codecov](https://codecov.io/gh/jofas/struct_field_names_as_array/branch/master/graph/badge.svg?token=69YKZ1JIBK)](https://codecov.io/gh/jofas/struct_field_names_as_array)
[![Latest Version](https://img.shields.io/crates/v/struct-field-names-as-array.svg)](https://crates.io/crates/struct-field-names-as-array)
[![Downloads](https://img.shields.io/crates/d/struct-field-names-as-array?label=downloads)](https://crates.io/crates/struct-field-names-as-array)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/struct-field-names-as-array/latest/struct_field_names_as_array)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Provides the `FieldNamesAsArray` procedural macro.
The macro adds the `FIELD_NAMES_AS_ARRAY` constant to the struct the
macro is dervied on.
The `FIELD_NAMES_AS_ARRAY` contains the field names of the given 
struct.

**Note:** The macro can only be derived from named structs.

## Table of Contents

<!--ts-->
   * [Usage](#usage)
   * [Attributes](#attributes)
      * [Container Attributes](#container-attributes)
         * [Rename all](#rename-all)
      * [Field Attributes](#field-attributes)
         * [Skip](#skip)
   * [Visibility](#visibility)
<!--te-->

## Usage

You can derive the `FieldNamesAsArray` macro like this:

```rust
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
struct Foo {
  bar: String,
  baz: String,
  bat: String,
}

assert_eq!(Foo::FIELD_NAMES_AS_ARRAY, ["bar", "baz", "bat"]);
```

## Attributes

The `FieldNamesAsArray` macro supports the
`field_names_as_array` attribute.
`field_names_as_array` can be applied to the container or to a field
with different arguments listed below.

### Container Attributes

Container attributes are global attributes that change the behavior
of the whole field names array, rather than that of a single field.

#### Rename all

The `rename_all` attribute renames every field of the struct according
to the provided naming convention.
This attribute works exactly like the [serde][serde_rename_all]
equivalent.
Supported are these naming conventions:
  - `lowercase`
  - `UPPERCASE`
  - `PascalCase`
  - `camelCase`
  - `snake_case`
  - `SCREAMING_SNAKE_CASE`
  - `kebab-case`
  - `SCREAMING-KEBAB-CASE`

```rust
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
#[field_names_as_array(rename_all = "SCREAMING-KEBAB-CASE")]
struct Foo {
  field_one: String,
  field_two: String,
  field_three: String,
}

assert_eq!(
  Foo::FIELD_NAMES_AS_ARRAY, 
  ["FIELD-ONE", "FIELD-TWO", "FIELD-THREE"],
);
```

**Note:** Same as serde's implementation of `rename_all`, it is
assumed that your field names follow the rust naming convention, that 
all field names must be given in `snake_case`.
If not, applying `rename_all` may result in unexpected field names.

### Field Attributes

Field attributes can be added to the fields of a named struct and 
change the behavior of a single field.

#### Skip

The `skip` attribute removes the field from `FIELD_NAMES_AS_ARRAY`.

```rust
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
struct Foo {
  bar: String,
  baz: String,
  #[field_names_as_array(skip)]
  bat: String,
}

assert_eq!(Foo::FIELD_NAMES_AS_ARRAY, ["bar", "baz"]);
```

## Visibility

The visibility of the `FIELD_NAMES_AS_ARRAY` is the same as the
corresponding struct.
E.g. is it `pub struct Foo { ... }`, the `FIELD_NAMES_AS_ARRAY`
will be public as well.
This, for example, will work:

```rust
mod foo {
  use struct_field_names_as_array::FieldNamesAsArray;

  #[derive(FieldNamesAsArray)]
  pub(super) struct Foo {
    bar: String,
    baz: String,
    bat: String,
  }
}

assert_eq!(foo::Foo::FIELD_NAMES_AS_ARRAY, ["bar", "baz", "bat"]);
```

Whereas this will not, since `FIELD_NAMES_AS_ARRAY` is private:

```compile_fail
mod foo {
  use struct_field_names_as_array::FieldNamesAsArray;

  #[derive(FieldNamesAsArray)]
  struct Foo {
    bar: String,
    baz: String,
    bat: String,
  }
}

assert_eq!(foo::Foo::FIELD_NAMES_AS_ARRAY, ["bar", "baz", "bat"]);
```

[serde_rename_all]: https://serde.rs/container-attrs.html#rename_all
