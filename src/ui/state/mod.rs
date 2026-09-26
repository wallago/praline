use std::collections::HashSet;

use ratatui::widgets::ListState;

use crate::app::App;
use crate::config::{Config, binds::Keybindings};
use crate::prelude::*;
use crate::ui::state::mode::{Dashboard, DashboardPane, Details, Mode};

mod binds;
pub(super) mod mode;

/// Which side owns the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Focus {
    #[default]
    Sidebar,
    Page,
}

/// Application state.
#[derive(Debug)]
pub struct State {
    /// Cleared to leave the event loop.
    pub(super) running: bool,
    /// Key bindings from `config.toml`.
    keybindings: Keybindings,
    pub(super) mode: Mode,
    /// Cursor in the dashboard's options list; survives mode switches.
    pub(crate) focus: Focus,
    pub(crate) dashboard: Dashboard,
    pub(crate) details: Details,
    // pub(super) settings: Settings,
    pub(crate) app: App,
}

impl State {
    /// Constructs a new instance of [`State`].
    pub(crate) fn new(config: Config, mut app: App) -> Result<Self> {
        app.generate(); // TODO handle error
        Ok(Self {
            running: true,
            mode: Mode::default(),
            keybindings: config.keybindings,
            focus: Focus::Sidebar,
            dashboard: Dashboard {
                options: ListState::default().with_selected(Some(0)),
                ..Dashboard::default()
            },
            details: Details {
                files: ListState::default().with_selected(Some(0)),
                ..Details::default()
            },
            app,
        })
    }
}
