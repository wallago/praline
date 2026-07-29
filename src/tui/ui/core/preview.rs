use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
};

use crate::tui::state::State;

/// Right column: read-only summary derived from the form.
pub(super) fn render_preview(state: &mut State, frame: &mut Frame, rect: Rect) {
    let checked = state.repo.options.iter().filter(|o| o.checked).count();

    let name = if state.repo.name.is_empty() {
        "No name define yet."
    } else {
        state.repo.name.as_str()
    };
    let desc = if state.repo.desc.is_empty() {
        "No desc define yet."
    } else {
        state.repo.desc.as_str()
    };
    let lines = vec![
        Line::from(name).bold(),
        Line::from(""),
        Line::from(desc),
        Line::from(""),
        Line::from(format!("Selected options : {checked}")),
    ];

    frame.render_widget(
        Paragraph::new(lines),
        rect.inner(Margin {
            vertical: 1,
            horizontal: 2,
        }),
    );
}
