use ratatui::widgets::ListState;

use crate::app::App;
use crate::config::{Config, binds::Keybindings};
use crate::prelude::*;

/// Keybindings.
pub(super) mod bind;
/// Modes.
pub(super) mod mode;

pub(super) use mode::*;

/// Which side owns the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Focus {
    /// Mode navigation on the left.
    #[default]
    Sidebar,
    /// The current mode's page.
    Page,
}

/// Outcome of the last app action.
#[derive(Debug)]
pub(crate) enum Status {
    /// It failed; says why.
    Error(String),
}

/// Application state.
#[derive(Debug)]
pub(crate) struct State {
    /// Cleared to leave the event loop.
    pub(super) running: bool,
    /// Key bindings from `config.toml`.
    keybindings: Keybindings,
    /// Mode shown on the page and selected in the sidebar.
    pub(super) mode: Mode,
    /// Which side owns the keyboard.
    pub(crate) focus: Focus,
    /// Dashboard state; survives mode switches.
    pub(crate) dashboard: Dashboard,
    /// Details state; survives mode switches.
    pub(crate) details: Details,
    /// The repo being built.
    pub(crate) app: App,
    /// Outcome of the last action; cleared by the next key.
    pub(super) status: Option<Status>,
    /// The generate dialog is open: a last look before the repo is written.
    pub(crate) confirm: bool,
    /// Where the repo was created; set just before the TUI quits.
    pub(crate) created: Option<std::path::PathBuf>,
}

impl State {
    /// Constructs a new instance of [`State`], rendering `app` once so the
    /// preview has something to show.
    pub(crate) fn new(config: Config, mut app: App) -> Result<Self> {
        app.generate()?;
        Ok(Self {
            running: true,
            mode: Mode::default(),
            keybindings: config.keybindings,
            // Straight into the options: the sidebar only switches pages.
            focus: Focus::Page,
            status: None,
            confirm: false,
            created: None,
            dashboard: Dashboard {
                // The first app: what most runs start by picking.
                apps: ListState::default().with_selected(Some(1)),
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
