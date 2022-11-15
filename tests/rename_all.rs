/*
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(FieldNamesAsArray)]
#[field_names_as_array(rename_all = "SCREAMING-KEBAB-CASE")]
struct RenameScreamingKebabCase {
  field_one: bool,
  field_two: bool,
  field_three: bool,
}

#[test]
fn test_rename_all() {
  assert_eq!(
    RenameScreamingKebabCase::FIELD_NAMES_AS_ARRAY,
    ["FIELD-ONE", "FIELD-TWO", "FIELD-THREE"],
  );
}
*/
