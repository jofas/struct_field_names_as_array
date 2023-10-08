/// Adds the `FIELD_NAMES_AS_ARRAY` constant to the deriving struct.
///
/// # Panics
///
/// If the token stream is not coming from a named struct or if
/// the `field_names_as_array` attribute is used wrongfully, deriving
/// this macro will fail.
///
/// # Examples
///
/// ```
/// use struct_field_names_as_array::FieldNamesAsArray;
///
/// #[derive(FieldNamesAsArray)]
/// struct Foo {
///     bar: String,
///     baz: String,
///     bat: String,
/// }
///
/// assert_eq!(Foo::FIELD_NAMES_AS_ARRAY, ["bar", "baz", "bat"]);
/// ```
///
pub use struct_field_names_as_array_derive::FieldNamesAsArray;

pub trait FieldNamesAsArray<const N: usize> {
    const FIELD_NAMES_AS_ARRAY: [&'static str; N];
}
