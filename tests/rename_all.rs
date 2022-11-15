use struct_field_names_as_array::FieldNamesAsArray;

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
