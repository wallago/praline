use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Stylize},
    widgets::{Block, Padding, Paragraph},
};

use crate::tui::{state::State, ui::highlight::highlight};

/// Render file content.
pub(super) fn render_content(state: &mut State, frame: &mut Frame, rect: Rect) {
    let mut entries = state.repo.inspect_stage();
    let Some(Some((name, (content, path)))) = entries
        .as_mut()
        .map(|entries| entries.get_index(state.staged_list.selected().unwrap_or_default()))
    else {
        return;
    };

    // let content_height = content.lines().count();
    // let viewport_height = rect.height.saturating_sub(2) as usize;
    // let max_scroll = content_height.saturating_sub(viewport_height);
    // TODO scroll
    // if module_tab.scroll_index > max_scroll {
    //     module_tab.scroll_index = max_scroll;
    // }

    let (highlight, ext) = highlight(path, content);
    let title = format!(" {name} -> {ext} ").fg(Color::Gray).bold();
    let content = Paragraph::new(highlight);
    let block = Block::bordered().title(title).padding(Padding::uniform(1));
    frame.render_widget(
        content.block(block),
        // .scroll((module_tab.scroll_index as u16, 0)),
        rect,
    );
    // frame.render_stateful_widget(
    //     Scrollbar::new(ScrollbarOrientation::VerticalRight)
    //         .begin_symbol(Some("↑"))
    //         .end_symbol(Some("↓")),
    //     rect.inner(Margin {
    //         vertical: 1,
    //         horizontal: 0,
    //     }),
    //     &mut ScrollbarState::new(max_scroll).position(module_tab.scroll_index),
    // );
}
