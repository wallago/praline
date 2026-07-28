use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Stylize},
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
        frame.render_widget(
            Block::bordered()
                .title(vec![
                    "|".fg(Color::Rgb(100, 100, 100)),
                    env!("CARGO_PKG_NAME").bold(),
                    "-".fg(Color::Rgb(100, 100, 100)),
                    env!("CARGO_PKG_VERSION").into(),
                    "|".fg(Color::Rgb(100, 100, 100)),
                ])
                .title_alignment(Alignment::Center),
            chunks[0],
        );
    }
    core::render_core(state, frame, chunks[1]);
    binds::render_key_bindings(state, frame, chunks[1]);
}
