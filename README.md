# struct-field-names-as-array

Provides the `FieldNamesAsArray` procedural macro.

The procedural macro adds the ``FIELD_NAMES_AS_ARRAY`` constant to
the struct.
The `FIELD_NAMES_AS_ARRAY` is an array containing the field names
of the struct (as the name suggests).
The visibility of the `FIELD_NAMES_AS_ARRAY` is the same as the
corresponding struct.

**NOTE:** the macro can only be derived by named structs.

### Example

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

* `skip`: do not add the field to `FIELD_NAMES_AS_ARRAY`

  **Example**

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
