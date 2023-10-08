//! TODO: top-level docs referring to other crate

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, parse_quote, ConstParam, Data, DeriveInput, Fields};

const ERR_MSG: &str = "Derive(FieldNamesAsArray) only applicable to named structs";

mod attrs;

use attrs::{parse_attributes, ContainerAttributes, FieldAttributes};

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(FieldNamesAsArray, attributes(field_names_as_array))]
pub fn derive_field_names_as_array(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = &input.generics.split_for_impl();

    let container_attributes = parse_attributes::<ContainerAttributes>(&input.attrs).unwrap();

    let field_names: Punctuated<String, Comma> = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields
                .named
                .into_iter()
                .filter_map(|f| {
                    let field_attributes = parse_attributes::<FieldAttributes>(&f.attrs).unwrap();

                    let field = f.ident.unwrap().to_string();

                    let field = container_attributes.apply_to_field(&field);

                    field_attributes.apply_to_field(&field)
                })
                .collect(),
            _ => panic!("{ERR_MSG}"),
        },
        _ => panic!("{ERR_MSG}"),
    };

    let len = field_names.len();

    let result = quote! {
      impl #impl_generics ::struct_field_names_as_array::FieldNamesAsArray<#len> for #name #type_generics #where_clause {
        #[doc=concat!("Generated array of field names for `", stringify!(#name #type_generics), "`.")]
        const FIELD_NAMES_AS_ARRAY: [&'static str; #len] = [#field_names];
      }
    };

    TokenStream::from(result)
}
