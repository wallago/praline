use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType},
};

use crate::ui::prelude::*;

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
        let title = Line::from(vec![
            Span::raw(" "),
            Span::styled("tdf", SECONDARY).bold(),
            Span::raw(" "),
        ]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title)
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
        render_options(frame, chunks[0]);
        let title = Line::from(vec![
            Span::raw(" "),
            Span::styled("tdf", SECONDARY).bold(),
            Span::raw(" "),
        ]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title)
            .border_style(ASCENT_BIS);
        frame.render_widget(block, chunks[0]);
        let title = Line::from(vec![
            Span::raw(" "),
            Span::styled("tdf", SECONDARY).bold(),
            Span::raw(" "),
        ]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title)
            .border_style(ASCENT_BIS);
        frame.render_widget(block, chunks[1]);
    }
}

fn render_options(frame: &mut Frame, chunk: Rect) {
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("options", SECONDARY).bold(),
        Span::raw(" "),
    ]);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .border_style(ASCENT_BIS);
    frame.render_widget(block, chunks[0]);
}
