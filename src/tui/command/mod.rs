use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{config::binds::Keybindings, tui::command::input::InputCommand};

/// Input commands.
pub(crate) mod input;

/// Possible scroll areas.
#[derive(Debug, PartialEq, Eq)]
pub enum ScrollType {
    /// Form.
    Form,
    /// Options.
    Options,
}

/// Application command.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    /// Exit application.
    Exit,
    /// Do nothing.
    Nothing,
    /// Next.
    Next(ScrollType),
    /// Previous.
    Previous(ScrollType),
    /// Input command.
    Input(InputCommand),
}

impl Command {
    pub fn from_key(event: KeyEvent, binds: &Keybindings) -> Self {
        if binds.quit.matches(&event) {
            Self::Exit
        } else if binds.scroll_down.matches(&event) {
            Self::Next(ScrollType::Options)
        } else if binds.scroll_up.matches(&event) {
            Self::Previous(ScrollType::Options)
        } else if binds.generate.matches(&event) {
            Self::Nothing
        } else {
            match event.code {
                KeyCode::Tab if event.modifiers == KeyModifiers::CONTROL => {
                    Self::Previous(ScrollType::Form)
                }
                KeyCode::Tab => Self::Next(ScrollType::Form),
                KeyCode::Enter => Self::Input(InputCommand::Enter),
                _ => Self::Nothing,
            }
        }
    }
}
