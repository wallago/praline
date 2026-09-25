use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType},
};

use crate::ui::prelude::*;

pub(in crate::ui::view) fn render(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(Direction::Horizontal, [Constraint::Min(0)])
        .spacing(2)
        .split(chunk);
    {
        row_1(state, frame, chunks[0]);
    }
}

fn row_1(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(Direction::Vertical, [Constraint::Min(0)]).split(chunk);

    {
        let title = Line::from(vec![
            Span::raw(" "),
            Span::styled("tdf", SECONDARY).bold(),
            Span::raw(" "),
        ]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);
        frame.render_widget(block, chunks[0]);
    }
}
