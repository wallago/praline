use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Stylize},
    widgets::Block,
};

use crate::tui::state::State;

/// File content.
mod content;

/// List.
mod list;

/// Renders the stage.
pub(crate) fn render_stage(state: &mut State, frame: &mut Frame, rect: Rect) {
    let inner = rect.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    let title = String::from(" Staged ").fg(Color::Gray).bold();
    frame.render_widget(
        Block::bordered()
            .title(title)
            .title_alignment(ratatui::layout::HorizontalAlignment::Center),
        rect,
    );
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .spacing(2)
            .areas(inner);
    list::render_list(state, frame, left);
    content::render_content(state, frame, right);
}
