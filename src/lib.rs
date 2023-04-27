#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, AttrStyle, Attribute, Data, DeriveInput, Fields, Meta, Visibility};

const ERR_MSG: &str = "Derive(FieldNamesAsArray) only applicable to named structs";

mod attrs;

use attrs::{ContainerAttribute, FieldAttribute, ParseAttribute};

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
#[proc_macro_derive(FieldNamesAsArray, attributes(field_names_as_array))]
pub fn derive_field_names_as_array(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = &input.generics.split_for_impl();

    let c_attrs = attributes::<ContainerAttribute>(&input.attrs);

    let field_names: Punctuated<String, Comma> = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields
                .named
                .into_iter()
                .filter_map(|f| {
                    let attrs = attributes::<FieldAttribute>(&f.attrs);

                    if let Some(attr) = attrs.first() {
                        match attr {
                            FieldAttribute::Skip => return None,
                        }
                    }

                    let mut res = f.ident.unwrap().to_string();

                    for t in &c_attrs {
                        res = t.apply(&res);
                    }

                    Some(res)
                })
                .collect(),
            _ => panic!("{}", ERR_MSG),
        },
        _ => panic!("{}", ERR_MSG),
    };

    let len = field_names.len();

    let vis = c_attrs
        .into_iter()
        .rev()
        .find_map(|a| match a {
            ContainerAttribute::Visibility(v) => Some(v),
            _ => None,
        })
        .unwrap_or(Visibility::Inherited);

    let result = quote! {
      impl #impl_generics #name #type_generics #where_clause {
        #[doc=concat!("Generated array of field names for `", stringify!(#name #type_generics), "`.")]
        #vis const FIELD_NAMES_AS_ARRAY: [&'static str; #len] = [#field_names];
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

        if let Meta::List(l) = meta {
            for arg in l.nested {
                res.push(A::parse(&arg));
            }
        }
    }

    res
}
