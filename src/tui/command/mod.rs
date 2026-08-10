use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{config::binds::Keybindings, tui::command::input::InputCommand};

/// Input commands.
pub(crate) mod input;

/// Possible scroll areas.
#[derive(Debug, PartialEq, Eq)]
pub enum ScrollType {
    /// Form.
    Form,
    /// Option.
    Option,
    /// Preset.
    Preset,
    /// Staged.
    Staged,
    /// Staged Panel.
    StagedPanel,
    /// Exported.
    Exported,
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
    /// Export repo.
    Export,
    /// Create repo.
    Create,
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
        } else if binds.scroll_right.matches(&event) {
            Self::Next(ScrollType::Preset)
        } else if binds.scroll_left.matches(&event) {
            Self::Previous(ScrollType::Preset)
        } else {
            match event.code {
                KeyCode::BackTab => Self::Previous(ScrollType::Form),
                KeyCode::Tab => Self::Next(ScrollType::Form),
                KeyCode::Enter => Self::Input(InputCommand::Enter),
                _ => Self::Nothing,
            }
        }
    }

    /// Command parsing while viewing the staged repo.
    pub(crate) fn from_staged_key(event: KeyEvent, binds: &Keybindings) -> Self {
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
        } else if binds.export.matches(&event) {
            Self::Export
        } else {
            match event.code {
                KeyCode::BackTab => Self::Previous(ScrollType::StagedPanel),
                KeyCode::Tab => Self::Next(ScrollType::StagedPanel),
                _ => Self::Nothing,
            }
        }
    }

    /// Command parsing while viewing the exported repo.
    pub(crate) fn from_exported_key(event: KeyEvent, binds: &Keybindings) -> Self {
        if binds.quit.matches(&event) {
            Self::Exit
        } else if binds.leave.matches(&event) {
            Self::Back
        } else if binds.confirm.matches(&event) {
            Self::Confirm
        } else if binds.create.matches(&event) {
            Self::Create
        } else if binds.scroll_down.matches(&event) {
            Self::Next(ScrollType::Exported)
        } else if binds.scroll_up.matches(&event) {
            Self::Previous(ScrollType::Exported)
        } else {
            Self::Nothing
        }
    }
}
