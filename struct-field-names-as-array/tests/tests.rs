#![cfg(feature = "derive")]
#![allow(dead_code)]

use struct_field_names_as_array::{FieldNamesAsArray, FieldNamesAsSlice};

#[derive(FieldNamesAsArray, FieldNamesAsSlice)]
struct Test {
    f1: String,
    f2: i64,
    f3: String,
    f4: bool,
}

#[derive(FieldNamesAsArray, FieldNamesAsSlice)]
struct TestGenerics<A, B, C> {
    foo: A,
    bar: B,
    baz: C,
}

#[derive(FieldNamesAsArray, FieldNamesAsSlice)]
struct TestSkip {
    a: String,
    b: String,
    #[field_names_as_array(skip)]
    #[field_names_as_slice(skip)]
    c: String,
}
#[derive(FieldNamesAsArray, FieldNamesAsSlice)]
struct TestRename {
    a: String,
    b: String,
    #[field_names_as_array(rename = "last_option")]
    #[field_names_as_slice(rename = "last_option")]
    c: String,
}

#[test]
fn test_struct() {
    assert_eq!(Test::FIELD_NAMES_AS_ARRAY, ["f1", "f2", "f3", "f4"]);
    assert_eq!(Test::FIELD_NAMES_AS_SLICE, ["f1", "f2", "f3", "f4"]);
}

#[test]
fn test_generics_struct() {
    assert_eq!(
        TestGenerics::<u8, u8, u8>::FIELD_NAMES_AS_ARRAY,
        ["foo", "bar", "baz"],
    );
    assert_eq!(
        TestGenerics::<u8, u8, u8>::FIELD_NAMES_AS_SLICE,
        ["foo", "bar", "baz"],
    );
}

#[test]
fn test_skip() {
    assert_eq!(TestSkip::FIELD_NAMES_AS_ARRAY, ["a", "b"]);
    assert_eq!(TestSkip::FIELD_NAMES_AS_SLICE, ["a", "b"]);
}

#[test]
fn test_rename() {
    assert_eq!(TestRename::FIELD_NAMES_AS_ARRAY, ["a", "b", "last_option"]);
    assert_eq!(TestRename::FIELD_NAMES_AS_SLICE, ["a", "b", "last_option"]);
}
