use std::time::{Duration, Instant};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Style},
    widgets::ListState,
};
use ratatui_explorer::{FileExplorer, FileExplorerBuilder, Theme};
use tui_input::Input;

use crate::prelude::*;
use crate::{
    config::{Config, binds::Keybindings},
    ui::prelude::*,
};

mod binds;

/// Which mode is currently showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Dashboard,
    Details,
    Settings,
}

/// Application state.
#[derive(Debug)]
pub struct State {
    /// Cleared to leave the event loop.
    pub(super) running: bool,
    /// Key bindings from `config.toml`.
    keybindings: Keybindings,
    mode: Mode,
}

impl State {
    /// Constructs a new instance of [`State`].
    pub(crate) fn new(config: Config) -> Result<Self> {
        Ok(Self {
            running: true,
            mode: Mode::Dashboard,
            keybindings: config.keybindings,
        })
    }
}

impl State {
    /// Handles a key press.
    pub(crate) fn on_key(&mut self, key: KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.running = false;
            return;
        }
        // match self.modal.take() {
        //     Some(modal) => self.modal = self.on_modal_key(modal, key),
        //     None => self.on_action(self.action(&key)),
        // }
    }
}
