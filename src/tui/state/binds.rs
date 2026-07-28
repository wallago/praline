use crate::tui::state::{FormFocus, State};

impl State {
    /// Returns the key bindings.
    pub fn get_key_bindings(&self) -> Vec<(String, &'static str)> {
        let mut binds = Vec::new();
        if self.input_mode {
            binds.push((self.keybindings.leave.to_string(), "Leave"));
            binds.push((self.keybindings.confirm.to_string(), "Confirm"));
        } else {
            if self.form_focus == FormFocus::Options {
                binds.push((self.keybindings.scroll_up.to_string(), "Scroll UP"));
                binds.push((self.keybindings.scroll_down.to_string(), "Scroll DOWN"));
            } else {
                binds.push((self.keybindings.enter.to_string(), "Enter"));
            }
            binds.push((self.keybindings.quit.to_string(), "Quit"));
            binds.push((self.keybindings.generate.to_string(), "Generate"));
        }
        binds
    }
}
