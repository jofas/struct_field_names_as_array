use syn::{Lit, Meta, NestedMeta};

pub trait ParseAttribute {
  fn parse(m: &NestedMeta) -> Self;
}

pub enum ContainerAttribute {
  RenameAll(RenameAll),
}

impl ContainerAttribute {
  pub fn apply(&self, v: &str) -> String {
    match self {
      Self::RenameAll(rn) => rn.apply(v),
    }
  }
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
  Lower,
  Upper,
  Pascal,
  Camel,
  Snake,
  ScreamingSnake,
  Kebab,
  ScreamingKebab,
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

  fn from_str(s: &str) -> Self {
    for (v, r) in Self::FROM_STR {
      if v == &s {
        return *r;
      }
    }

    panic!("unable to parse rename_all rule: {}", s);
  }

  fn apply(self, v: &str) -> String {
    match self {
      Self::Lower | Self::Snake => v.to_owned(),
      Self::Upper | Self::ScreamingSnake => v.to_ascii_uppercase(),
      Self::Pascal => {
        let mut pascal = String::new();
        let mut capitalize = true;
        for ch in v.chars() {
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
        let pascal = Self::Pascal.apply(v);
        pascal[..1].to_ascii_lowercase() + &pascal[1..]
      }
      Self::Kebab => v.replace('_', "-"),
      Self::ScreamingKebab => {
        Self::ScreamingSnake.apply(v).replace('_', "-")
      }
    }
  }
}
