use ratatui::widgets::ListState;

/// Focusable panes on the dashboard, in Tab order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::EnumIter)]
pub(crate) enum DashboardPane {
    #[default]
    Options,
    Tdf,
    Preview,
    Status,
}

/// Everything the dashboard remembers between visits.
#[derive(Debug, Default)]
pub(crate) struct Dashboard {
    pub(crate) focus: DashboardPane,
    pub(crate) options: ListState,
}
