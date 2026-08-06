use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::tui::state::{State, screen::Screen};

/// Core part.
mod core;

/// Stage part.
mod stage;

/// Export part.
mod export;

/// Key bindings.
mod binds;

/// Utils functions.
mod utils;

/// Content highlight.
mod highlight;

/// Renders the user interface widgets.
pub fn render(state: &mut State, frame: &mut Frame) {
    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Length(3), Constraint::Min(0)],
    )
    .direction(Direction::Vertical)
    .margin(1)
    .split(frame.area());
    {
        let gray = Color::Gray;
        let title = Line::from(vec![
            " > ".fg(gray),
            env!("CARGO_PKG_NAME").bold(),
            "-".fg(gray),
            env!("CARGO_PKG_VERSION").into(),
            " < ".fg(gray),
        ])
        .bold();

        let block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Center);
        let inner = block.inner(chunks[0]);
        frame.render_widget(block, chunks[0]);

        let desc = Paragraph::new(env!("CARGO_PKG_DESCRIPTION").fg(Color::Blue))
            .alignment(Alignment::Center)
            .bold();
        frame.render_widget(desc, inner);
    }
    if state.screen_mode == Screen::Generated {
        stage::render_stage(state, frame, chunks[1]);
    } else if state.screen_mode == Screen::Exported {
        export::render_export(state, frame, chunks[1]);
    } else {
        core::render_core(state, frame, chunks[1]);
    }
    binds::render_key_bindings(state, frame, chunks[1]);

    // Draw the current toast (if any) as a compact single row in the top-right.
    if let Some(toast) = &state.toast {
        let full = frame.area();
        let text = format!(" {} ", toast.message);
        let len = u16::try_from(text.chars().count()).unwrap_or(u16::MAX);
        let width = (len + 2).min(full.width);
        let area = Rect {
            x: full.x + full.width.saturating_sub(width) - 1,
            y: full.y,
            width,
            height: 1,
        };
        let block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_set(symbols::border::QUADRANT_OUTSIDE)
            .border_style(Style::default().fg(toast.kind.color()));
        frame.render_widget(Clear, area);
        frame.render_widget(Paragraph::new(text).block(block), area);
    }
}
