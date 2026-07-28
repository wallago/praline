use std::sync::mpsc;

use ratatui::style::Color;

use crate::{app::RepoBuilder, error::Result, tui::command::Command, tui::event::Event};

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
}

impl State {
    /// Constructs a new instance of [`State`].
    pub fn new(accent_color: Option<Color>) -> Result<Self> {
        let state = Self {
            running: true,
            accent_color: accent_color.unwrap_or(Color::White),
            repo: RepoBuilder::default(),
            form_focus: FormFocus::Name,
        };
        Ok(state)
    }

    /// Returns the key bindings.
    pub fn get_key_bindings(&self) -> Vec<(&str, &str)> {
        vec![("q", "Quit"), ("i/e", "prev/next")]
    }

    /// Runs a command and updates the state.
    pub fn run_command(
        &mut self,
        command: Command,
        event_sender: mpsc::Sender<Event>,
    ) -> Result<()> {
        match command {
            Command::Exit => {
                self.running = false;
            }
            Command::NextFormFocus => {
                self.form_focus = self.form_focus.next();
            }
            Command::PrevFormFocus => {
                self.form_focus = self.form_focus.prev();
            }
            Command::Nothing => {}
        }
        Ok(())
    }
}
