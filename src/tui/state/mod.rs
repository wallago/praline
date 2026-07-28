use ratatui::{
    style::Color,
    widgets::{List, ListState},
};
use tui_input::Input;

use crate::{
    app::RepoBuilder,
    config::{Config, binds::Keybindings},
    error::Result,
};

/// Running command.
pub(crate) mod command;

/// Helper to get key bindings.
pub(crate) mod binds;

/// Form focus.
#[derive(PartialEq, Debug)]
pub enum FormFocus {
    Name,
    Desc,
    Options,
}

impl FormFocus {
    /// The next field in the cycle, wrapping around.
    fn next(&self) -> Self {
        match self {
            Self::Name => Self::Desc,
            Self::Desc => Self::Options,
            Self::Options => Self::Name,
        }
    }

    /// The previous field in the cycle, wrapping around.
    fn prev(&self) -> Self {
        match self {
            Self::Name => Self::Options,
            Self::Desc => Self::Name,
            Self::Options => Self::Desc,
        }
    }
}

/// Application state.
#[derive(Debug)]
pub struct State {
    /// Is the application running?
    pub running: bool,
    /// Terminal accent color.
    pub accent_color: Color,
    /// Repo builder.
    pub repo: RepoBuilder,
    /// Form focus.
    pub form_focus: FormFocus,
    /// Input.
    pub input: Input,
    /// Enable input.
    pub input_mode: bool,
    /// List of options.
    pub options_list: ListState,
    /// Active key bindings (defaults merged with `config.toml`).
    pub keybindings: Keybindings,
}

impl State {
    /// Constructs a new instance of [`State`].
    pub fn new(accent_color: Option<Color>, config: Config) -> Result<Self> {
        let repo = RepoBuilder::default();
        let state = Self {
            running: true,
            accent_color: accent_color.unwrap_or(Color::White),
            repo,
            form_focus: FormFocus::Name,
            input: Input::default(),
            input_mode: false,
            options_list: ListState::default().with_selected(Some(0)),
            keybindings: config.keybindings,
        };
        Ok(state)
    }
}
