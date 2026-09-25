use crate::ui::prelude::*;

impl State {
    /// Returns the key bindings.
    pub(crate) fn get_key_bindings(&self) -> Vec<(String, &'static str)> {
        let mut binds = Vec::new();
        match self.mode {
            Mode::Dashboard => {
                binds.push((self.keybindings.leave.to_string(), "Leave"));
                binds.push((self.keybindings.confirm.to_string(), "Confirm"));
            }
            Mode::Details => {
                binds.push((self.keybindings.leave.to_string(), "Leave"));
                binds.push((self.keybindings.confirm.to_string(), "Confirm"));
            }
            Mode::Settings => {
                binds.push((self.keybindings.leave.to_string(), "Leave"));
                binds.push((self.keybindings.confirm.to_string(), "Confirm"));
            }
        }
        binds.push((self.keybindings.quit.to_string(), "Quit"));
        binds
    }
}
