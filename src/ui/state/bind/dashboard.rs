use strum::IntoEnumIterator;

use super::prelude::*;
use crate::{app::App, prelude::OptId, ui::prelude::*};

impl Dashboard {
    pub(super) fn on_action(&mut self, action: Action, app: &mut App) {
        match (self.focus, action) {
            (DashboardPane::Options, Action::Up) => {
                step_in(&mut self.options, OptId::iter().len(), false)
            }
            (DashboardPane::Options, Action::Down) => {
                step_in(&mut self.options, OptId::iter().len(), true)
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
