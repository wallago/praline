use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph, Wrap},
};

use crate::app::tool::category::Category;
use crate::tui::{state::State, ui::utils::badge};

/// Right column: read-only summary derived from the form.
pub(super) fn render_preview(state: &State, frame: &mut Frame, rect: Rect) {
    let inner = rect.inner(Margin {
        vertical: 1,
        horizontal: 2,
    });
    let [identity, selection, groups, status] = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Min(0),
        Constraint::Length(3),
    ])
    .areas(inner);

    render_identity(state, frame, identity);
    render_selection(state, frame, selection);
    render_groups(state, frame, groups);
    render_status(state, frame, status);
}

/// Repo identity: `owner/name` over the description.
fn render_identity(state: &State, frame: &mut Frame, rect: Rect) {
    let mut path = Vec::new();
    if state.repo.owner.is_empty() {
        path.push(Span::raw("<owner>").fg(Color::DarkGray));
    } else {
        path.push(Span::raw(state.repo.owner.as_str()).fg(Color::Gray));
    }
    path.push(Span::raw("/").fg(Color::DarkGray));
    if state.repo.name.is_empty() {
        path.push(Span::raw("<name>").fg(Color::DarkGray));
    } else {
        path.push(
            Span::raw(state.repo.name.as_str())
                .fg(state.accent_color)
                .bold(),
        );
    }

    let desc = if state.repo.desc.is_empty() {
        Line::from(Span::raw("no description yet").fg(Color::DarkGray).italic())
    } else {
        Line::from(Span::raw(state.repo.desc.as_str()))
    };

    let block = Block::bordered()
        .title(String::from(" Repo ").fg(Color::Gray).bold())
        .padding(Padding::horizontal(1));
    frame.render_widget(
        Paragraph::new(vec![Line::from(path), Line::from(""), desc])
            .block(block)
            .wrap(Wrap { trim: true }),
        rect,
    );
}

/// Which preset is applied and how much of the tool list it covers.
fn render_selection(state: &State, frame: &mut Frame, rect: Rect) {
    let total = state.repo.options.len();
    let checked = state.repo.options.iter().filter(|opt| opt.checked).count();

    let preset = match state.repo.active_preset() {
        Some(preset) => Line::from(vec![
            Span::raw(preset.name()).fg(Color::Green).bold(),
            Span::raw(" — ").fg(Color::DarkGray),
            Span::raw(preset.desc()).fg(Color::Gray),
        ]),
        None => Line::from(vec![
            Span::raw("custom").fg(Color::Yellow).bold(),
            Span::raw(" — ").fg(Color::DarkGray),
            Span::raw("hand-picked selection").fg(Color::Gray),
        ]),
    };

    /// Width of the fill bar, in cells.
    const BAR: usize = 20;
    let filled = if total == 0 {
        0
    } else {
        checked.saturating_mul(BAR) / total
    };
    let bar = Line::from(vec![
        Span::raw("█".repeat(filled)).fg(Color::Green),
        Span::raw("░".repeat(BAR.saturating_sub(filled))).fg(Color::DarkGray),
        Span::raw(format!("  {checked}/{total} tools")).fg(Color::Gray),
    ]);

    let block = Block::bordered()
        .title(String::from(" Preset ").fg(Color::Gray).bold())
        .padding(Padding::horizontal(1));
    frame.render_widget(
        Paragraph::new(vec![preset, Line::from(""), bar]).block(block),
        rect,
    );
}

/// Selected tools, grouped under the category badge they carry.
fn render_groups(state: &State, frame: &mut Frame, rect: Rect) {
    let mut lines = Vec::new();
    for category in Category::ALL {
        let tools: Vec<String> = state
            .repo
            .options
            .iter()
            .filter(|opt| opt.checked && opt.tool.category() == category)
            .map(|opt| opt.tool.name())
            .collect();
        if tools.is_empty() {
            continue;
        }
        lines.push(Line::from(vec![
            badge(&category),
            Span::raw(" "),
            Span::raw(tools.join(" ")).fg(Color::Gray),
        ]));
    }
    if lines.is_empty() {
        lines.push(Line::from(
            Span::raw("nothing selected").fg(Color::DarkGray).italic(),
        ));
    }

    let block = Block::bordered()
        .title(String::from(" Selected ").fg(Color::Gray).bold())
        .padding(Padding::horizontal(1));
    frame.render_widget(
        Paragraph::new(lines).block(block).wrap(Wrap { trim: true }),
        rect,
    );
}

/// Whether the form can generate yet, and what is missing if not.
///
/// Mirrors the conditions in [`RepoBuilder::check`]; keep the two in step.
///
/// [`RepoBuilder::check`]: crate::app::RepoBuilder::check
fn render_status(state: &State, frame: &mut Frame, rect: Rect) {
    let mut missing = Vec::new();
    if state.repo.name.is_empty() {
        missing.push("name");
    }
    if state.repo.desc.is_empty() {
        missing.push("desc");
    }
    if !state.repo.options.iter().any(|opt| opt.checked) {
        missing.push("a tool");
    }

    let line = if missing.is_empty() {
        Line::from(vec![
            Span::raw(" ready ")
                .bg(Color::Green)
                .fg(Color::Black)
                .bold(),
            Span::raw("  press ").fg(Color::DarkGray),
            Span::raw(state.keybindings.generate.to_string())
                .fg(Color::Yellow)
                .bold(),
            Span::raw(" to generate").fg(Color::DarkGray),
        ])
    } else {
        Line::from(vec![
            Span::raw(" missing ")
                .bg(Color::Red)
                .fg(Color::Black)
                .bold(),
            Span::raw(format!("  {}", missing.join(", "))).fg(Color::Gray),
        ])
    };

    frame.render_widget(
        Paragraph::new(line).block(Block::bordered().padding(Padding::horizontal(1))),
        rect,
    );
}
