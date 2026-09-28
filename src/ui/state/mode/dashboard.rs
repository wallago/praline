use ratatui::widgets::ListState;

/// Focusable panes on the dashboard, in Tab order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::EnumIter)]
pub(crate) enum DashboardPane {
    /// Option list.
    #[default]
    Options,
    /// Form fields.
    Form,
}

/// Everything the dashboard remembers between visits.
#[derive(Debug, Default)]
pub(crate) struct Dashboard {
    /// Pane that takes the keys while the page has focus.
    pub(crate) focus: DashboardPane,
    /// Cursor in the option list.
    pub(crate) options: ListState,
}
