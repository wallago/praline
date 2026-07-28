use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::command::input::InputCommand;

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

impl From<KeyEvent> for Command {
    fn from(key_event: KeyEvent) -> Self {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Self::Exit,
            KeyCode::Down | KeyCode::Char('e') => Self::Next(ScrollType::Options),
            KeyCode::Up | KeyCode::Char('i') => Self::Previous(ScrollType::Options),
            KeyCode::Tab => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    Self::Previous(ScrollType::Form)
                } else {
                    Self::Next(ScrollType::Form)
                }
            }
            KeyCode::Enter => Self::Input(InputCommand::Enter),
            _ => Self::Nothing,
        }
    }
}
