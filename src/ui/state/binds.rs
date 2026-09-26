use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::ListState,
};
use strum::IntoEnumIterator;

use crate::{
    app::App,
    prelude::OptId,
    ui::{
        prelude::*,
        state::mode::{Dashboard, DashboardPane, Mode},
    },
};

/// What a key asks for, once bindings are resolved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    /// Leave the app.
    Quit,
    /// Open the key reference.
    Help,
    /// Unbound key.
    Nothing,
    /// Move up or scroll up.
    Up,
    /// Move down or scroll down.
    Down,
    /// Next pane in tab order.
    NextPane,
    /// Previous pane in tab order.
    PrevPane,
    Enter,
    Leave,
    Generate,
}

impl State {
    /// Returns the key bindings.
    pub(crate) fn get_key_bindings(&self) -> Vec<(String, &'static str)> {
        let mut binds = Vec::new();
        if self.focus == Focus::Sidebar {
            binds.push((self.keybindings.scroll_up.to_string(), "Scroll UP"));
            binds.push((self.keybindings.scroll_down.to_string(), "Scroll DOWN"));
        } else {
            match self.mode {
                Mode::Dashboard => {
                    binds.push((self.keybindings.leave.to_string(), "Leave"));
                    binds.push((self.keybindings.confirm.to_string(), "Confirm"));
                    binds.push((self.keybindings.scroll_up.to_string(), "Scroll UP"));
                    binds.push((self.keybindings.scroll_down.to_string(), "Scroll DOWN"));
                    binds.push((self.keybindings.generate.to_string(), "Generate"));
                    if self.dashboard.focus == DashboardPane::Options {
                        binds.push((self.keybindings.enter.to_string(), "Toogle opt"));
                    }
                }
                Mode::Details => {
                    binds.push((self.keybindings.leave.to_string(), "Leave"));
                    binds.push((self.keybindings.confirm.to_string(), "Confirm"));
                }
                Mode::Settings => {
                    binds.push((self.keybindings.leave.to_string(), "Leave"));
                    binds.push((self.keybindings.confirm.to_string(), "Confirm"));
                }
            }
        }
        binds.push((self.keybindings.quit.to_string(), "Quit"));
        binds
    }

    /// Resolves `key`: bindings from `config.toml` first, then fixed keys.
    fn action(&self, key: &KeyEvent) -> Action {
        let keys = &self.keybindings;
        let bound = [
            (&keys.quit, Action::Quit),
            (&keys.scroll_up, Action::Up),
            (&keys.scroll_down, Action::Down),
            (&keys.leave, Action::Leave),
            (&keys.confirm, Action::Enter),
            (&keys.generate, Action::Generate),
        ];
        if let Some(&(_, action)) = bound.iter().find(|(pattern, _)| pattern.matches(key)) {
            return action;
        }
        match key.code {
            KeyCode::Up => Action::Up,
            KeyCode::Down => Action::Down,
            KeyCode::Tab => Action::NextPane,
            KeyCode::BackTab => Action::PrevPane,
            KeyCode::Char('?') => Action::Help,
            _ => Action::Nothing,
        }
    }

    /// Runs `action` against the panes.
    fn on_action(&mut self, action: Action) {
        match (self.focus, action) {
            (_, Action::Quit) => self.running = false,
            (Focus::Sidebar, Action::Up) => self.mode = self.mode.previous(),
            (Focus::Sidebar, Action::Down) => self.mode = self.mode.next(),
            (Focus::Sidebar, Action::NextPane | Action::PrevPane) => {
                let forward = action == Action::NextPane;
                let entered = match self.mode {
                    Mode::Dashboard => {
                        self.dashboard.focus = edge(forward);
                        true
                    }
                    Mode::Details | Mode::Settings => false,
                };
                if entered {
                    self.focus = Focus::Page;
                }
            }
            (Focus::Sidebar, Action::Enter) => self.focus = Focus::Page,
            (Focus::Page, Action::Leave) => self.focus = Focus::Sidebar,
            (Focus::Page, Action::Generate) => {
                if self.mode == Mode::Dashboard {
                    self.app.generate(); // TODO catch error
                }
            }
            (Focus::Page, Action::NextPane | Action::PrevPane) => {
                let forward = action == Action::NextPane;
                let stayed = match self.mode {
                    Mode::Dashboard => self.dashboard.step_pane(forward),
                    Mode::Details | Mode::Settings => false,
                };
                if !stayed {
                    self.focus = Focus::Sidebar;
                }
            }
            (Focus::Page, _) => match self.mode {
                Mode::Dashboard => self.dashboard.on_action(action, &mut self.app),
                Mode::Details | Mode::Settings => {}
            },
            _ => {}
        }
    }

    /// Handles a key press.
    pub(crate) fn on_key(&mut self, key: KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == self.keybindings.quit.code {
            self.running = false;
            return;
        }
        self.on_action(self.action(&key));
    }
}

impl Dashboard {
    fn on_action(&mut self, action: Action, app: &mut App) {
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
            }
            _ => {}
        }
    }

    /// Moves focus to the neighbouring pane; `false` when there is none.
    fn step_pane(&mut self, forward: bool) -> bool {
        let Some(pane) = neighbour(self.focus, forward) else {
            return false;
        };
        self.focus = pane;
        true
    }
}

/// Moves `table`'s selection one row within `len` rows, without wrapping.
fn step_in(table: &mut ListState, len: usize, down: bool) {
    let at = table.selected().unwrap_or(0);
    let next = if down { at + 1 } else { at.saturating_sub(1) };
    table.select(Some(next.min(len.saturating_sub(1))));
}

/// Next (or previous) variant of `current` in declaration order; `None` past either end.
fn neighbour<T: IntoEnumIterator + PartialEq + Copy>(current: T, forward: bool) -> Option<T> {
    let all: Vec<T> = T::iter().collect();
    let at = all.iter().position(|p| *p == current)?;
    let to = if forward { at + 1 } else { at.checked_sub(1)? };
    all.get(to).copied()
}

/// First variant when entering forward, last when entering backward.
fn edge<T: IntoEnumIterator + Default>(forward: bool) -> T {
    if forward {
        T::iter().next()
    } else {
        T::iter().last()
    }
    .unwrap_or_default()
}
