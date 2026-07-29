use std::path::Path;

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, Paragraph},
};

use crate::tui::{state::State, ui::highlight::highlight};

/// Render file content.
pub(super) fn render_content(state: &mut State, frame: &mut Frame, rect: Rect) {
    let content = "isnritnrsi";

    let content_height = content.lines().count();
    let viewport_height = rect.height.saturating_sub(2) as usize;
    let max_scroll = content_height.saturating_sub(viewport_height);
    // TODO scroll
    // if module_tab.scroll_index > max_scroll {
    //     module_tab.scroll_index = max_scroll;
    // }

    frame.render_widget(
        Paragraph::new(highlight(Path::new("./"), &content)).block(
            Block::bordered()
                .title(vec![
                    "|".fg(Color::Gray),
                    "ienien".fg(Color::Gray).bold(),
                    "| ".fg(Color::Gray),
                ])
                .title_alignment(Alignment::Center),
        ),
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
