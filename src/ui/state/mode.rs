use ratatui::widgets::ListState;

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
pub enum Mode {
    #[default]
    Dashboard,
    Details,
    Settings,
}

impl Mode {
    pub fn next(self) -> Self {
        Self::from_repr(self as usize + 1).unwrap_or(self)
    }

    pub fn previous(self) -> Self {
        (self as usize)
            .checked_sub(1)
            .and_then(Self::from_repr)
            .unwrap_or(self)
    }
}

/// Focusable panes on the dashboard, in Tab order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::EnumIter)]
pub enum DashboardPane {
    #[default]
    Options,
    Tdf,
    Preview,
    Status,
}

/// Everything the dashboard remembers between visits.
#[derive(Debug, Default)]
pub struct Dashboard {
    pub focus: DashboardPane,
    pub options: ListState,
}
