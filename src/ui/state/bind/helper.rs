use crate::ui::prelude::*;

impl State {
    /// Returns the key bindings.
    pub(crate) fn get_key_bindings(&self) -> Vec<(String, &'static str)> {
        let mut binds = Vec::new();
        if self.focus == Focus::Sidebar {
            binds.push((self.keybindings.scroll_up.to_string(), "Scroll UP"));
            binds.push((self.keybindings.scroll_down.to_string(), "Scroll DOWN"));
        } else {
            match self.mode {
                Mode::Dashboard => {
                    binds.push((self.keybindings.leave.to_string(), "Leave"));
                    binds.push((self.keybindings.confirm.to_string(), "Confirm"));
                    binds.push((self.keybindings.scroll_up.to_string(), "Scroll UP"));
                    binds.push((self.keybindings.scroll_down.to_string(), "Scroll DOWN"));
                    if self.dashboard.focus == DashboardPane::Options {
                        binds.push((self.keybindings.enter.to_string(), "Toogle opt"));
                    }
                }
                Mode::Details => {
                    binds.push((self.keybindings.leave.to_string(), "Leave"));
                    binds.push((self.keybindings.confirm.to_string(), "Confirm"));
                    binds.push((self.keybindings.scroll_up.to_string(), "Scroll UP"));
                    binds.push((self.keybindings.scroll_down.to_string(), "Scroll DOWN"));
                }
                Mode::Settings => {
                    binds.push((self.keybindings.leave.to_string(), "Leave"));
                    binds.push((self.keybindings.confirm.to_string(), "Confirm"));
                }
            }
        }
        binds.push((self.keybindings.quit.to_string(), "Quit"));
        binds
    }
}
