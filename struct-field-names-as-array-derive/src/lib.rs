//! TODO: top-level docs referring to other crate

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Error, Field, Fields, FieldsNamed, Result,
};

mod attrs;

use attrs::{ContainerAttributes, FieldAttributes, ParseAttributes};

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(FieldNamesAsArray, attributes(field_names_as_array))]
pub fn derive_field_names_as_array(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let (impl_generics, type_generics, where_clause) = &input.generics.split_for_impl();

    let container_attributes =
        ContainerAttributes::parse_attributes("field_names_as_array", &input.attrs).unwrap();

    let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = input.data
    else {
        panic!("Derive(FieldNamesAsArray) only applicable to named structs");
    };

    let field_names = field_names(named, &container_attributes).unwrap();

    let len = field_names.len();

    TokenStream::from(quote! {
        impl #impl_generics ::struct_field_names_as_array::FieldNamesAsArray<#len> for #name #type_generics #where_clause {
            #[doc=concat!("Generated array of field names for `", stringify!(#name #type_generics), "`.")]
            const FIELD_NAMES_AS_ARRAY: [&'static str; #len] = [#(#field_names),*];
        }
    })
}

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(FieldNamesAsSlice, attributes(field_names_as_slice))]
pub fn derive_field_names_as_slice(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let (impl_generics, type_generics, where_clause) = &input.generics.split_for_impl();

    let container_attributes =
        ContainerAttributes::parse_attributes("field_names_as_slice", &input.attrs).unwrap();

    let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = input.data
    else {
        panic!("Derive(FieldNamesAsSlice) only applicable to named structs");
    };

    let field_names = field_names(named, &container_attributes).unwrap();

    TokenStream::from(quote! {
        impl #impl_generics ::struct_field_names_as_array::FieldNamesAsSlice for #name #type_generics #where_clause {
            #[doc=concat!("Generated slice of field names for `", stringify!(#name #type_generics), "`.")]
            const FIELD_NAMES_AS_SLICE: &'static [&'static str] = &[#(#field_names),*];
        }
    })
}

fn field_names(
    fields: Punctuated<Field, Comma>,
    container_attributes: &ContainerAttributes,
) -> Result<Vec<String>> {
    let mut res = Vec::new();

    for field in fields {
        let field_attributes =
            FieldAttributes::parse_attributes(container_attributes.attribute(), &field.attrs)?;

        let Some(field) = field.ident else {
            return Err(Error::new_spanned(field, "field must be a named field"));
        };

        let field = container_attributes.apply_to_field(&field.to_string());

        if let Some(field) = field_attributes.apply_to_field(&field) {
            res.push(field);
        }
    }

    Ok(res)
}
