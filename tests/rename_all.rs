#![allow(dead_code)]

use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
#[field_names_as_array(rename_all = "lowercase")]
struct RenameLowercase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_lowercase() {
  assert_eq!(
    RenameLowercase::FIELD_NAMES_AS_ARRAY,
    ["field_one", "field_two", "field_three"],
  );
}

#[derive(FieldNamesAsArray)]
#[field_names_as_array(rename_all = "UPPERCASE")]
struct RenameUppercase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_uppercase() {
  assert_eq!(
    RenameUppercase::FIELD_NAMES_AS_ARRAY,
    ["FIELD_ONE", "FIELD_TWO", "FIELD_THREE"],
  );
}

#[derive(FieldNamesAsArray)]
#[field_names_as_array(rename_all = "PascalCase")]
struct RenamePascalCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_pascal_case() {
  assert_eq!(
    RenamePascalCase::FIELD_NAMES_AS_ARRAY,
    ["FieldOne", "FieldTwo", "FieldThree"],
  );
}

#[derive(FieldNamesAsArray)]
#[field_names_as_array(rename_all = "camelCase")]
struct RenameCamelCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_camel_case() {
  assert_eq!(
    RenameCamelCase::FIELD_NAMES_AS_ARRAY,
    ["fieldOne", "fieldTwo", "fieldThree"],
  );
}

#[derive(FieldNamesAsArray)]
#[field_names_as_array(rename_all = "snake_case")]
struct RenameSnakeCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_snake_case() {
  assert_eq!(
    RenameSnakeCase::FIELD_NAMES_AS_ARRAY,
    ["field_one", "field_two", "field_three"],
  );
}

#[derive(FieldNamesAsArray)]
#[field_names_as_array(rename_all = "SCREAMING_SNAKE_CASE")]
struct RenameScreamingSnakeCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_screaming_snake_case() {
  assert_eq!(
    RenameScreamingSnakeCase::FIELD_NAMES_AS_ARRAY,
    ["FIELD_ONE", "FIELD_TWO", "FIELD_THREE"],
  );
}

#[derive(FieldNamesAsArray)]
#[field_names_as_array(rename_all = "kebab-case")]
struct RenameKebabCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_kebab_case() {
  assert_eq!(
    RenameKebabCase::FIELD_NAMES_AS_ARRAY,
    ["field-one", "field-two", "field-three"],
  );
}

#[derive(FieldNamesAsArray)]
#[field_names_as_array(rename_all = "SCREAMING-KEBAB-CASE")]
struct RenameScreamingKebabCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_screaming_kebab_case() {
  assert_eq!(
    RenameScreamingKebabCase::FIELD_NAMES_AS_ARRAY,
    ["FIELD-ONE", "FIELD-TWO", "FIELD-THREE"],
  );
}
