use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::tui::state::State;

/// Render list of files.
pub(super) fn render_list(state: &mut State, frame: &mut Frame, rect: Rect) {
    let Some(entries) = state.repo.inspect_stage() else {
        return;
    };

    let block = Block::new();
    let items = entries
        .iter()
        .enumerate()
        .map(|(id, (name, _))| {
            let span = Span::raw(name);
            let line = Line::from(span);
            let content = if Some(id) == state.staged_list.selected() {
                line.fg(Color::Green)
            } else {
                line
            };
            ListItem::new(vec![content])
        })
        .collect::<Vec<ListItem>>();

    let list = List::new(items).block(block).highlight_symbol("> ");
    frame.render_stateful_widget(list, rect, &mut state.staged_list);

    let mut sb = ScrollbarState::new(state.repo.options.len())
        .position(state.staged_list.selected().unwrap_or(0));
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight),
        rect.inner(Margin {
            horizontal: 0,
            vertical: 1,
        }),
        &mut sb,
    );
}
