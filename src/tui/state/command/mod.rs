use std::sync::mpsc;

use crate::prelude::*;
use crate::tui::{
    command::{Command, ScrollType},
    event::Event,
    state::{State, form::FormFocus, staged_panel, toast::ToastType},
};

/// Input command.
mod input;

impl State {
    /// Runs a command and updates the state.
    pub(crate) fn run_command(
        &mut self,
        command: Command,
        _event_sender: mpsc::Sender<Event>,
    ) -> Result<()> {
        match command {
            Command::Exit => {
                self.running = false;
            }
            Command::Next(ScrollType::Form) => self.form_focus = self.form_focus.next(),
            Command::Previous(ScrollType::Form) => self.form_focus = self.form_focus.prev(),
            Command::Next(ScrollType::Option) => {
                if self.form_focus == FormFocus::Options {
                    self.option_list.select_next();
                }
            }
            Command::Previous(ScrollType::Option) => {
                if self.form_focus == FormFocus::Options {
                    self.option_list.select_previous();
                }
            }
            Command::Next(ScrollType::Staged) => {
                if self.staged_panel_focus == staged_panel::StagedPanelFocus::List {
                    self.staged_list.select_next();
                } else if self.staged_panel_focus == staged_panel::StagedPanelFocus::Content {
                    self.staged_content_viewport += 1;
                }
            }
            Command::Previous(ScrollType::Staged) => {
                if self.staged_panel_focus == staged_panel::StagedPanelFocus::List {
                    self.staged_list.select_previous();
                } else if self.staged_panel_focus == staged_panel::StagedPanelFocus::Content {
                    self.staged_content_viewport = self.staged_content_viewport.saturating_sub(1);
                }
            }
            Command::Next(ScrollType::StagedPanel) => {
                self.staged_panel_focus = self.staged_panel_focus.next();
                self.staged_content_viewport = 0;
            }
            Command::Previous(ScrollType::StagedPanel) => {
                self.staged_panel_focus = self.staged_panel_focus.prev();
                self.staged_content_viewport = 0;
            }
            Command::Next(ScrollType::Exported) => {
                self.explorer.handle(ratatui_explorer::Input::Down)?;
            }
            Command::Previous(ScrollType::Exported) => {
                self.explorer.handle(ratatui_explorer::Input::Up)?;
            }
            Command::Nothing => {}
            Command::Generate => {
                if self.repo.check() {
                    self.repo.generate()?;
                    self.screen_mode = super::screen::Screen::Generated;
                } else {
                    self.show_toast("Name is required", ToastType::Error);
                }
            }
            Command::Export => {
                self.screen_mode = super::screen::Screen::Exported;
            }
            Command::Confirm => {
                if self.screen_mode == super::screen::Screen::Exported {
                    self.explorer.handle(ratatui_explorer::Input::Right)?;
                }
            }
            Command::Create => {
                self.repo.create(self.explorer.cwd())?;
                self.running = false;
            }
            Command::Back => {
                self.screen_mode = match self.screen_mode {
                    super::screen::Screen::Form
                    | super::screen::Screen::Generated
                    | super::screen::Screen::Editing => super::screen::Screen::Form,
                    super::screen::Screen::Exported => super::screen::Screen::Generated,
                }
            }
            Command::Input(command) => self.run_input_command(command),
        }
        Ok(())
    }
}
