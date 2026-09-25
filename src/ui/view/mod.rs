use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{
        Block,
        BorderType::{self},
        Paragraph,
    },
};

use super::prelude::*;

mod mode;
mod sidebar;

/// Renders the user interface widgets.
pub(super) fn render(state: &mut State, frame: &mut Frame) {
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("praline ", ASCENT).bold(),
        Span::styled("- repo builder", SECONDARY),
        Span::raw(" "),
    ]);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(ASCENT)
        .fg(PRIMARY)
        .bg(BACKGROUND)
        .title(title)
        .title_alignment(Alignment::Left);
    frame.render_widget(block, frame.area());
    {
        let chunks = Layout::new(
            Direction::Horizontal,
            [Constraint::Length(32), Constraint::Min(0)],
        )
        .margin(2)
        .spacing(2)
        .split(frame.area());
        sidebar::render(state, frame, chunks[0]);
        mode::dashboard::render(state, frame, chunks[1]);
    }
    render_key_bindings(state, frame, frame.area());
}

/// Renders the key bindings.
fn render_key_bindings(state: &mut State, frame: &mut Frame, rect: Rect) {
    let chunks = Layout::vertical([Constraint::Percentage(100), Constraint::Min(1)]).split(rect);
    let key_bindings = state.get_key_bindings();
    let line = Line::from(
        key_bindings
            .iter()
            .enumerate()
            .flat_map(|(i, (keys, desc))| {
                vec![
                    "[".fg(UNASCENT),
                    keys.clone().fg(PRIMARY),
                    "→ ".fg(UNASCENT),
                    Span::from(*desc).fg(SECONDARY),
                    "]".fg(UNASCENT),
                    if i == key_bindings.len() - 1 { "" } else { " " }.into(),
                ]
            })
            .collect::<Vec<Span>>(),
    );
    let Ok(width) = u16::try_from(line.width()) else {
        return;
    };
    if width > chunks[1].width.saturating_sub(25) {
        return;
    }
    frame.render_widget(Paragraph::new(line.alignment(Alignment::Center)), chunks[1]);
}
