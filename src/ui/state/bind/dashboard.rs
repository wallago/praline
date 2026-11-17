use std::{env::current_dir, path::Path};

use ratatui::style::Style;
use ratatui_explorer::{FileExplorer, FileExplorerBuilder, Theme};
use tui_input::Input;

use super::prelude::*;
use crate::{
    app::App,
    prelude::*,
    ui::{prelude::*, state::bind::edge},
};

impl Dashboard {
    /// Runs `action` against the focused pane.
    pub(super) fn on_action(&mut self, action: Action, app: &mut App) -> Result<()> {
        match (self.focus, action) {
            (DashboardPane::Apps, Action::Up | Action::Down) => {
                // The repo, each app, then "add app".
                step_in(&mut self.apps, app.apps.len() + 2, action == Action::Down);
                self.options.select(Some(0));
            }
            (DashboardPane::Apps, Action::Enter) => match self.apps_row(app) {
                AppsRow::Add => self.start_app_edit(AppEdit::New, String::new()),
                AppsRow::Repo | AppsRow::App(_) => self.focus = DashboardPane::Options,
            },
            (_, Action::AddApp) => {
                self.focus = DashboardPane::Apps;
                self.apps.select(Some(app.apps.len() + 1));
                self.start_app_edit(AppEdit::New, String::new());
            }
            (DashboardPane::Apps, Action::RenameApp) => {
                if let AppsRow::App(at) = self.apps_row(app) {
                    self.start_app_edit(AppEdit::Rename(at), app.apps[at].name.clone());
                }
            }
            (DashboardPane::Apps, Action::RemoveApp) => {
                if let AppsRow::App(at) = self.apps_row(app) {
                    app.remove_app(at);
                    app.generate()?;
                    let last = app.apps.len();
                    self.apps.select(Some(at.min(last - 1) + 1));
                }
            }
            (DashboardPane::Options, Action::Up | Action::Down) => {
                let len = app.rows(self.view(app)).len();
                step_in(&mut self.options, len, action == Action::Down);
            }
            (DashboardPane::Options, Action::Enter) => {
                let view = self.view(app);
                let Some(row) = self
                    .options
                    .selected()
                    .and_then(|at| app.rows(view).into_iter().nth(at))
                else {
                    return Ok(());
                };
                if row.conflicted {
                    return Ok(());
                }
                app.toggle(view, row.id);
                app.generate()?;
            }
            (DashboardPane::Form, Action::Enter) => {
                if let Some(field) = self.row.field() {
                    self.editing = Some(Input::new(field.get(app).to_owned()));
                } else if self.row == FormRow::Dest {
                    self.picker = dir_picker(&app.dest).ok();
                }
            }
            (DashboardPane::Form, Action::Up | Action::Down) => {
                self.row = neighbour(self.row, action == Action::Down)
                    .unwrap_or_else(|| edge(action == Action::Down));
            }
            _ => {}
        }
        Ok(())
    }

    /// Opens the name buffer of the apps pane, holding `value`.
    fn start_app_edit(&mut self, edit: AppEdit, value: String) {
        self.app_edit = Some(edit);
        self.editing = Some(Input::new(value));
    }

    /// Drives the open picker: move, step in and out of directories,
    /// `confirm` takes the one you're in, `leave` closes it untouched.
    pub(super) fn on_picker_action(&mut self, action: Action, app: &mut App) {
        let Some(picker) = &mut self.picker else {
            return;
        };
        let _ = match action {
            Action::Up => picker.handle(ratatui_explorer::Input::Up),
            Action::Down => picker.handle(ratatui_explorer::Input::Down),
            Action::Parent => picker.handle(ratatui_explorer::Input::Left),
            Action::Start => current_dir().and_then(|dir| picker.set_cwd(dir)),
            Action::Right => picker.set_cwd(picker.current().path.clone()),
            Action::Validate => {
                app.dest.clone_from(picker.cwd());
                self.picker = None;
                Ok(())
            }
            Action::Enter => picker.handle(ratatui_explorer::Input::Right),
            Action::Leave => {
                self.picker = None;
                Ok(())
            }
            _ => Ok(()),
        };
    }

    /// Moves focus to the neighbouring pane; `false` when there is none.
    pub(super) fn step_pane(&mut self, forward: bool) -> bool {
        let Some(pane) = neighbour(self.focus, forward) else {
            return false;
        };
        self.focus = pane;
        true
    }
}

/// A picker over the directories under `dir`. `Theme::new`, not `default`:
/// the default brings its own border, and the modal draws one already.
fn dir_picker(dir: &Path) -> std::io::Result<FileExplorer> {
    let theme = Theme::new()
        .with_dir_style(SECONDARY)
        .with_highlight_symbol("> ")
        .with_highlight_item_style(Style::default().fg(PRIMARY).bold())
        .with_highlight_dir_style(Style::default().fg(HIGHLIGHT).bold());
    FileExplorerBuilder::default()
        .working_dir(dir)
        .filter_map(|file| file.is_dir.then_some(file))
        .theme(theme)
        .build()
}
