use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::{Color, Stylize},
    widgets::{Block, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::tui::{state::State, ui::highlight::highlight};

/// Render file content.
pub(super) fn render_content(state: &mut State, frame: &mut Frame, rect: Rect, focused: bool) {
    let mut entries = state.repo.inspect_stage();
    let Some(Some((_, (content, path)))) = entries
        .as_mut()
        .map(|entries| entries.get_index(state.staged_list.selected().unwrap_or_default()))
    else {
        return;
    };

    let content_height = content.lines().count();
    let viewport_height = rect.height.saturating_sub(4) as usize;
    let max_scroll = content_height.saturating_sub(viewport_height);
    if state.staged_content_viewport > max_scroll {
        state.staged_content_viewport = max_scroll;
    }

    let (highlight, ext) = highlight(path, content);
    let title = format!(" {} -> {ext} ", env!("CARGO_PKG_NAME"))
        .fg(Color::Gray)
        .bold();
    let content = Paragraph::new(highlight);
    let mut block = Block::bordered().title(title).padding(Padding::uniform(1));
    if focused {
        block = block.border_style(Color::Yellow);
    }
    frame.render_widget(
        content.block(block).scroll((
            u16::try_from(state.staged_content_viewport).unwrap_or(u16::MAX),
            0,
        )),
        rect,
    );
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        rect.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut ScrollbarState::new(max_scroll).position(state.staged_content_viewport),
    );
}
