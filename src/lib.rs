#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
  parse_macro_input, AttrStyle, Data, DeriveInput, Fields, Meta,
  NestedMeta,
};

const ERR_MSG: &str =
  "Derive(FieldNamesAsArray) only applicable to named structs";

#[proc_macro_derive(
  FieldNamesAsArray,
  attributes(field_names_as_array)
)]
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
        .filter_map(|f| {
          for attr in f.attrs.iter() {
            match attr.style {
              AttrStyle::Outer => {}
              _ => continue,
            }

            let attr_name = attr
              .path
              .segments
              .iter()
              .last()
              .cloned()
              .expect("attribute is badly formatted");

            if attr_name.ident != "field_names_as_array" {
              continue;
            }

            let meta = attr
              .parse_meta()
              .expect("cannot parse attribute to meta");

            let list = match meta {
              Meta::List(l) => l,
              _ => panic!("field_names_as_array needs an argument"),
            };

            let arg = list
              .nested
              .iter()
              .next()
              .expect("argument list cannot be empty");

            match arg {
              NestedMeta::Meta(m) => match m.path().get_ident() {
                Some(i) if i == "skip" => return None,
                _ => panic!("unknown argument"),
              },
              _ => panic!("badly formatted argument"),
            }
          }

          Some(f.ident.unwrap().to_string())
        })
        .collect(),
      _ => panic!("{}", ERR_MSG),
    },
    _ => panic!("{}", ERR_MSG),
  };

  let result = quote! {
    impl #impl_generics #name #type_generics #where_clause {
      #[doc=concat!("Generated array of field names for `", stringify!(#name #type_generics), "`.")]
      #vis const FIELD_NAMES_AS_ARRAY: &'static [&'static str] =
        &[#field_names];
    }
  };

  TokenStream::from(result)
}
