#![allow(dead_code)]

mod test {
    use struct_field_names_as_array::FieldNamesAsArray;

    #[derive(FieldNamesAsArray)]
    #[field_names_as_array(visibility = "pub")]
    pub struct TestPub {
        f1: String,
        f2: i64,
        f3: String,
        f4: bool,
    }
    #[derive(FieldNamesAsArray)]
    #[field_names_as_array(visibility = "pub(super)")]
    pub struct TestPubSuper {
        f1: String,
        f2: i64,
        f3: String,
        f4: bool,
    }

    #[derive(FieldNamesAsArray)]
    #[field_names_as_array(visibility = "pub(crate)")]
    pub struct TestPubCrate {
        f1: String,
        f2: i64,
        f3: String,
        f4: bool,
    }
}

#[test]
fn test_visibility() {
    assert_eq!(
        test::TestPub::FIELD_NAMES_AS_ARRAY,
        ["f1", "f2", "f3", "f4"]
    );
    assert_eq!(
        test::TestPubSuper::FIELD_NAMES_AS_ARRAY,
        ["f1", "f2", "f3", "f4"]
    );
    assert_eq!(
        test::TestPubCrate::FIELD_NAMES_AS_ARRAY,
        ["f1", "f2", "f3", "f4"]
    );
}
