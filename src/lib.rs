//! Provides the `FieldNamesAsArray` procedural macro.
//!
//! The procedural macro adds the ``FIELD_NAMES_AS_ARRAY`` constant to
//! the struct.
//! The `FIELD_NAMES_AS_ARRAY` is an array containing the field names
//! of the struct (as the name suggests).
//! The visibility of the `FIELD_NAMES_AS_ARRAY` is the same as the
//! corresponding struct.
//!
//! **NOTE:** the macro can only be derived by named structs.
//!
//! ## Example
//!
//! ```rust
//! use struct_field_names_as_array::FieldNamesAsArray;
//!
//! #[derive(FieldNamesAsArray)]
//! struct Foo {
//!   bar: String,
//!   baz: String,
//!   bat: String,
//! }
//!
//! assert_eq!(Foo::FIELD_NAMES_AS_ARRAY, ["bar", "baz", "bat"]);
//! ```
//!

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

const ERR_MSG: &str =
  "Derive(FieldNamesAsArray) only applicable to named structs";

#[proc_macro_derive(FieldNamesAsArray)]
pub fn derive_field_names_as_array(
  input: TokenStream,
) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);

  let name = &input.ident;
  let vis = &input.vis;
  let (impl_generics, type_generics, where_clause) =
    &input.generics.split_for_impl();

  let field_names: Punctuated<String, Comma> = match input.data {
    Data::Struct(data_struct) => match data_struct.fields {
      Fields::Named(fields) => fields
        .named
        .into_iter()
        .map(|f| f.ident.unwrap().to_string())
        .collect(),
      _ => panic!("{}", ERR_MSG),
    },
    _ => panic!("{}", ERR_MSG),
  };

  let result = quote! {
    impl #impl_generics #name #type_generics #where_clause {
      #vis const FIELD_NAMES_AS_ARRAY: &'static [&'static str] =
        &[#field_names];
    }
  };

  TokenStream::from(result)
}
