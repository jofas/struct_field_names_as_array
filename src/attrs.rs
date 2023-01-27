use syn::{AttrStyle, Attribute as SynAttribute};

use quote::quote;

use proc_macro2::TokenStream;

use darling::error::Error;
use darling::util::Override;
use darling::FromMeta;

use std::str::FromStr;

pub struct Fields(pub Vec<Field>);

impl Fields {
  pub fn array_tokens(&self) -> TokenStream {
    // TODO: formatting logic into field

    let raw = self.0.iter().fold(String::new(), |acc, x| {
      if x.skip {
        acc
      } else if let Some(f) = &x.flatten {
        match &f.delim {
          Some(d) => format!("{acc}\"{}{d}TODO\",", x.name),
          None => format!("{acc}\"{}.TODO\",", x.name),
        }
      } else {
        format!("{acc}\"{}\",", x.name)
      }
    });

    let inner = TokenStream::from_str(&raw).unwrap();

    quote! {
      [#inner]
    }
  }

  pub fn array_len_tokens(&self) -> TokenStream {
    // TODO: formatting logic into field

    let raw = self.0.iter().fold(String::new(), |acc, x| {
      if x.skip {
        acc
      } else if acc.is_empty() {
        String::from("1")
      } else {
        format!("{acc} + 1")
      }
    });

    let inner = TokenStream::from_str(&raw).unwrap();

    quote! {
      (#inner)
    }
  }
}

pub struct Field {
  name: String,
  skip: bool,
  flatten: Option<Flatten>,
}

impl Field {
  pub fn new<S: AsRef<str>>(
    name: S,
    container_attrs: &Attributes<ContainerAttribute>,
    field_attrs: &Attributes<FieldAttribute>,
  ) -> Self {
    let mut name = name.as_ref().to_owned();
    let mut skip = false;
    let mut flatten = None;

    for ca in container_attrs.iter() {
      name = ca.rename_all(name);
    }

    for fa in field_attrs.iter() {
      if fa.skip {
        skip = true;
      }

      if let Some(f) = &fa.flatten {
        flatten = Some(f.clone().unwrap_or_default());
      }
    }

    Self {
      name,
      skip,
      flatten,
    }
  }
}

pub struct Attributes<T> {
  inner: Vec<T>,
}

impl<T> Attributes<T> {
  fn iter(&self) -> impl Iterator<Item = &T> {
    self.inner.iter()
  }
}

impl<T: FromMeta> Attributes<T> {
  pub fn new(attrs: &[SynAttribute]) -> Self {
    let mut inner = Vec::new();

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

      inner.push(
        T::from_meta(&meta)
          .expect("unable to parse container attribute"),
      );
    }

    Self { inner }
  }
}

#[derive(FromMeta)]
pub struct ContainerAttribute {
  rename_all: RenameAll,
}

impl ContainerAttribute {
  fn rename_all<S: AsRef<str>>(&self, name: S) -> String {
    self.rename_all.apply(name)
  }
}

#[derive(FromMeta)]
pub struct FieldAttribute {
  #[darling(default)]
  skip: bool,
  #[darling(default)]
  flatten: Option<Override<Flatten>>,
}

#[derive(Clone, Copy)]
pub enum RenameAll {
  Lower,
  Upper,
  Pascal,
  Camel,
  Snake,
  ScreamingSnake,
  Kebab,
  ScreamingKebab,
}

impl FromMeta for RenameAll {
  fn from_string(value: &str) -> Result<Self, Error> {
    for (v, r) in Self::FROM_STR {
      if v == &value {
        return Ok(*r);
      }
    }

    Err(Error::unknown_value(value))
  }
}

impl RenameAll {
  const FROM_STR: &'static [(&'static str, Self)] = &[
    ("lowercase", Self::Lower),
    ("UPPERCASE", Self::Upper),
    ("PascalCase", Self::Pascal),
    ("camelCase", Self::Camel),
    ("snake_case", Self::Snake),
    ("SCREAMING_SNAKE_CASE", Self::ScreamingSnake),
    ("kebab-case", Self::Kebab),
    ("SCREAMING-KEBAB-CASE", Self::ScreamingKebab),
  ];

  fn apply<S: AsRef<str>>(self, name: S) -> String {
    let name = name.as_ref();

    match self {
      Self::Lower | Self::Snake => name.to_owned(),
      Self::Upper | Self::ScreamingSnake => name.to_ascii_uppercase(),
      Self::Pascal => {
        let mut pascal = String::new();
        let mut capitalize = true;

        for ch in name.chars() {
          if ch == '_' {
            capitalize = true;
          } else if capitalize {
            pascal.push(ch.to_ascii_uppercase());
            capitalize = false;
          } else {
            pascal.push(ch);
          }
        }

        pascal
      }
      Self::Camel => {
        let pascal = Self::Pascal.apply(name);
        pascal[..1].to_ascii_lowercase() + &pascal[1..]
      }
      Self::Kebab => name.replace('_', "-"),
      Self::ScreamingKebab => {
        Self::ScreamingSnake.apply(name).replace('_', "-")
      }
    }
  }
}

#[derive(Default, Debug, FromMeta, Clone)]
#[darling(default)]
pub struct Flatten {
  delim: Option<String>,
}

#[cfg(test)]
mod tests {
  use super::RenameAll;

  #[test]
  fn rename_fields() {
    for &(
      original,
      upper,
      pascal,
      camel,
      screaming,
      kebab,
      screaming_kebab,
    ) in &[
      (
        "outcome", "OUTCOME", "Outcome", "outcome", "OUTCOME",
        "outcome", "OUTCOME",
      ),
      (
        "very_tasty",
        "VERY_TASTY",
        "VeryTasty",
        "veryTasty",
        "VERY_TASTY",
        "very-tasty",
        "VERY-TASTY",
      ),
      ("a", "A", "A", "a", "A", "a", "A"),
      ("z42", "Z42", "Z42", "z42", "Z42", "z42", "Z42"),
    ] {
      assert_eq!(RenameAll::Upper.apply(original), upper);
      assert_eq!(RenameAll::Pascal.apply(original), pascal);
      assert_eq!(RenameAll::Camel.apply(original), camel);
      assert_eq!(RenameAll::Snake.apply(original), original);
      assert_eq!(
        RenameAll::ScreamingSnake.apply(original),
        screaming
      );
      assert_eq!(RenameAll::Kebab.apply(original), kebab);
      assert_eq!(
        RenameAll::ScreamingKebab.apply(original),
        screaming_kebab
      );
    }
  }
}
