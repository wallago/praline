use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Block,
};

use crate::tui::state::State;

/// Preview.
mod preview;

/// Form.
mod form;

/// Renders the core.
pub(crate) fn render_core(state: &mut State, frame: &mut Frame, rect: Rect) {
    frame.render_widget(Block::bordered(), rect);
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(rect);
    form::render_form(state, frame, left);
    preview::render_preview(state, frame, right);
}
