use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, HighlightSpacing, List, ListState, Padding, Paragraph, Wrap},
};
use strum::IntoEnumIterator;

use crate::{
    app::{APPS_DIR, View, opt::Scope},
    ui::{prelude::*, state::mode::Mode},
};

/// Draws the sidebar in three equal cells: what the repo will be, the pages,
/// and a word on what's under the cursor.
pub(super) fn render(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let [summary, navigate, about] = Layout::vertical([Constraint::Ratio(1, 3); 3]).areas(chunk);
    render_navigate(state, frame, navigate);
    render_summary(state, frame, summary);
    render_about(state, frame, about);
}

/// A rounded pane titled `title`, accented when `focused`.
fn pane(title: &str, focused: bool) -> Block<'_> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title(Line::from(vec![
            Span::raw(" "),
            Span::styled(title, if focused { ASCENT } else { SECONDARY }).bold(),
            Span::raw(" "),
        ]))
        .padding(Padding::new(1, 1, 0, 0))
        .border_style(if focused { ASCENT } else { ASCENT_BIS })
}

/// Page list, with the current page selected.
fn render_navigate(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let block = pane("pages", state.focus == Focus::Sidebar);
    let items = Mode::iter().map(|mode| match mode {
        Mode::Dashboard => "build",
        Mode::Details => "preview files",
    });
    let list = List::new(items)
        .block(block)
        .style(SECONDARY)
        .highlight_style(Style::new().fg(ASCENT).bg(ASCENT_BIS).bold())
        .highlight_symbol(Span::styled("▌ ", ASCENT))
        .highlight_spacing(HighlightSpacing::Always);
    let mut list_state = ListState::default().with_selected(Some(state.mode as usize));
    frame.render_stateful_widget(list, chunk, &mut list_state);
}

/// What `c` would create: where, which layout, how many files.
fn render_summary(state: &State, frame: &mut Frame, chunk: Rect) {
    let app = &state.app;
    let files = app
        .staged
        .as_ref()
        .map_or(0, std::collections::BTreeMap::len);
    let layout = if app.is_multi() {
        format!("{} apps in {APPS_DIR}/", app.apps.len())
    } else {
        "one app at the root".into()
    };
    let target = app.dest.join(&app.name);
    let row = |key: &'static str, value: String| {
        Line::from(vec![
            Span::styled(format!("{key:<7}"), SECONDARY),
            Span::styled(value, PRIMARY),
        ])
    };
    let mut lines = vec![
        row("repo", app.name.clone()),
        row("layout", layout),
        row("files", files.to_string()),
        row("into", target.display().to_string()),
    ];
    if let Some((a, b)) = app.conflict() {
        let (a, b): (&str, &str) = (a.into(), b.into());
        lines.push(Line::styled(format!("✕ {a} vs {b}"), ERROR));
    }
    let paragraph = Paragraph::new(Text::from(lines))
        .block(pane("summary", false))
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, chunk);
}

/// A few lines on what the cursor is on, so the screen explains itself.
fn render_about(state: &State, frame: &mut Frame, chunk: Rect) {
    let paragraph = Paragraph::new(about(state))
        .block(pane("about", false))
        .style(SECONDARY)
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, chunk);
}

/// The text of the about pane.
fn about(state: &State) -> Text<'static> {
    let app = &state.app;
    let dashboard = &state.dashboard;
    if state.focus == Focus::Sidebar {
        return Text::from(vec![
            Line::from("build: pick the apps and their options."),
            Line::from(""),
            Line::from("preview files: read every file before creating the repo."),
        ]);
    }
    if state.mode == Mode::Details {
        return Text::from(
            "Every file the repo will hold. Nothing is written until you create it.",
        );
    }
    match dashboard.focus {
        DashboardPane::Apps => Text::from(vec![
            Line::from("repo: tooling every app shares (CI, flake, just, checks)."),
            Line::from(""),
            Line::from("One app: the repo is that app."),
            Line::from(format!(
                "Two or more: each goes in {APPS_DIR}/<name>/; Rust apps share a Cargo workspace."
            )),
        ]),
        DashboardPane::Options => {
            let view = dashboard.view(app);
            let Some(row) = dashboard
                .options
                .selected()
                .and_then(|at| app.rows(view).into_iter().nth(at))
            else {
                return Text::from("Nothing to pick here.");
            };
            let def = row.id.def();
            let name: &'static str = row.id.into();
            let mut lines = vec![
                Line::styled(name, PRIMARY).bold(),
                Line::from(def.desc),
                Line::from(""),
            ];
            lines.push(Line::from(match (def.scope, view) {
                (Scope::Repo, _) => "Repo-wide: one for every app.".to_string(),
                (Scope::App, View::App(at)) => format!("For the app `{}` only.", app.apps[at].name),
                (Scope::App, View::Repo) => String::new(),
            }));
            if row.conflicted {
                lines.push(Line::styled("Ruled out by a checked option.", ERROR));
            } else if row.idle {
                lines.push(Line::styled(
                    "Checked, but its parent or a requirement is off.",
                    HIGHLIGHT,
                ));
            }
            Text::from(lines)
        }
        DashboardPane::Form => Text::from(vec![
            Line::from("owner: GitHub account, used in links."),
            Line::from("repo: folder and repo name."),
            Line::from("desc: one line, in the README and manifests."),
            Line::from("dest: where the repo folder is created."),
        ]),
    }
}
