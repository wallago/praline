mod dashboard;
mod details;

pub(crate) use dashboard::*;
pub(crate) use details::*;

/// Which mode is currently showing.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    Eq,
    strum::EnumIter,
    strum::IntoStaticStr,
    strum::FromRepr,
)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum Mode {
    #[default]
    Dashboard,
    Details,
    Settings,
}

impl Mode {
    pub(crate) fn next(self) -> Self {
        Self::from_repr(self as usize + 1).unwrap_or(self)
    }

    pub(crate) fn previous(self) -> Self {
        (self as usize)
            .checked_sub(1)
            .and_then(Self::from_repr)
            .unwrap_or(self)
    }
}
