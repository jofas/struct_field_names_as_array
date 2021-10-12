#![allow(dead_code)]

use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
struct Test {
  f1: String,
  f2: i64,
  f3: String,
  f4: bool,
}

#[derive(FieldNamesAsArray)]
struct TestGenerics<A, B, C> {
  foo: A,
  bar: B,
  baz: C,
}

#[derive(FieldNamesAsArray)]
struct TestSkip {
  a: String,
  b: String,
  #[field_names_as_array(skip)]
  c: String,
}

#[test]
fn test_struct() {
  assert_eq!(Test::FIELD_NAMES_AS_ARRAY, ["f1", "f2", "f3", "f4"]);
}

#[test]
fn test_generics_struct() {
  assert_eq!(
    TestGenerics::<u8, u8, u8>::FIELD_NAMES_AS_ARRAY,
    ["foo", "bar", "baz"],
  );
}

#[test]
fn test_skip() {
  assert_eq!(TestSkip::FIELD_NAMES_AS_ARRAY, ["a", "b"]);
}
