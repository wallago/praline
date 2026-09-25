use std::{cell::LazyCell, ffi::OsStr, path::Path};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::Paragraph,
};
use syntect_assets::assets::HighlightingAssets;
use tui_syntax_highlight::Highlighter;

use super::spread;
use crate::ui::{
    placeholder::{PRESETS, TOOLS},
    state::{File, Pane, State, Subject},
    theme::{self, BLUE, CYAN, GRAY, GREEN, PURPLE},
};

thread_local! {
    /// Syntax definitions and themes, loaded on first use.
    static ASSETS: LazyCell<HighlightingAssets> = LazyCell::new(HighlightingAssets::from_binary);
}

/// `[4]-Preview`: the selected tool's details, or the selected file's body.
pub(super) fn render(state: &mut State, frame: &mut Frame, area: Rect) {
    let block = theme::pane("[4]-Preview", state.focus == Pane::Preview);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let [header, _, body] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(inner);

    let (left, right, text) = match (state.subject, state.file()) {
        (Subject::File, Some(file)) => file_preview(state, file),
        (Subject::File, None) => (
            Line::styled("nothing to preview", GRAY),
            Line::default(),
            Text::default(),
        ),
        (Subject::Tool, _) => tool_preview(state),
    };
    spread(frame, header, left, right);

    let overflow = text.height().saturating_sub(usize::from(body.height));
    state.scroll = state
        .scroll
        .min(u16::try_from(overflow).unwrap_or(u16::MAX));
    frame.render_widget(Paragraph::new(text).scroll((state.scroll, 0)), body);
}

/// Header and highlighted body of `file`.
fn file_preview(state: &State, file: File) -> (Line<'static>, Line<'static>, Text<'static>) {
    let tool = TOOLS.get(file.tool).map_or(Span::raw(""), |tool| {
        Span::styled(tool.name(), theme::category(tool.category))
    });
    (
        Line::from(file.path).bold(),
        Line::from(vec![Span::styled("written by ", GRAY), tool]),
        highlight(file.path, &state.body(file)),
    )
}

/// Header and details of the tool selected in `[2]`.
fn tool_preview(state: &State) -> (Line<'static>, Line<'static>, Text<'static>) {
    let index = state.tool();
    let Some(tool) = TOOLS.get(index) else {
        return (Line::default(), Line::default(), Text::default());
    };
    let status = match (state.is_checked(index), state.blocker(index)) {
        (false, _) => Span::styled("○ off", GRAY),
        (true, None) => Span::styled("● on", GREEN),
        (true, Some(blocker)) => {
            Span::styled(format!("● on, inactive until {blocker} is on"), PURPLE)
        }
    };
    let presets: Vec<&str> = PRESETS
        .iter()
        .filter(|preset| preset.includes(tool.id))
        .map(|preset| preset.name)
        .collect();

    let mut lines = vec![Line::raw(tool.desc), Line::default()];
    field(&mut lines, "STATUS", [status]);
    if let Some(parent) = tool.parent() {
        field(&mut lines, "PARENT", [Span::raw(parent)]);
    }
    if let Some(needs) = tool.needs {
        field(&mut lines, "NEEDS", [Span::raw(needs)]);
    }
    if tool.writes.is_empty() {
        let flag = format!("nothing, it only switches {{if:{}}} blocks", tool.id);
        field(&mut lines, "WRITES", [Span::styled(flag, GRAY)]);
    } else {
        let paths = tool.writes.iter().map(|path| Span::styled(*path, CYAN));
        field(&mut lines, "WRITES", paths);
    }
    if !tool.also_in.is_empty() {
        let paths = tool.also_in.iter().map(|path| Span::styled(*path, CYAN));
        field(&mut lines, "ALSO IN", paths);
    }
    let presets = if presets.is_empty() {
        Span::styled("none", GRAY)
    } else {
        Span::raw(presets.join(" · "))
    };
    field(&mut lines, "PRESETS", [presets]);

    (
        Line::from(tool.id).bold(),
        Line::styled(tool.category.label(), theme::category(tool.category)),
        Text::from(lines),
    )
}

/// Pushes `LABEL     value` rows; extra values get their own indented rows.
fn field(
    lines: &mut Vec<Line<'static>>,
    label: &'static str,
    values: impl IntoIterator<Item = Span<'static>>,
) {
    for (at, value) in values.into_iter().enumerate() {
        let label = if at == 0 { label } else { "" };
        lines.push(Line::from(vec![
            Span::styled(format!("{label:<10}"), BLUE),
            value,
        ]));
    }
}

/// Colors `content` by the syntax its path suggests, with line numbers.
fn highlight(path: &str, content: &str) -> Text<'static> {
    ASSETS.with(|assets| {
        let Ok(syntaxes) = assets.get_syntax_set() else {
            return Text::raw(content.to_owned());
        };
        let path = Path::new(path);
        let by_suffix = |name: Option<&OsStr>| {
            name.and_then(OsStr::to_str)
                .and_then(|name| syntaxes.find_syntax_by_extension(name))
        };
        let syntax = by_suffix(path.extension())
            .or_else(|| by_suffix(path.file_name()))
            .or_else(|| {
                content
                    .lines()
                    .next()
                    .and_then(|line| syntaxes.find_syntax_by_first_line(line))
            })
            .unwrap_or_else(|| syntaxes.find_syntax_plain_text());
        Highlighter::new(assets.get_theme("ansi").clone())
            .line_number_style(Style::new().fg(GRAY))
            .line_number_separator_style(Style::new().fg(GRAY))
            .highlight_reader(content.as_bytes(), syntax, syntaxes)
            .unwrap_or_else(|_| Text::raw(content.to_owned()))
    })
}
