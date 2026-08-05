use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Paragraph, WidgetRef},
};

use crate::tui::state::State;

/// Renders the export.
pub(crate) fn render_export(state: &mut State, frame: &mut Frame, rect: Rect) {
    let inner = rect.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    let title = String::from(" Export [3/3] ").fg(Color::Gray).bold();
    frame.render_widget(
        Block::bordered()
            .title(title)
            .title_alignment(ratatui::layout::HorizontalAlignment::Center),
        rect,
    );

    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Length(2), Constraint::Min(0)],
    )
    .split(inner);

    let hint = Line::from(vec![
        "Creating ".fg(Color::Gray),
        state.repo.name.as_str().fg(state.accent_color).bold(),
        " in ".fg(Color::Gray),
        state.explorer.cwd().display().to_string().bold(),
    ]);
    frame.render_widget(Paragraph::new(hint), chunks[0]);

    state
        .explorer
        .widget()
        .render_ref(chunks[1], frame.buffer_mut());
}
