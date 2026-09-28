use strum::IntoEnumIterator;
use tui_input::Input;

use super::prelude::*;
use crate::{
    app::{App, prelude::OptId},
    ui::{prelude::*, state::bind::edge},
};

impl Dashboard {
    /// Runs `action` against the focused pane.
    pub(super) fn on_action(&mut self, action: Action, app: &mut App) {
        match (self.focus, action) {
            (DashboardPane::Options, Action::Up | Action::Down) => {
                step_in(
                    &mut self.options,
                    OptId::iter().len(),
                    action == Action::Down,
                );
            }
            (DashboardPane::Options, Action::Enter) => {
                let Some(opt) = self
                    .options
                    .selected()
                    .and_then(|at| app.options.get_mut(at))
                else {
                    return;
                };
                opt.checked = !opt.checked;
                app.generate(); // TODO handle error.
            }
            (DashboardPane::Form, Action::Enter) => {
                self.editing = Some(Input::new(self.field.get(app).to_owned()));
            }
            (DashboardPane::Form, Action::Up | Action::Down) => {
                self.field = neighbour(self.field, action == Action::Down)
                    .unwrap_or_else(|| edge(action == Action::Down));
            }
            _ => {}
        }
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
