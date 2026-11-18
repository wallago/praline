use ratatui::{
    Frame,
    layout::Constraint,
    style::Stylize,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Clear, Padding, Paragraph, Wrap},
};

use crate::{
    app::{APPS_DIR, App, View, opt::Scope},
    prelude::OptId,
    ui::prelude::*,
};

/// Draws the create dialog over the page while it's open: where the repo
/// goes, its apps and the repo-wide options, then how to go on.
pub(super) fn render_confirm(state: &State, frame: &mut Frame) {
    if !state.confirm {
        return;
    }
    let area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Percentage(70));
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("create repo", ASCENT).bold(),
        Span::raw(" "),
    ]);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .padding(Padding::new(2, 2, 1, 1))
        .border_style(ASCENT)
        .bg(BACKGROUND);
    let paragraph = Paragraph::new(summary(&state.app))
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

/// `key` in the muted column, then `value`.
fn row(key: &'static str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{key:<10}"), SECONDARY),
        Span::styled(value, PRIMARY),
    ])
}

/// The options checked in `view` that `scope` holds, by name.
fn names(app: &App, view: View, scope: Scope) -> String {
    let names: Vec<&str> = app
        .rows(view)
        .into_iter()
        .filter(|row| row.checked && !row.idle && row.id.def().scope == scope)
        .map(|row| <&'static str>::from(row.id))
        .collect();
    if names.is_empty() {
        "none".into()
    } else {
        names.join(", ")
    }
}

/// The dialog's text.
fn summary(app: &App) -> Text<'static> {
    let files = app
        .staged
        .as_ref()
        .map_or(0, std::collections::BTreeMap::len);
    let target = app.dest.join(&app.name);
    let mut lines = vec![
        row("into", target.display().to_string()),
        row("files", files.to_string()),
        row("owner", app.owner.clone()),
        row("desc", app.desc.clone()),
        Line::from(""),
    ];
    let workspace = app.is_multi()
        && (0..app.apps.len()).any(|at| app.checked(View::App(at)).contains(&OptId::RustCommon));
    lines.push(row(
        "layout",
        match (app.is_multi(), workspace) {
            (false, _) => "one app at the root".into(),
            (true, true) => format!(
                "{} apps in {APPS_DIR}/, Rust ones in a Cargo workspace",
                app.apps.len()
            ),
            (true, false) => format!("{} apps in {APPS_DIR}/", app.apps.len()),
        },
    ));
    for (at, unit) in app.apps.iter().enumerate() {
        let place = if app.is_multi() {
            format!("{APPS_DIR}/{}/", unit.name)
        } else {
            "./".into()
        };
        lines.push(Line::from(vec![
            Span::styled("  ▬ ", ASCENT),
            Span::styled(format!("{place:<16}"), PRIMARY),
            Span::styled(names(app, View::App(at), Scope::App), SECONDARY),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(row("shared", names(app, View::Repo, Scope::Repo)));
    lines.push(Line::from(""));
    if let Some((a, b)) = app.conflict() {
        let (a, b): (&str, &str) = (a.into(), b.into());
        lines.push(Line::styled(
            format!("✕ {a} conflicts with {b}: go back and untick one"),
            ERROR,
        ));
    } else if target.exists() {
        lines.push(Line::styled(
            format!(
                "✕ {} already exists: change the name or dest",
                target.display()
            ),
            ERROR,
        ));
    } else {
        lines.push(Line::styled("⏎ create and quit   Esc back", HIGHLIGHT));
    }
    Text::from(lines)
}
