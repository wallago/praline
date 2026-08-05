/// Staged panel focus.
#[derive(PartialEq, Debug)]
pub enum StagedPanelFocus {
    /// List.
    List,
    /// Content.
    Content,
}

impl StagedPanelFocus {
    /// The next field in the cycle, wrapping around.
    pub(crate) fn next(&self) -> Self {
        match self {
            Self::List => Self::Content,
            Self::Content => Self::List,
        }
    }

    /// The previous field in the cycle, wrapping around.
    pub(crate) fn prev(&self) -> Self {
        match self {
            Self::List => Self::Content,
            Self::Content => Self::List,
        }
    }
}
