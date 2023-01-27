extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{
  parse_macro_input, Data, DeriveInput, Fields as SynFields,
};

const ERR_MSG: &str =
  "Derive(FieldNamesAsArray) only applicable to named structs";

mod attrs;

use attrs::{
  Attributes, ContainerAttribute, Field, FieldAttribute, Fields,
};

/// `FieldNamesAsArray` procedural macro.
///
/// # Panics
///
/// If the token stream is not coming from a named struct or if
/// the `field_names_as_array` attribute is used wrongfully, deriving
/// this macro will fail.
///
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

  let ca = Attributes::<ContainerAttribute>::new(&input.attrs);

  let fields = Fields(match input.data {
    Data::Struct(data_struct) => match data_struct.fields {
      SynFields::Named(fields) => {
        let mut res = Vec::new();

        for f in fields.named {
          let fa = Attributes::<FieldAttribute>::new(&f.attrs);

          let field =
            Field::new(f.ident.unwrap().to_string(), &ca, &fa);

          res.push(field);
        }

        res
      }
      _ => panic!("{}", ERR_MSG),
    },
    _ => panic!("{}", ERR_MSG),
  });

  let array_len = fields.array_len_tokens();
  let array = fields.array_tokens();

  quote! {
    impl #impl_generics #name #type_generics #where_clause {
      #[doc=concat!("Generated array of field names for `", stringify!(#name #type_generics), "`.")]
      #vis const FIELD_NAMES_AS_ARRAY: [&'static str; #array_len] = #array;
    }
  }.into()
}
