/// Form focus.
#[derive(PartialEq, Debug)]
pub enum FormFocus {
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
            Self::Name => Self::Desc,
            Self::Desc => Self::Options,
            Self::Options => Self::Name,
        }
    }

    /// The previous field in the cycle, wrapping around.
    pub(crate) fn prev(&self) -> Self {
        match self {
            Self::Name => Self::Options,
            Self::Desc => Self::Name,
            Self::Options => Self::Desc,
        }
    }
}
