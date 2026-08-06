use std::sync::mpsc;

use tui_input::{Input, backend::crossterm::EventHandler};

use crate::{
    error::Result,
    tui::{
        command::{Command, ScrollType, input::InputCommand},
        event::Event,
        state::{State, form::FormFocus, staged_panel, toast::ToastType},
    },
};

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
                    self.generated_mode = true;
                } else {
                    self.show_toast("Name is required", ToastType::Error);
                }
            }
            Command::Export => {
                self.generated_mode = false;
                self.exported_mode = true;
            }
            Command::Confirm => {
                if self.exported_mode {
                    self.explorer.handle(ratatui_explorer::Input::Right)?;
                }
            }
            Command::Create => {
                self.repo.create(self.explorer.cwd())?;
                self.exported_mode = false;
                self.running = false;
            }
            Command::Back => {
                if self.generated_mode {
                    self.generated_mode = false;
                } else if self.exported_mode {
                    self.exported_mode = false;
                    self.generated_mode = true;
                }
            }
            Command::Input(command) => match command {
                InputCommand::Handle(event) => {
                    self.input.handle_event(&event);
                    match self.form_focus {
                        FormFocus::Owner => self.repo.owner = self.input.value().to_string(),
                        FormFocus::Name => self.repo.name = self.input.value().to_string(),
                        FormFocus::Desc => self.repo.desc = self.input.value().to_string(),
                        FormFocus::Options => {}
                    }
                }
                InputCommand::Enter => {
                    if self.form_focus == FormFocus::Options
                        && let Some(id) = self.option_list.selected()
                    {
                        if let Some(opt) = self.repo.options.get_mut(id) {
                            opt.checked = !opt.checked;
                        }
                    } else {
                        let value = match self.form_focus {
                            FormFocus::Owner => self.repo.owner.clone(),
                            FormFocus::Name => self.repo.name.clone(),
                            FormFocus::Desc => self.repo.desc.clone(),
                            FormFocus::Options => return Ok(()),
                        };
                        self.input = Input::new(value);
                        self.input_mode = true;
                    }
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
