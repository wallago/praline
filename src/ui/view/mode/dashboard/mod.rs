use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Color,
};

use crate::ui::{prelude::*, state::mode::DashboardPane};

/// Draws `apps`.
mod apps;
/// Draws `form`.
mod form;
/// Draws `options`.
mod options;
/// Draws `picker`.
mod picker;

/// Draws the build page: the options of the entry picked, then a column
/// with the repo's fields on top and its apps below.
pub(in crate::ui::view) fn render(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let [options, side] = Layout::horizontal([Constraint::Min(30), Constraint::Length(36)])
        .spacing(2)
        .areas(chunk);
    let [form, apps] = Layout::vertical([Constraint::Ratio(1, 2); 2]).areas(side);
    apps::render_apps(state, frame, apps);
    options::render_options(state, frame, options);
    form::render_form(state, frame, form);
    picker::render_picker(state, frame);
}

/// Border colour for `pane`: accented when it owns the keyboard.
fn border(state: &State, pane: DashboardPane) -> Color {
    if state.focus == Focus::Page && state.dashboard.focus == pane {
        ASCENT
    } else {
        ASCENT_BIS
    }
}
