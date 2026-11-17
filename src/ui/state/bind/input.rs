use ratatui::crossterm::event::{Event, KeyEvent};
use tui_input::{InputRequest, backend::crossterm::to_input_request};

use crate::{app::App, prelude::*, ui::prelude::*};

impl State {
    /// Feeds `key` to the open field: `leave` drops the edit, `confirm` saves
    /// it once it passes the field's rules, anything else edits the buffer.
    pub(super) fn on_edit_key(&mut self, key: KeyEvent) -> Result<()> {
        let dashboard = &mut self.dashboard;
        let Some(input) = &mut dashboard.editing else {
            return Ok(());
        };
        if let Some(edit) = dashboard.app_edit {
            return self.on_app_name_key(key, edit);
        }
        if self.keybindings.leave.matches(&key) {
            dashboard.editing = None;
        } else if self.keybindings.confirm.matches(&key)
            && let Some(field) = dashboard.row.field()
        {
            if field.validate(input.value()).is_ok() {
                input.value().clone_into(field.get_mut(&mut self.app));
                dashboard.editing = None;
                self.app.generate()?;
            }
        } else if let Some(request) = to_input_request(&Event::Key(key))
            && let Some(field) = dashboard.row.field()
        {
            // A char the field can't hold is dropped before it's ever shown.
            if let InputRequest::InsertChar(c) = request
                && !field.can_insert(input.value(), c)
            {
                return Ok(());
            }
            input.handle(request);
        }
        Ok(())
    }

    /// Feeds `key` to the app name being typed: `leave` drops it, `confirm`
    /// adds or renames the app once the name is a valid package name that no
    /// other app has.
    fn on_app_name_key(&mut self, key: KeyEvent, edit: AppEdit) -> Result<()> {
        let dashboard = &mut self.dashboard;
        let Some(input) = &mut dashboard.editing else {
            return Ok(());
        };
        if self.keybindings.leave.matches(&key) {
            dashboard.editing = None;
            dashboard.app_edit = None;
        } else if self.keybindings.confirm.matches(&key) {
            let name = input.value().to_owned();
            if app_name_error(&self.app, &name, edit).is_some() {
                return Ok(());
            }
            match edit {
                AppEdit::New => {
                    self.app.add_app(name);
                    dashboard.apps.select(Some(self.app.apps.len()));
                    dashboard.options.select(Some(0));
                    dashboard.focus = DashboardPane::Options;
                }
                AppEdit::Rename(at) => self.app.apps[at].name = name,
            }
            dashboard.editing = None;
            dashboard.app_edit = None;
            self.app.generate()?;
        } else if let Some(request) = to_input_request(&Event::Key(key)) {
            if let InputRequest::InsertChar(c) = request
                && !Field::Name.can_insert(input.value(), c)
            {
                return Ok(());
            }
            input.handle(request);
        }
        Ok(())
    }
}

/// Why `name` can't be the name `edit` gives an app, if it can't: the package
/// name rules, and no two apps alike.
pub(crate) fn app_name_error(app: &App, name: &str, edit: AppEdit) -> Option<&'static str> {
    if let Err(reason) = Field::Name.validate(name) {
        return Some(reason);
    }
    let taken = app
        .apps
        .iter()
        .enumerate()
        .any(|(at, unit)| unit.name == name && edit != AppEdit::Rename(at));
    taken.then_some("already taken")
}
