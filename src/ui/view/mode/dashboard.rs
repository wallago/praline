use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType},
};

use crate::ui::theme::ASCENT_BIS;

pub(in crate::ui::view) fn render(frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(65), Constraint::Percentage(35)],
    )
    .spacing(2)
    .split(chunk);
    {
        row_1(frame, chunks[0]);
        row_2(frame, chunks[1]);
    }
}

fn row_1(frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(65), Constraint::Percentage(35)],
    )
    .split(chunk);

    {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(ASCENT_BIS);
        frame.render_widget(block, chunks[0]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(ASCENT_BIS);
        frame.render_widget(block, chunks[1]);
    }
}

fn row_2(frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(85), Constraint::Percentage(15)],
    )
    .split(chunk);

    {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(ASCENT_BIS);
        frame.render_widget(block, chunks[0]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(ASCENT_BIS);
        frame.render_widget(block, chunks[1]);
    }
}
