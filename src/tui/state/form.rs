/// Form focus.
#[derive(PartialEq, Debug)]
pub enum FormFocus {
    /// Owner field.
    Owner,
    /// Name field.
    Name,
    /// Desc field.
    Desc,
    /// Tool options field.
    Options,
}

impl FormFocus {
    /// The next field in the cycle, wrapping around.
    pub(crate) fn next(&self) -> Self {
        match self {
            Self::Owner => Self::Name,
            Self::Name => Self::Desc,
            Self::Desc => Self::Options,
            Self::Options => Self::Owner,
        }
    }

    /// The previous field in the cycle, wrapping around.
    pub(crate) fn prev(&self) -> Self {
        match self {
            Self::Owner => Self::Options,
            Self::Name => Self::Owner,
            Self::Desc => Self::Name,
            Self::Options => Self::Desc,
        }
    }
}
