use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Block,
};

use crate::tui::state::State;

/// File content.
mod content;

// /// List.
// mod list;

/// Renders the stage.
pub(crate) fn render_stage(state: &mut State, frame: &mut Frame, rect: Rect) {
    frame.render_widget(Block::bordered(), rect);
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(rect);
    // form::render_form(state, frame, left);
    content::render_content(state, frame, right);
}
