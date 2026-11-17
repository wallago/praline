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

/// Draws the build page: the repo and its apps, the options of the one
/// picked, and the repo's fields.
pub(in crate::ui::view) fn render(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let [apps, options, form] = Layout::horizontal([
        Constraint::Length(30),
        Constraint::Min(30),
        Constraint::Length(36),
    ])
    .spacing(2)
    .areas(chunk);
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
