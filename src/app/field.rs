use crate::app::App;

/// A free-text field of the repo, with the rules its value has to follow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::EnumIter)]
pub(crate) enum Field {
    /// GitHub account: `https://github.com/{owner}/{name}`.
    #[default]
    Owner,
    /// Package name, and the directory the repo is created in.
    Name,
    /// One-line summary; lands in a TOML string and a `//!` line.
    Desc,
}

impl Field {
    /// Longest value accepted, in chars.
    const fn max_len(self) -> usize {
        match self {
            Self::Owner => 39,
            Self::Name => 64,
            Self::Desc => 200,
        }
    }

    /// Whether `c` can appear anywhere in the value.
    fn allows(self, c: char) -> bool {
        match self {
            Self::Owner => c.is_ascii_alphanumeric() || c == '-',
            Self::Name => c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_',
            Self::Desc => !c.is_control() && c != '"' && c != '\\',
        }
    }

    /// Whether typing `c` into `value` is let through: the char is allowed
    /// here and there's room left for it.
    pub(crate) fn can_insert(self, value: &str, c: char) -> bool {
        self.allows(c) && value.chars().count() < self.max_len()
    }
    /// Checks the whole value; the error is short enough for a border title.
    pub(crate) fn validate(self, value: &str) -> Result<(), &'static str> {
        if value.is_empty() {
            return Err("required");
        }
        if value.chars().count() > self.max_len() {
            return Err("too long");
        }
        if !value.chars().all(|c| self.allows(c)) {
            return Err("bad character");
        }
        match self {
            Self::Owner
                if value.starts_with('-') || value.ends_with('-') || value.contains("--") =>
            {
                Err("no '-' at the ends or twice")
            }
            Self::Name if !value.starts_with(|c: char| c.is_ascii_lowercase()) => {
                Err("must start with a letter")
            }
            _ => Ok(()),
        }
    }

    /// The field's current value in `app`.
    pub(crate) fn get(self, app: &App) -> &str {
        match self {
            Self::Owner => &app.owner,
            Self::Name => &app.name,
            Self::Desc => &app.desc,
        }
    }

    /// The field's value in `app`, to overwrite.
    pub(crate) fn get_mut(self, app: &mut App) -> &mut String {
        match self {
            Self::Owner => &mut app.owner,
            Self::Name => &mut app.name,
            Self::Desc => &mut app.desc,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::Field::{self, Desc, Name, Owner};

    #[rstest]
    #[case::owner(Owner, "wallago")]
    #[case::owner_with_dash(Owner, "some-org")]
    #[case::name(Name, "praline")]
    #[case::name_with_digit_and_marks(Name, "demo_app-2")]
    #[case::desc(Desc, "Build your repo like a lazy boss.")]
    fn accepts_a_valid_value(#[case] field: Field, #[case] value: &str) {
        assert_eq!(field.validate(value), Ok(()));
    }
    #[rstest]
    #[case::empty(Owner, "")]
    #[case::owner_leading_dash(Owner, "-org")]
    #[case::owner_trailing_dash(Owner, "org-")]
    #[case::owner_double_dash(Owner, "my--org")]
    #[case::owner_underscore(Owner, "my_org")]
    #[case::name_leading_digit(Name, "2fast")]
    #[case::name_uppercase(Name, "Praline")]
    #[case::name_path(Name, "../escape")]
    #[case::desc_quote(Desc, "say \"hi\"")]
    #[case::desc_newline(Desc, "two\nlines")]
    fn rejects_an_invalid_value(#[case] field: Field, #[case] value: &str) {
        assert!(field.validate(value).is_err());
    }

    #[test]
    fn rejects_a_value_past_the_cap() {
        assert_eq!(Owner.validate(&"a".repeat(39)), Ok(()));
        assert!(Owner.validate(&"a".repeat(40)).is_err());
    }

    #[rstest]
    #[case::allowed(Name, "", 'a', true)]
    #[case::not_allowed(Name, "", 'A', false)]
    #[case::quote_in_desc(Desc, "", '"', false)]
    #[case::full(Owner, &"a".repeat(39), 'b', false)]
    fn lets_a_key_through_only_when_it_fits(
        #[case] field: Field,
        #[case] value: &str,
        #[case] c: char,
        #[case] expected: bool,
    ) {
        assert_eq!(field.can_insert(value, c), expected);
    }
}
