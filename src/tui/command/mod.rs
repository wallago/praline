use ratatui::crossterm::event::{KeyCode, KeyEvent};

/// Application command.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    /// Exit application.
    Exit,
    /// Do nothing.
    Nothing,
    /// Next form focus.
    NextFormFocus,
    /// Previous form focus.
    PrevFormFocus,
}

impl From<KeyEvent> for Command {
    fn from(key_event: KeyEvent) -> Self {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Self::Exit,
            KeyCode::Down | KeyCode::Char('e') => Self::NextFormFocus,
            KeyCode::Up | KeyCode::Char('i') => Self::PrevFormFocus,
            _ => Self::Nothing,
        }
    }
}
