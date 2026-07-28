use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Stylize},
    text::Line,
    widgets::Block,
};

use crate::tui::state::State;

/// Core Module
mod core;

/// Key bindings
mod binds;

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
        // .bg(Color::Blue)
        .bold();
        frame.render_widget(
            Block::bordered()
                .title(title)
                .title_alignment(Alignment::Center),
            chunks[0],
        );
    }
    core::render_core(state, frame, chunks[1]);
    binds::render_key_bindings(state, frame, chunks[1]);
}
