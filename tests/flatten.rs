#![allow(dead_code)]

use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
struct FlattenSimple {
  a: bool,
  b: bool,
  #[field_names_as_array(flatten)]
  c: Flattened,
}

#[derive(FieldNamesAsArray)]
struct FlattenDelim {
  a: bool,
  b: bool,
  #[field_names_as_array(flatten(delim = "/"))]
  c: Flattened,
}

#[derive(FieldNamesAsArray)]
struct Flattened {
  #[field_names_as_array(skip)]
  w: u8,
  x: u8,
  y: u8,
  z: u8,
}

/*
#[test]
#[ignore]
fn test_flatten_simple() {
  assert_eq!(
    FlattenSimple::FIELD_NAMES_AS_ARRAY,
    ["a", "b", "c.x", "c.y", "c.z"],
  );
}

#[test]
#[ignore]
fn test_flatten_delim() {
  assert_eq!(
    FlattenDelim::FIELD_NAMES_AS_ARRAY,
    ["a", "b", "c/x", "c/y", "c/z"],
  );
}
*/
