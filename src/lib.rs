#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
  parse_macro_input, AttrStyle, Attribute, Data, DeriveInput, Fields,
  Meta,
};

const ERR_MSG: &str =
  "Derive(FieldNamesAsArray) only applicable to named structs";

mod attrs;

use attrs::{ContainerAttribute, FieldAttribute, ParseAttribute};

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

  let c_attrs = attributes::<ContainerAttribute>(&input.attrs);

  // TODO: apply c_attrs to field

  let field_names: Punctuated<String, Comma> = match input.data {
    Data::Struct(data_struct) => match data_struct.fields {
      Fields::Named(fields) => fields
        .named
        .into_iter()
        .filter_map(|f| {
          let attrs = attributes::<FieldAttribute>(&f.attrs);

          for attr in attrs {
            match attr {
              FieldAttribute::Skip => return None,
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

fn attributes<A: ParseAttribute>(attrs: &[Attribute]) -> Vec<A> {
  let mut res = Vec::new();

  for attr in attrs {
    if attr.style != AttrStyle::Outer {
      continue;
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
      .expect("unable to parse attribute to meta");

    match meta {
      Meta::List(l) => {
        for arg in l.nested {
          res.push(A::parse(&arg));
        }
      }
      _ => {}
    }
  }

  res
}
