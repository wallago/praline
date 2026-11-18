use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Clear, Padding, Paragraph},
};

use crate::{
    app::{APPS_DIR, App, View, opt::Scope},
    prelude::OptId,
    ui::prelude::*,
};

/// Draws the generate dialog over the page while it's open: a summary, a
/// panel for the repo and one per app, each listing its options, then the
/// question.
pub(super) fn render_confirm(state: &State, frame: &mut Frame) {
    if !state.confirm {
        return;
    }
    let app = &state.app;
    let summary = summary_lines(app);
    let summary_height = u16::try_from(summary.len())
        .unwrap_or(u16::MAX)
        .saturating_add(2);
    let mut panels = vec![("repo".to_string(), repo_lines(app))];
    panels.extend((0..app.apps.len()).map(|at| (app_title(app, at), app_lines(app, at))));
    let tallest = panels
        .iter()
        .map(|(_, lines)| lines.len())
        .max()
        .unwrap_or(0);
    // The panels' borders and lines, then a blank line and the question.
    let panels_height = u16::try_from(tallest).unwrap_or(u16::MAX).saturating_add(2);
    // The outer border and padding, the summary, the panels, a blank line and
    // the question.
    let height = 3 + summary_height + panels_height + 1 + 1;
    let area = frame
        .area()
        .centered(Constraint::Percentage(80), Constraint::Length(height));
    let outer = titled("generate", true).padding(Padding::new(2, 2, 1, 0));
    let inner = outer.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(outer.bg(BACKGROUND), area);

    let [summary_area, row, _, question] = Layout::vertical([
        Constraint::Length(summary_height),
        Constraint::Length(panels_height),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(inner);
    frame.render_widget(
        Paragraph::new(Text::from(summary)).block(titled("summary", false)),
        summary_area,
    );
    let count = u32::try_from(panels.len()).unwrap_or(u32::MAX);
    let areas = Layout::horizontal(panels.iter().map(|_| Constraint::Ratio(1, count)))
        .spacing(1)
        .split(row);
    for ((title, lines), &area) in panels.into_iter().zip(areas.iter()) {
        frame.render_widget(
            Paragraph::new(Text::from(lines)).block(titled(&title, false)),
            area,
        );
    }
    render_question(app, frame, question);
}

/// A rounded box titled `title`, accented when `strong`.
fn titled(title: &str, strong: bool) -> Block<'_> {
    let (border, text) = if strong {
        (ASCENT, ASCENT)
    } else {
        (ASCENT_BIS, SECONDARY)
    };
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(border)
        .title(Line::from(vec![
            Span::raw(" "),
            Span::styled(title.to_string(), text).bold(),
            Span::raw(" "),
        ]))
        .padding(Padding::horizontal(1))
}

/// `key` in the muted column, then `value`.
fn row(key: &'static str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{key:<7}"), SECONDARY),
        Span::styled(value, PRIMARY),
    ])
}

/// What generating writes: the repo, how its apps are laid out, how many
/// files, and where.
fn summary_lines(app: &App) -> Vec<Line<'static>> {
    let files = app
        .staged
        .as_ref()
        .map_or(0, std::collections::BTreeMap::len);
    let workspace =
        (0..app.apps.len()).any(|at| app.checked(View::App(at)).contains(&OptId::RustCommon));
    let layout = match (app.is_multi(), workspace) {
        (false, _) => "one app, at the root".into(),
        (true, true) => format!(
            "{} apps in {APPS_DIR}/, one Cargo workspace",
            app.apps.len()
        ),
        (true, false) => format!("{} apps in {APPS_DIR}/", app.apps.len()),
    };
    vec![
        row("repo", app.name.clone()),
        row("layout", layout),
        row("files", files.to_string()),
        row("into", app.dest.join(&app.name).display().to_string()),
    ]
}

/// Who the repo is for, then its repo-wide options.
fn repo_lines(app: &App) -> Vec<Line<'static>> {
    let mut lines = vec![
        row("owner", app.owner.clone()),
        row("desc", app.desc.clone()),
        Line::default(),
    ];
    lines.extend(options(app, View::Repo, Scope::Repo));
    lines
}

/// An app panel's title: where the app goes.
fn app_title(app: &App, at: usize) -> String {
    if app.is_multi() {
        format!("{APPS_DIR}/{}/", app.apps[at].name)
    } else {
        "app · ./".into()
    }
}

/// An app's own options.
fn app_lines(app: &App, at: usize) -> Vec<Line<'static>> {
    options(app, View::App(at), Scope::App)
}

/// The options checked in `view` that `scope` holds, one per line and
/// indented like the option tree, or "nothing" when there are none.
fn options(app: &App, view: View, scope: Scope) -> Vec<Line<'static>> {
    let rows: Vec<_> = app
        .rows(view)
        .into_iter()
        .filter(|row| row.checked && !row.idle && row.id.def().scope == scope)
        .collect();
    let Some(top) = rows.iter().map(|row| row.depth).min() else {
        return vec![Line::styled("nothing", SECONDARY).italic()];
    };
    rows.into_iter()
        .map(|row| {
            let name: &'static str = row.id.into();
            Line::from(vec![
                Span::raw("  ".repeat(row.depth - top)),
                Span::styled("▬ ", ASCENT),
                Span::styled(name, PRIMARY),
            ])
        })
        .collect()
}

/// "Generate <name>?" with its keys, or why the repo can't be generated.
fn render_question(app: &App, frame: &mut Frame, area: Rect) {
    let target = app.dest.join(&app.name);
    let blocker = if let Some((a, b)) = app.conflict() {
        let (a, b): (&str, &str) = (a.into(), b.into());
        Some(format!("✕ {a} conflicts with {b}"))
    } else if target.exists() {
        Some(format!("✕ {} already exists", target.display()))
    } else {
        None
    };
    let line = if let Some(blocker) = blocker {
        Line::styled(blocker, ERROR).bold()
    } else {
        Line::from(vec![
            Span::styled("Generate ", PRIMARY),
            Span::styled(app.name.clone(), ASCENT).bold(),
            Span::styled("?  ", PRIMARY),
            Span::styled("y", ASCENT).bold(),
            Span::styled(" yes  ", SECONDARY),
            Span::styled("n", ASCENT).bold(),
            Span::styled(" no", SECONDARY),
        ])
    };
    frame.render_widget(Paragraph::new(line.centered()), area);
}
