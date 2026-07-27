use std::sync::mpsc;

use ratatui::style::Color;

use crate::{command::Command, error::Result, tui::event::Event};

/// Application state.
#[derive(Debug)]
pub struct State {
    /// Is the application running?
    pub running: bool,
    /// Terminal accent color.
    pub accent_color: Color,
}

impl State {
    /// Constructs a new instance of [`State`].
    pub fn new(accent_color: Option<Color>) -> Result<Self> {
        let state = Self {
            running: true,
            accent_color: accent_color.unwrap_or(Color::White),
        };
        Ok(state)
    }

    /// Returns the key bindings.
    pub fn get_key_bindings(&self) -> Vec<(&str, &str)> {
        vec![("q", "Quit")]
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
            Command::Nothing => {}
        }
        Ok(())
    }
}
