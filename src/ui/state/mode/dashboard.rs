use ratatui::widgets::ListState;
use ratatui_explorer::FileExplorer;
use tui_input::Input;

use crate::app::{App, View};
use crate::prelude::*;

/// Focusable panes on the dashboard, in Tab order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::EnumIter)]
pub(crate) enum DashboardPane {
    /// The repo and its apps: picks whose options the next pane shows.
    Apps,
    /// Option list of the entry picked in `Apps`.
    #[default]
    Options,
    /// Form fields.
    Form,
}

/// What the name being typed in the apps pane is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AppEdit {
    /// A new app, added once the name is confirmed.
    New,
    /// The app at this index, renamed.
    Rename(usize),
}

/// One line of the apps pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AppsRow {
    /// The repo-wide options.
    Repo,
    /// The app at this index.
    App(usize),
    /// Adds an app.
    Add,
}

/// Everything the dashboard remembers between visits.
#[derive(Debug, Default)]
pub(crate) struct Dashboard {
    /// Pane that takes the keys while the page has focus.
    pub(crate) focus: DashboardPane,
    /// Cursor in the apps pane: the repo, then each app, then "add".
    pub(crate) apps: ListState,
    /// What the name being typed is for; `None` unless naming an app.
    pub(crate) app_edit: Option<AppEdit>,
    /// Cursor in the option list.
    pub(crate) options: ListState,
    /// Field under the cursor in the form.
    pub(crate) row: FormRow,
    /// Buffer of the field or app name being typed into; `None` while browsing.
    pub(crate) editing: Option<Input>,
    /// Destination picker modal.
    pub(crate) picker: Option<FileExplorer>,
}

impl Dashboard {
    /// The line under the cursor in the apps pane.
    pub(crate) fn apps_row(&self, app: &App) -> AppsRow {
        match self.apps.selected().unwrap_or(0) {
            0 => AppsRow::Repo,
            at if at <= app.apps.len() => AppsRow::App(at - 1),
            _ => AppsRow::Add,
        }
    }

    /// Whose options the options pane shows: the entry under the apps
    /// cursor, or the last app while "add" is selected.
    pub(crate) fn view(&self, app: &App) -> View {
        match self.apps_row(app) {
            AppsRow::Repo => View::Repo,
            AppsRow::App(at) => View::App(at),
            AppsRow::Add => View::App(app.apps.len() - 1),
        }
    }
}

/// Rows of the form, in cursor order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::EnumIter)]
pub(crate) enum FormRow {
    /// Owner row form.
    #[default]
    Owner,
    /// Name row form.
    Name,
    /// Description row form.
    Desc,
    /// Destination row form.
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
