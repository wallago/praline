use std::time::{Duration, Instant};

use ratatui::{style::Color, widgets::ListState};
use tui_input::Input;

use crate::{
    app::RepoBuilder,
    config::{Config, binds::Keybindings},
};

/// Running command.
pub(crate) mod command;

/// Helper to get key bindings.
pub(crate) mod binds;

/// Toast.
mod toast;

/// Form.
pub(crate) mod form;

/// Application state.
pub struct State {
    /// Is the application running?
    pub running: bool,
    /// Terminal accent color.
    pub accent_color: Color,
    /// Repo builder.
    pub repo: RepoBuilder,
    /// Form focus.
    pub form_focus: form::FormFocus,
    /// Input.
    pub input: Input,
    /// Enable input.
    pub input_mode: bool,
    /// List of options.
    pub option_list: ListState,
    /// Active key bindings (defaults merged with `config.toml`).
    pub keybindings: Keybindings,
    /// Show generated repo.
    pub generated_mode: bool,
    /// Current toast, if any (non-tokio: we manage timing ourselves).
    pub toast: Option<toast::Toast>,
    /// When the current toast should be hidden.
    pub toast_expires_at: Option<Instant>,
    /// List of staged files.
    pub staged_list: ListState,
}

impl State {
    /// Constructs a new instance of [`State`].
    pub(crate) fn new(accent_color: Option<Color>, config: Config) -> Self {
        let repo = RepoBuilder::default();
        Self {
            running: true,
            accent_color: accent_color.unwrap_or(Color::White),
            repo,
            form_focus: form::FormFocus::Owner,
            input: Input::default(),
            input_mode: false,
            option_list: ListState::default().with_selected(Some(0)),
            keybindings: config.keybindings,
            generated_mode: false,
            toast: None,
            toast_expires_at: None,
            staged_list: ListState::default().with_selected(Some(0)),
        }
    }

    fn show_toast(&mut self, msg: &str, kind: toast::ToastType) {
        self.toast = Some(toast::Toast {
            message: msg.to_string(),
            kind,
        });
        self.toast_expires_at = Some(Instant::now() + Duration::from_secs(3));
    }

    /// Hide the current toast once its lifetime has elapsed. Must be called
    /// every loop iteration since we manage toast timing ourselves (non-tokio).
    pub(crate) fn tick_toasts(&mut self) {
        if let Some(expiry) = self.toast_expires_at
            && Instant::now() >= expiry
        {
            self.toast = None;
            self.toast_expires_at = None;
        }
    }
}
