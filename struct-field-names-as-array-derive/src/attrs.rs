use syn::{meta::ParseNestedMeta, AttrStyle, Attribute, LitStr, Result};

pub fn parse_attributes<A: ParseAttribute + Default>(attrs: &[Attribute]) -> Result<A> {
    let mut res = A::default();

    for attr in attrs {
        if attr.style != AttrStyle::Outer {
            continue;
        }

        // TODO: pass `field_names_as_array` as argument to this function
        //       when `field_names_as_slice` becomes a thing
        if attr.path().is_ident("field_names_as_array") {
            attr.parse_nested_meta(|meta| res.parse_attribute(meta))?;
        }
    }

    Ok(res)
}

pub trait ParseAttribute {
    fn parse_attribute(&mut self, m: ParseNestedMeta) -> Result<()>;
}

pub struct ContainerAttributes {
    rename_all: RenameAll,
}

impl ContainerAttributes {
    pub fn apply_to_field(&self, field: &str) -> String {
        self.rename_all.rename_field(field)
    }
}

impl ParseAttribute for ContainerAttributes {
    fn parse_attribute(&mut self, m: ParseNestedMeta) -> Result<()> {
        if m.path.is_ident("skip") {
            return Err(m.error("skip is a field attribute, not a container attribute"));
        }

        if m.path.is_ident("rename_all") {
            self.rename_all = RenameAll::from_str(&m.value()?.parse::<LitStr>()?.value());
            return Ok(());
        }

        Err(m.error("unknown attribute"))
    }
}

impl Default for ContainerAttributes {
    fn default() -> Self {
        Self {
            rename_all: RenameAll::Snake,
        }
    }
}

#[derive(Default)]
pub struct FieldAttributes {
    skip: bool,
}

impl FieldAttributes {
    pub fn apply_to_field(&self, field: &str) -> Option<String> {
        if self.skip {
            return None;
        }

        Some(field.to_owned())
    }
}

impl ParseAttribute for FieldAttributes {
    fn parse_attribute(&mut self, m: ParseNestedMeta) -> Result<()> {
        if m.path.is_ident("rename_all") {
            return Err(m.error("rename_all is a container attribute, not a field attribute"));
        }

        if m.path.is_ident("skip") {
            self.skip = true;
            return Ok(());
        }

        Err(m.error("unknown attribute"))
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

        panic!("unable to parse rename_all rule: {s}");
    }

    fn rename_field(self, v: &str) -> String {
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
                let pascal = Self::Pascal.rename_field(v);
                pascal[..1].to_ascii_lowercase() + &pascal[1..]
            }
            Self::Kebab => v.replace('_', "-"),
            Self::ScreamingKebab => Self::ScreamingSnake.rename_field(v).replace('_', "-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RenameAll;

    #[test]
    fn rename_fields() {
        for &(original, upper, pascal, camel, screaming, kebab, screaming_kebab) in &[
            (
                "outcome", "OUTCOME", "Outcome", "outcome", "OUTCOME", "outcome", "OUTCOME",
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
            assert_eq!(RenameAll::Upper.rename_field(original), upper);
            assert_eq!(RenameAll::Pascal.rename_field(original), pascal);
            assert_eq!(RenameAll::Camel.rename_field(original), camel);
            assert_eq!(RenameAll::Snake.rename_field(original), original);
            assert_eq!(RenameAll::ScreamingSnake.rename_field(original), screaming);
            assert_eq!(RenameAll::Kebab.rename_field(original), kebab);
            assert_eq!(
                RenameAll::ScreamingKebab.rename_field(original),
                screaming_kebab
            );
        }
    }
}
