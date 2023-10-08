use syn::{meta::ParseNestedMeta, AttrStyle, Attribute, LitStr, Result};

pub trait ParseAttributes: Sized {
    fn default(attribute: &'static str) -> Self;
    fn parse_attribute(&mut self, m: ParseNestedMeta) -> Result<()>;

    fn parse_attributes(attribute_name: &'static str, attributes: &[Attribute]) -> Result<Self> {
        let mut res = Self::default(attribute_name);

        for attribute in attributes {
            if attribute.style != AttrStyle::Outer {
                continue;
            }

            if attribute.path().is_ident(attribute_name) {
                attribute.parse_nested_meta(|meta| res.parse_attribute(meta))?;
            }
        }

        Ok(res)
    }
}

pub struct ContainerAttributes {
    attribute: &'static str,
    rename_all: RenameAll,
}

impl ContainerAttributes {
    pub fn apply_to_field(&self, field: &str) -> String {
        self.rename_all.rename_field(field)
    }

    pub fn attribute(&self) -> &'static str {
        self.attribute
    }
}

impl ParseAttributes for ContainerAttributes {
    fn default(attribute: &'static str) -> Self {
        Self {
            attribute,
            rename_all: RenameAll::Snake,
        }
    }

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

pub struct FieldAttributes {
    attribute: &'static str,
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

impl ParseAttributes for FieldAttributes {
    fn default(attribute: &'static str) -> Self {
        Self {
            attribute,
            skip: false,
        }
    }

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
