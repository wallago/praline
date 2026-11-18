use crate::ui::prelude::*;

impl State {
    /// The keys that do something right now, for the bottom border.
    pub(crate) fn get_key_bindings(&self) -> Vec<(String, &'static str)> {
        let keys = &self.keybindings;
        let mut binds = Vec::new();
        if self.confirm {
            binds.push((keys.confirm.to_string(), "create and quit"));
            binds.push((keys.leave.to_string(), "back"));
            return binds;
        }
        if self.dashboard.picker.is_some() {
            binds.push((format!("{}/{}", keys.scroll_up, keys.scroll_down), "move"));
            binds.push((keys.enter.to_string(), "open dir"));
            binds.push(("-".into(), "parent"));
            binds.push(("_".into(), "start dir"));
            binds.push(("v".into(), "pick this dir"));
            binds.push((keys.leave.to_string(), "cancel"));
            return binds;
        }
        if self.dashboard.editing.is_some() {
            binds.push((keys.confirm.to_string(), "save"));
            binds.push((keys.leave.to_string(), "cancel"));
            return binds;
        }
        if self.focus == Focus::Sidebar {
            binds.push((format!("{}/{}", keys.scroll_up, keys.scroll_down), "page"));
            binds.push(("Tab".into(), "enter page"));
        } else {
            match self.mode {
                Mode::Dashboard => match self.dashboard.focus {
                    DashboardPane::Apps => {
                        binds.push((keys.enter.to_string(), "open"));
                        binds.push(("a".into(), "add app"));
                        binds.push(("r".into(), "rename"));
                        binds.push(("d".into(), "remove"));
                    }
                    DashboardPane::Options => {
                        binds.push((keys.enter.to_string(), "toggle"));
                        binds.push(("a".into(), "add app"));
                    }
                    DashboardPane::Form => binds.push((keys.enter.to_string(), "edit")),
                },
                Mode::Details => {
                    binds.push((format!("{}/{}", keys.scroll_up, keys.scroll_down), "scroll"));
                }
            }
            binds.push(("Tab".into(), "next pane"));
            binds.push((keys.leave.to_string(), "pages"));
        }
        binds.push((keys.create.to_string(), "create repo"));
        binds.push((keys.quit.to_string(), "quit"));
        binds
    }
}
