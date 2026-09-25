use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, HighlightSpacing, List, ListState, Padding},
};
use strum::IntoEnumIterator;

use crate::{
    prelude::OptId,
    ui::{prelude::*, state::mode::DashboardPane},
};

pub(in crate::ui::view) fn render(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(65), Constraint::Percentage(35)],
    )
    .spacing(2)
    .split(chunk);
    {
        row_1(state, frame, chunks[0]);
        row_2(state, frame, chunks[1]);
    }
}

fn row_1(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(65), Constraint::Percentage(35)],
    )
    .split(chunk);

    {
        render_options(state, frame, chunks[0]);
        let title = Line::from(vec![
            Span::raw(" "),
            Span::styled("tdf", SECONDARY).bold(),
            Span::raw(" "),
        ]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title)
            .border_style(border(state, DashboardPane::Tdf));
        frame.render_widget(block, chunks[1]);
    }
}

fn row_2(state: &State, frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(85), Constraint::Percentage(15)],
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
            .border_style(border(state, DashboardPane::Preview));
        frame.render_widget(block, chunks[0]);
        let title = Line::from(vec![
            Span::raw(" "),
            Span::styled("tdf", SECONDARY).bold(),
            Span::raw(" "),
        ]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title)
            .border_style(border(state, DashboardPane::Status));
        frame.render_widget(block, chunks[1]);
    }
}

fn render_options(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("options", SECONDARY).bold(),
        Span::raw(" "),
    ]);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .padding(Padding::new(1, 1, 1, 0))
        .border_style(border(state, DashboardPane::Options));
    let options = OptId::iter().map(<&'static str>::from).collect::<Vec<_>>();
    let list = List::new(options)
        .block(block)
        .style(SECONDARY)
        .highlight_style(Style::new().fg(ASCENT).bg(ASCENT_BIS).bold())
        .highlight_symbol(Span::styled("▌ ", ASCENT))
        .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(list, chunk, &mut state.dashboard.options);
}

/// Border colour for `pane`: accented when it owns the keyboard.
fn border(state: &State, pane: DashboardPane) -> Color {
    if state.focus == Focus::Page && state.dashboard.focus == pane {
        ASCENT
    } else {
        ASCENT_BIS
    }
}
