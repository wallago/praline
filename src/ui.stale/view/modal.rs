use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Cell, Clear, Padding, Paragraph, Row, Table, TableState},
};
use tui_input::Input;

use crate::ui::{
    placeholder::PRESETS,
    state::{Modal, State},
    theme::{self, CYAN, GRAY, GREEN},
};

/// Draws the open dialog, if any, over the panes.
pub(super) fn render(state: &mut State, frame: &mut Frame) {
    let Some(mut modal) = state.modal.take() else {
        return;
    };
    match &mut modal {
        Modal::Repo { fields, focus } => render_repo(state, frame, fields, *focus),
        Modal::Preset(table) => render_preset(state, frame, table),
        Modal::Generate(input) => render_generate(state, frame, input),
        Modal::Help => render_help(state, frame),
    }
    state.modal = Some(modal);
}

/// Clears a centered box and draws the dialog frame; returns its inside.
fn dialog(frame: &mut Frame, title: &str, width: u16, height: u16) -> Rect {
    let area = frame
        .area()
        .centered(Constraint::Length(width), Constraint::Length(height));
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(GREEN)
        .title_top(Line::from(format!(" {title} ")).bold().centered())
        .padding(Padding::horizontal(1));
    let inner = block.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    inner
}

/// A labelled input box; cyan and holding the cursor when focused.
fn field(frame: &mut Frame, area: Rect, label: &str, input: &Input, focused: bool) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(if focused { CYAN } else { GREEN })
        .title_top(Span::styled(format!(" {label} "), CYAN))
        .padding(Padding::horizontal(1));
    let inner = block.inner(area);
    let scroll = input.visual_scroll(usize::from(inner.width.saturating_sub(1)));
    let offset = u16::try_from(scroll).unwrap_or(u16::MAX);
    frame.render_widget(
        Paragraph::new(input.value())
            .scroll((0, offset))
            .block(block),
        area,
    );
    if focused {
        let cursor = u16::try_from(input.visual_cursor()).unwrap_or(u16::MAX);
        frame.set_cursor_position((inner.x + cursor.saturating_sub(offset), inner.y));
    }
}

/// A gray hint line for the bottom of a dialog.
fn hints(pairs: &[(String, &'static str)]) -> Line<'static> {
    Line::from(theme::hints(pairs))
}

/// Owner, name and description, one box each.
fn render_repo(state: &State, frame: &mut Frame, fields: &[Input; 3], focus: usize) {
    let inner = dialog(frame, "Repo", 48, 15);
    let [_, owner, name, desc, _, keys] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(inner);
    let labels = ["Owner", "Name", "Description"];
    for (at, (area, (label, input))) in [owner, name, desc]
        .into_iter()
        .zip(labels.into_iter().zip(fields))
        .enumerate()
    {
        field(frame, area, label, input, at == focus);
    }
    let pairs = [
        (String::from("tab"), "field"),
        (state.keys.confirm.to_string(), "save"),
        (state.keys.leave.to_string(), "cancel"),
    ];
    frame.render_widget(hints(&pairs), keys);
}

/// Preset table; `✓` marks the one the current selection matches.
fn render_preset(state: &State, frame: &mut Frame, table: &mut TableState) {
    let inner = dialog(frame, "Preset", 58, 11);
    let [_, list, _, keys] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(5),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(inner);
    let current = state.preset().map(|preset| preset.name);
    let header = Row::new([
        Cell::from(""),
        Cell::from("PRESET"),
        Cell::from(Line::from("TOOLS").right_aligned()),
        Cell::from("DESCRIPTION"),
    ])
    .style(Style::new().fg(GREEN).bold());
    let rows = PRESETS.iter().map(|preset| {
        let mark = if current == Some(preset.name) {
            "✓"
        } else {
            ""
        };
        Row::new([
            Cell::from(Span::styled(mark, GREEN)),
            Cell::from(preset.name),
            Cell::from(Line::from(preset.count().to_string()).right_aligned()),
            Cell::from(Span::styled(preset.desc, GRAY)),
        ])
    });
    let widths = [
        Constraint::Length(1),
        Constraint::Length(8),
        Constraint::Length(5),
        Constraint::Fill(1),
    ];
    frame.render_stateful_widget(
        Table::new(rows, widths)
            .header(header)
            .column_spacing(2)
            .row_highlight_style(theme::selection(true)),
        list,
        table,
    );
    let pairs = [
        (state.keys.confirm.to_string(), "apply"),
        (state.keys.leave.to_string(), "cancel"),
    ];
    frame.render_widget(hints(&pairs), keys);
}

/// Destination prompt and what would be written.
fn render_generate(state: &State, frame: &mut Frame, input: &Input) {
    let inner = dialog(frame, "Generate", 58, 10);
    let [_, destination, summary, _, keys] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(inner);
    field(frame, destination, "Destination", input, true);
    let summary_line = format!(
        " {} files from {} tools, nothing is written yet",
        state.files().len(),
        state.active()
    );
    frame.render_widget(Line::styled(summary_line, GRAY), summary);
    let pairs = [
        (state.keys.confirm.to_string(), "write"),
        (state.keys.leave.to_string(), "cancel"),
    ];
    frame.render_widget(hints(&pairs), keys);
}

/// Every key, grouped by where it works.
fn render_help(state: &State, frame: &mut Frame) {
    let keys = &state.keys;
    let moves = format!("{} {}  ↓ ↑", keys.scroll_down, keys.scroll_up);
    let groups = [
        (
            "ANYWHERE",
            vec![
                (String::from("tab  ⇧tab"), "cycle panes"),
                (String::from("1 2 3 4"), "jump to a pane"),
                (String::from("r"), "edit the repo"),
                (String::from("p"), "pick a preset"),
                (keys.generate.to_string(), "generate"),
                (String::from("?"), "this help"),
                (keys.quit.to_string(), "quit"),
            ],
        ),
        (
            "LISTS",
            vec![
                (moves.clone(), "move"),
                (String::from("space"), "toggle a tool"),
                (format!("{}  →", keys.scroll_right), "preview"),
            ],
        ),
        (
            "PREVIEW",
            vec![
                (moves, "scroll"),
                (String::from("pgup pgdn"), "scroll a page"),
                (format!("{}  ←", keys.scroll_left), "back to the list"),
            ],
        ),
    ];
    let mut lines = Vec::new();
    for (title, pairs) in groups {
        if !lines.is_empty() {
            lines.push(Line::default());
        }
        lines.push(Line::styled(title, GREEN).bold());
        for (key, what) in pairs {
            lines.push(Line::from(vec![
                Span::styled(format!("  {key:<12}"), CYAN).bold(),
                Span::raw(what),
            ]));
        }
    }
    let height = u16::try_from(lines.len())
        .unwrap_or(u16::MAX)
        .saturating_add(4);
    let inner = dialog(frame, "Keys", 40, height);
    let [body, _, footer] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(inner);
    frame.render_widget(Paragraph::new(lines), body);
    frame.render_widget(hints(&[(String::from("any key"), "close")]), footer);
}
