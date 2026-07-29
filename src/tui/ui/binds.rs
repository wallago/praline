use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::state::State;

/// Renders the key bindings.
pub(crate) fn render_key_bindings(state: &mut State, frame: &mut Frame, rect: Rect) {
    let chunks = Layout::vertical([Constraint::Percentage(100), Constraint::Min(1)]).split(rect);
    let key_bindings = state.get_key_bindings();
    let line = Line::from(
        key_bindings
            .iter()
            .enumerate()
            .flat_map(|(i, (keys, desc))| {
                vec![
                    "[".fg(Color::Rgb(100, 100, 100)),
                    keys.clone().yellow(),
                    "→ ".fg(Color::Rgb(100, 100, 100)),
                    Span::from(*desc),
                    "]".fg(Color::Rgb(100, 100, 100)),
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
