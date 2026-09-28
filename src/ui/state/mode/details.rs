use ratatui::widgets::ListState;

/// Focusable panes on the details, in Tab order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::EnumIter)]
pub(crate) enum DetailsPane {
    #[default]
    List,
    Content,
}

/// Everything the details remembers between visits.
#[derive(Debug, Default)]
pub(crate) struct Details {
    pub(crate) focus: DetailsPane,
    pub(crate) files: ListState,
    /// Preview scroll, in rows.
    pub(crate) scroll: u16,
}
