# struct-field-names-as-array

[![Build Status](https://github.com/jofas/struct_field_names_as_array/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/struct_field_names_as_array/actions/workflows/build.yml)
[![Codecov](https://codecov.io/gh/jofas/struct_field_names_as_array/branch/master/graph/badge.svg?token=69YKZ1JIBK)](https://codecov.io/gh/jofas/struct_field_names_as_array)
[![Latest Version](https://img.shields.io/crates/v/struct-field-names-as-array.svg)](https://crates.io/crates/struct-field-names-as-array)
[![Downloads](https://img.shields.io/crates/d/struct-field-names-as-array?label=downloads)](https://crates.io/crates/struct-field-names-as-array)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/struct-field-names-as-array/latest/struct_field_names_as_array)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Provides the `FieldNamesAsArray` procedural macro.
`FieldNamesAsArray` lets you generate an array of the field names of
a named struct.

The procedural macro adds the `FIELD_NAMES_AS_ARRAY` constant to
the struct.
The `FIELD_NAMES_AS_ARRAY` is the array containing the field names
of the struct.

**Note:** the macro can only be derived by named structs.


## Table of Contents

<!--ts-->
   * [Usage](#usage)
   * [Attributes](#attributes)
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
`field_names_as_array` attribute with the following possible
arguments:

* `skip`: do not add the field to `FIELD_NAMES_AS_ARRAY`:

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


### Visibility

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
