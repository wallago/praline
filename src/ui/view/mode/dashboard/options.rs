use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, HighlightSpacing, List, Padding},
};

use crate::{
    app::{Row, View},
    ui::{prelude::*, state::mode::DashboardPane},
};

/// Draws the option tree of the entry picked in the apps pane.
pub(super) fn render_options(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let view = state.dashboard.view(&state.app);
    let whose = match view {
        View::Repo => "repo-wide".to_string(),
        View::App(at) => format!("app {}", state.app.apps[at].name),
    };
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("options", SECONDARY).bold(),
        Span::styled(format!(" · {whose} "), SECONDARY),
    ]);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .padding(Padding::new(1, 1, 1, 0))
        .border_style(super::border(state, DashboardPane::Options));
    let rows = state.app.rows(view);
    let conflicted = state
        .dashboard
        .options
        .selected()
        .and_then(|at| rows.get(at))
        .is_some_and(|row| row.conflicted);
    let highlight_style = if conflicted {
        Style::new().fg(ERROR).bg(ASCENT_BIS).bold()
    } else {
        Style::new().fg(ASCENT).bg(ASCENT_BIS).bold()
    };
    let list = List::new(rows.iter().map(line))
        .block(block)
        .style(SECONDARY)
        .highlight_style(highlight_style)
        .highlight_symbol(Span::styled("▌ ", ASCENT))
        .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(list, chunk, &mut state.dashboard.options);
}

/// One option, indented by depth: `▬` checked, `▭` checked but waiting on
/// its parent, `✕` ruled out.
fn line(row: &Row) -> Line<'static> {
    let mark = if row.idle {
        Span::styled("▭ ", HIGHLIGHT)
    } else if row.checked {
        Span::styled("▬ ", ASCENT)
    } else if row.conflicted {
        Span::styled("✕ ", ERROR)
    } else {
        Span::raw("  ")
    };
    let name = <&'static str>::from(row.id);
    let name = if row.checked {
        Span::styled(name, PRIMARY)
    } else {
        Span::raw(name)
    };
    Line::from(vec![Span::raw("  ".repeat(row.depth)), mark, name])
}
