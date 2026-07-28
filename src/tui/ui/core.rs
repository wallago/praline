use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::tui::state::{FormFocus, State};

/// Renders the core.
pub fn render_core(state: &mut State, frame: &mut Frame, rect: Rect) {
    frame.render_widget(Block::bordered(), rect);
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(rect);
    render_form(state, frame, left);
    render_preview(state, frame, right);
}

/// Left column: form.
fn render_form(state: &mut State, frame: &mut Frame, rect: Rect) {
    let inner = rect.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [name, desc, options] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
    ])
    .areas(inner);

    render_input(
        frame,
        name,
        "Name",
        &state.repo.name,
        state.form_focus == FormFocus::Name,
    );
    render_input(
        frame,
        desc,
        "Desc",
        &state.repo.desc,
        state.form_focus == FormFocus::Desc,
    );
    // render_options(state, frame, options);
}

/// Right column: read-only summary derived from the form.
fn render_preview(state: &mut State, frame: &mut Frame, rect: Rect) {
    // let checked = state.repo.options.iter().filter(|o| o.checked).count();

    let lines = vec![
        Line::from(state.repo.name.as_str()).bold(),
        Line::from(""),
        Line::from(state.repo.desc.as_str()),
        // Line::from(""),
        // Line::from(format!("Project Options : {checked}")),
    ];

    frame.render_widget(
        Paragraph::new(lines),
        rect.inner(Margin {
            vertical: 1,
            horizontal: 2,
        }),
    );
}

/// A single bordered text field.
fn render_input(frame: &mut Frame, rect: Rect, title: &str, value: &str, focused: bool) {
    let mut block = Block::bordered().title(title);
    if focused {
        block = block.border_style(Color::Yellow);
    }
    frame.render_widget(Paragraph::new(value).block(block), rect);
}
