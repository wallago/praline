use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Cell, Row, Table},
};

use crate::ui::{
    placeholder::{TOOLS, Tool},
    state::{File, Pane, State},
    theme::{self, GRAY, GREEN, LIME},
};

/// Width from which `[2]` also shows each tool's blurb.
const WIDE: u16 = 72;
/// Width of `[3]`'s tool column: the longest tool name.
const TOOL_WIDTH: u16 = 12;

/// `[2]-Tools`: every tool as a tree, with its dot, category and file count.
pub(super) fn render_tools(state: &mut State, frame: &mut Frame, area: Rect) {
    let focused = state.focus == Pane::Tools;
    let mut widths = vec![
        Constraint::Length(1),
        Constraint::Length(14),
        Constraint::Length(8),
        Constraint::Length(5),
    ];
    if area.width >= WIDE {
        widths.push(Constraint::Fill(1));
    }
    let header = Row::new([
        Cell::from(""),
        Cell::from("TOOL"),
        Cell::from("CATEGORY"),
        Cell::from(Line::from("FILES").right_aligned()),
        Cell::from("DESCRIPTION"),
    ])
    .style(Style::new().fg(GREEN).bold());
    let rows = TOOLS
        .iter()
        .enumerate()
        .map(|(index, tool)| tool_row(state, index, tool));
    let block = theme::pane("[2]-Tools", focused)
        .title_bottom(theme::counter(state.tools.selected(), TOOLS.len()));
    let table = Table::new(rows, widths)
        .header(header)
        .column_spacing(2)
        .row_highlight_style(theme::selection(focused))
        .block(block);
    frame.render_stateful_widget(table, area, &mut state.tools);
}

/// One `[2]` row; everything but the category goes gray while inactive.
fn tool_row(state: &State, index: usize, tool: &Tool) -> Row<'static> {
    let active = state.is_active(index);
    let dot = match (state.is_checked(index), active) {
        (true, true) => Span::styled("●", GREEN),
        (true, false) => Span::styled("●", GRAY),
        (false, _) => Span::styled("○", GRAY),
    };
    let text = if active {
        Style::new()
    } else {
        Style::new().fg(GRAY)
    };
    let mut name = Vec::new();
    if let Some(parent) = tool.parent() {
        let last = TOOLS
            .get(index + 1)
            .is_none_or(|next| next.parent() != Some(parent));
        name.push(Span::styled(if last { "└─ " } else { "├─ " }, GRAY));
    }
    name.push(Span::styled(tool.name(), text));
    let files = if tool.writes.is_empty() {
        Span::styled("flag", GRAY)
    } else {
        Span::styled(tool.writes.len().to_string(), text)
    };
    Row::new([
        Cell::from(dot),
        Cell::from(Line::from(name)),
        Cell::from(Span::styled(
            tool.category.label(),
            theme::category(tool.category),
        )),
        Cell::from(Line::from(files).right_aligned()),
        Cell::from(Span::styled(tool.desc, GRAY)),
    ])
}

/// `[3]-Files`: what the active tools write; the selected tool's are lime.
pub(super) fn render_files(state: &mut State, frame: &mut Frame, area: Rect) {
    let focused = state.focus == Pane::Files;
    let files = state.files();
    let tool = state.tool();
    // Borders, padding, the tool column and the gap before it.
    let room = usize::from(area.width.saturating_sub(4 + TOOL_WIDTH + 1));
    let rows = files
        .iter()
        .map(|file| file_row(*file, file.tool == tool, room));
    let block = theme::pane("[3]-Files", focused)
        .title_bottom(theme::counter(state.files.selected(), files.len()));
    if files.is_empty() {
        let inner = block.inner(area);
        frame.render_widget(block, area);
        frame.render_widget(
            Line::styled("nothing to write, turn a tool on", GRAY),
            inner,
        );
        return;
    }
    let widths = [Constraint::Fill(1), Constraint::Length(TOOL_WIDTH)];
    let table = Table::new(rows, widths)
        .row_highlight_style(theme::selection(focused))
        .block(block);
    frame.render_stateful_widget(table, area, &mut state.files);
}

/// One `[3]` row: dimmed directory, file name, writing tool. A path longer
/// than `room` loses its start, so the file name stays visible.
fn file_row(file: File, owned: bool, room: usize) -> Row<'static> {
    let length = file.path.chars().count();
    let path = if length > room {
        let kept: String = file.path.chars().skip(length + 1 - room).collect();
        format!("…{kept}")
    } else {
        file.path.to_owned()
    };
    let (dir, name) = path
        .rfind('/')
        .map_or(("", path.as_str()), |at| path.split_at(at + 1));
    let name = if owned {
        Span::styled(name.to_owned(), LIME).bold()
    } else {
        Span::raw(name.to_owned())
    };
    let tool = TOOLS.get(file.tool).map_or(Span::raw(""), |tool| {
        Span::styled(tool.name(), theme::category(tool.category))
    });
    Row::new([
        Cell::from(Line::from(vec![Span::styled(dir.to_owned(), GRAY), name])),
        Cell::from(Line::from(tool).right_aligned()),
    ])
}
