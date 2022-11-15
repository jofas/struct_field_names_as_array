use syn::{Lit, Meta, NestedMeta};

pub trait ParseAttribute {
  fn parse(m: &NestedMeta) -> Self;
}

pub enum ContainerAttribute {
  RenameAll(RenameAll),
}

impl ParseAttribute for ContainerAttribute {
  fn parse(m: &NestedMeta) -> Self {
    match m {
      NestedMeta::Meta(m) => {
        let ident = m.path().get_ident();

        match ident {
          Some(i) if i == "skip" => {
            panic!(
              "skip is a field attribute, not a container attribute"
            );
          }
          Some(i) if i == "rename_all" => match m {
            Meta::NameValue(mnv) => match &mnv.lit {
              Lit::Str(ls) => {
                Self::RenameAll(RenameAll::from_str(&ls.value()))
              }
              _ => panic!(
                "attribute rename_all expects a string as value"
              ),
            },
            _ => panic!("attribute rename_all badly formatted"),
          },
          _ => panic!("unknown attribute"),
        }
      }
      NestedMeta::Lit(_) => panic!("unable to parse attribute"),
    }
  }
}

pub enum FieldAttribute {
  Skip,
}

impl ParseAttribute for FieldAttribute {
  fn parse(m: &NestedMeta) -> Self {
    match m {
      NestedMeta::Meta(m) => match m.path().get_ident() {
        Some(i) if i == "skip" => Self::Skip,
        Some(i) if i == "rename_all" => {
          panic!(
            "rename_all is a container attribute, not a field attribute"
          );
        }
        _ => panic!("unknown attribute"),
      },
      NestedMeta::Lit(_) => panic!("unable to parse attribute"),
    }
  }
}

#[derive(Clone, Copy)]
pub enum RenameAll {
  LowerCase,
  UpperCase,
  PascalCase,
  CamelCase,
  SnakeCase,
  ScreamingSnakeCase,
  KebabCase,
  ScreamingKebabCase,
}

impl RenameAll {
  const FROM_STR: &'static [(&'static str, Self)] = &[
    ("lowercase", Self::LowerCase),
    ("UPPERCASE", Self::UpperCase),
    ("PascalCase", Self::PascalCase),
    ("camelCase", Self::CamelCase),
    ("snake_case", Self::SnakeCase),
    ("SCREAMING_SNAKE_CASE", Self::ScreamingSnakeCase),
    ("kebab-case", Self::KebabCase),
    ("SCREAMING-KEBAB-CASE", Self::ScreamingKebabCase),
  ];

  fn from_str(s: &str) -> Self {
    for (v, r) in Self::FROM_STR {
      if v == &s {
        return *r;
      }
    }

    panic!("unable to parse rename_all rule: {}", s);
  }
}
