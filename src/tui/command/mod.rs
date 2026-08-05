use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{config::binds::Keybindings, tui::command::input::InputCommand};

/// Input commands.
pub(crate) mod input;

/// Possible scroll areas.
#[derive(Debug, PartialEq, Eq)]
pub enum ScrollType {
    /// Form.
    Form,
    /// Options.
    Option,
    /// Staged.
    Staged,
    /// Staged Panel.
    StagedPanel,
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
    /// Generate repo.
    Generate,
    /// Confirm.
    Confirm,
    /// Back.
    Back,
}

impl Command {
    /// Command parsing.
    pub(crate) fn from_key(event: KeyEvent, binds: &Keybindings) -> Self {
        if binds.quit.matches(&event) {
            Self::Exit
        } else if binds.scroll_down.matches(&event) {
            Self::Next(ScrollType::Option)
        } else if binds.scroll_up.matches(&event) {
            Self::Previous(ScrollType::Option)
        } else if binds.generate.matches(&event) {
            Self::Generate
        } else {
            match event.code {
                KeyCode::BackTab => Self::Previous(ScrollType::Form),
                KeyCode::Tab => Self::Next(ScrollType::Form),
                KeyCode::Enter => Self::Input(InputCommand::Enter),
                _ => Self::Nothing,
            }
        }
    }

    /// Command parsing while viewing the generated repo.
    pub(crate) fn from_generated_key(event: KeyEvent, binds: &Keybindings) -> Self {
        if binds.quit.matches(&event) {
            Self::Exit
        } else if binds.leave.matches(&event) {
            Self::Back
        } else if binds.confirm.matches(&event) {
            Self::Confirm
        } else if binds.scroll_down.matches(&event) {
            Self::Next(ScrollType::Staged)
        } else if binds.scroll_up.matches(&event) {
            Self::Previous(ScrollType::Staged)
        } else {
            match event.code {
                KeyCode::BackTab => Self::Previous(ScrollType::StagedPanel),
                KeyCode::Tab => Self::Next(ScrollType::StagedPanel),
                _ => Self::Nothing,
            }
        }
    }
}
