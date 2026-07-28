use std::sync::mpsc;

use tui_input::{Input, backend::crossterm::EventHandler};

use crate::{
    error::Result,
    tui::{
        command::{Command, ScrollType, input::InputCommand},
        event::Event,
        state::{FormFocus, State},
    },
};

impl State {
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
            Command::Next(ScrollType::Form) => self.form_focus = self.form_focus.next(),
            Command::Previous(ScrollType::Form) => self.form_focus = self.form_focus.prev(),
            Command::Next(ScrollType::Options) => {
                if self.form_focus == FormFocus::Options {
                    self.options_list.select_next();
                }
            }
            Command::Previous(ScrollType::Options) => {
                if self.form_focus == FormFocus::Options {
                    self.options_list.select_previous();
                }
            }
            Command::Nothing => {}

            Command::Input(command) => match command {
                InputCommand::Handle(event) => {
                    self.input.handle_event(&event);
                    match self.form_focus {
                        FormFocus::Name => self.repo.name = self.input.value().to_string(),
                        FormFocus::Desc => self.repo.desc = self.input.value().to_string(),
                        FormFocus::Options => {}
                    }
                }
                InputCommand::Enter => {
                    let value = match self.form_focus {
                        FormFocus::Name => self.repo.name.clone(),
                        FormFocus::Desc => self.repo.desc.clone(),
                        FormFocus::Options => return Ok(()), // not a text field
                    };
                    self.input = Input::new(value);
                    self.input_mode = true;
                }
                InputCommand::Confirm => {
                    self.input_mode = false;
                }
                InputCommand::Exit => {
                    self.input = Input::default();
                    self.input_mode = false;
                }
            },
        }
        Ok(())
    }
}
