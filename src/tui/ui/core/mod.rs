use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    widgets::Block,
};

use crate::tui::state::State;

/// Preview.
mod preview;

/// Form.
mod form;

/// Renders the core.
pub(crate) fn render_core(state: &mut State, frame: &mut Frame, rect: Rect) {
    let title = String::from(" Home [1/3] ").fg(Color::Gray).bold();
    frame.render_widget(
        Block::bordered()
            .title(title)
            .title_alignment(ratatui::layout::HorizontalAlignment::Center),
        rect,
    );
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(rect);
    form::render_form(state, frame, left);
    preview::render_preview(state, frame, right);
}
