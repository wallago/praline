use ratatui::widgets::ListState;
use tui_input::Input;

use crate::prelude::*;

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
    /// Field under the cursor in the form.
    pub(crate) row: FormRow,
    /// Buffer of the field being typed into; `None` while browsing.
    pub(crate) editing: Option<Input>,
}

/// Rows of the form, in cursor order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::EnumIter)]
pub(crate) enum FormRow {
    #[default]
    Owner,
    Name,
    Desc,
    Dest,
}

impl FormRow {
    /// The text field behind this row; `None` for rows you don't type into.
    pub(crate) const fn field(self) -> Option<Field> {
        match self {
            Self::Owner => Some(Field::Owner),
            Self::Name => Some(Field::Name),
            Self::Desc => Some(Field::Desc),
            Self::Dest => None,
        }
    }
}
