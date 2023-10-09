/// Derives the [`FieldNamesAsArray`] trait.
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
#[cfg(feature = "derive")]
pub use struct_field_names_as_array_derive::FieldNamesAsArray;

/// Derives the [`FieldNamesAsSlice`] trait.
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
/// use struct_field_names_as_array::FieldNamesAsSlice;
///
/// #[derive(FieldNamesAsSlice)]
/// struct Foo {
///     bar: String,
///     baz: String,
///     bat: String,
/// }
///
/// assert_eq!(Foo::FIELD_NAMES_AS_SLICE, ["bar", "baz", "bat"]);
/// ```
///
#[cfg(feature = "derive")]
pub use struct_field_names_as_array_derive::FieldNamesAsSlice;

/// Exposes the `FIELD_NAMES_AS_ARRAY` constant.
///
pub trait FieldNamesAsArray<const N: usize> {
    const FIELD_NAMES_AS_ARRAY: [&'static str; N];
}

/// Exposes the `FIELD_NAMES_AS_SLICE` constant.
///
pub trait FieldNamesAsSlice {
    const FIELD_NAMES_AS_SLICE: &'static [&'static str];
}
