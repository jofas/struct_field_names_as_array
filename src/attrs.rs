use syn::NestedMeta;

pub trait ParseAttribute {
  fn parse(m: &NestedMeta) -> Self;
}

pub enum ContainerAttribute {
  RenameAll(RenameAll),
}

impl ParseAttribute for ContainerAttribute {
  fn parse(m: &NestedMeta) -> Self {
    match m {
      NestedMeta::Meta(m) => match m.path().get_ident() {
        Some(i) if i == "skip" => {
          panic!(
            "skip is a field attribute, not a container attribute"
          );
        }
        Some(i) if i == "rename_all" => {
          // TODO: here parse RenameAll
          unimplemented!()
        }
        _ => panic!("unknown attribute"),
      },
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
