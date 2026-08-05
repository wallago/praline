use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Stylize},
    widgets::Block,
};

use crate::tui::state::{State, staged_panel};

/// File content.
mod content;

/// List.
mod list;

/// Renders the stage.
pub(crate) fn render_stage(state: &mut State, frame: &mut Frame, rect: Rect) {
    let inner = rect.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    let title = String::from(" Staged ").fg(Color::Gray).bold();
    frame.render_widget(
        Block::bordered()
            .title(title)
            .title_alignment(ratatui::layout::HorizontalAlignment::Center),
        rect,
    );
    let name_width = state
        .repo
        .inspect_stage()
        .and_then(|entries| entries.keys().map(|name| name.chars().count()).max())
        .unwrap_or(0);
    let max_width = inner.width / 2;
    let list_width = u16::try_from(name_width)
        .unwrap_or(u16::MAX)
        .saturating_add(5)
        .clamp(10.min(max_width), max_width);
    let [left, right] = Layout::horizontal([Constraint::Length(list_width), Constraint::Fill(1)])
        .spacing(2)
        .areas(inner);
    list::render_list(state, frame, left);
    content::render_content(
        state,
        frame,
        right,
        state.staged_panel_focus == staged_panel::StagedPanelFocus::Content,
    );
}
