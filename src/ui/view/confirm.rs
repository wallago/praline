use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Clear, Padding, Paragraph, Wrap},
};

use crate::{
    app::{APPS_DIR, App, View, opt::Scope},
    prelude::OptId,
    ui::prelude::*,
};

/// Draws the generate dialog over the page while it's open: the repo, its
/// apps and the shared tooling, each in a box, then a yes or no.
pub(super) fn render_confirm(state: &State, frame: &mut Frame) {
    if !state.confirm {
        return;
    }
    let app = &state.app;
    let apps = app_lines(app);
    let apps_height = u16::try_from(apps.len())
        .unwrap_or(u16::MAX)
        .saturating_add(2)
        .max(6);
    let shared_text = chips(&names(app, View::Repo, Scope::Repo));
    // The width a shared line gets inside the outer and inner boxes.
    let shared_width = (frame.area().width * 7 / 10).saturating_sub(10).max(1);
    let shared_height = u16::try_from(shared_text.width())
        .unwrap_or(u16::MAX)
        .div_ceil(shared_width)
        .max(1)
        + 2;
    // Outer border and padding, the boxes, a blank line, the question, the buttons.
    let height = 3 + apps_height + shared_height + 1 + 1 + 3;
    let area = frame
        .area()
        .centered(Constraint::Percentage(70), Constraint::Length(height));
    let outer = titled("generate", true).padding(Padding::new(2, 2, 1, 0));
    let inner = outer.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(outer.bg(BACKGROUND), area);

    let [top, shared, _, answer] = Layout::vertical([
        Constraint::Length(apps_height),
        Constraint::Length(shared_height),
        Constraint::Length(1),
        Constraint::Length(4),
    ])
    .areas(inner);
    let [repo, apps_area] =
        Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)])
            .spacing(1)
            .areas(top);
    frame.render_widget(
        Paragraph::new(repo_lines(app))
            .block(titled("repo", false))
            .wrap(Wrap { trim: true }),
        repo,
    );
    frame.render_widget(
        Paragraph::new(Text::from(apps)).block(titled(&layout(app), false)),
        apps_area,
    );
    frame.render_widget(
        Paragraph::new(shared_text)
            .block(titled("shared tooling", false))
            .wrap(Wrap { trim: true }),
        shared,
    );
    render_answer(state, frame, answer);
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
        Span::styled(format!("{key:<6}"), SECONDARY),
        Span::styled(value, PRIMARY),
    ])
}

/// Where the repo goes and what it holds.
fn repo_lines(app: &App) -> Text<'static> {
    let files = app
        .staged
        .as_ref()
        .map_or(0, std::collections::BTreeMap::len);
    Text::from(vec![
        row("into", app.dest.join(&app.name).display().to_string()),
        row("files", files.to_string()),
        row("owner", app.owner.clone()),
        row("desc", app.desc.clone()),
    ])
}

/// The apps box title: how the apps are laid out.
fn layout(app: &App) -> String {
    let workspace =
        (0..app.apps.len()).any(|at| app.checked(View::App(at)).contains(&OptId::RustCommon));
    match (app.is_multi(), workspace) {
        (false, _) => "one app, at the root".into(),
        (true, true) => format!("{} apps · Cargo workspace", app.apps.len()),
        (true, false) => format!("{} apps", app.apps.len()),
    }
}

/// One line per app: its folder, then what it is made of.
fn app_lines(app: &App) -> Vec<Line<'static>> {
    app.apps
        .iter()
        .enumerate()
        .map(|(at, unit)| {
            let place = if app.is_multi() {
                format!("{APPS_DIR}/{}/", unit.name)
            } else {
                "./".into()
            };
            let mut line = vec![
                Span::styled("▬ ", ASCENT),
                Span::styled(format!("{place:<16}"), PRIMARY).bold(),
            ];
            line.extend(chips(&leaves(app, View::App(at))).spans);
            Line::from(line)
        })
        .collect()
}

/// The options checked in `view` that `scope` holds, by name.
fn names(app: &App, view: View, scope: Scope) -> Vec<&'static str> {
    app.rows(view)
        .into_iter()
        .filter(|row| row.checked && !row.idle && row.id.def().scope == scope)
        .map(|row| <&'static str>::from(row.id))
        .collect()
}

/// The app options checked in `view` with no checked option under them:
/// `rust.server.ws` says `rust`, `rust.common` and `rust.server` already.
fn leaves(app: &App, view: View) -> Vec<&'static str> {
    let checked: Vec<OptId> = app
        .rows(view)
        .into_iter()
        .filter(|row| row.checked && !row.idle)
        .map(|row| row.id)
        .collect();
    checked
        .iter()
        .filter(|id| !checked.iter().any(|other| other.def().parent == Some(**id)))
        .map(|&id| <&'static str>::from(id))
        .collect()
}

/// `names` as a row of muted chips, or "nothing" when there are none.
fn chips(names: &[&'static str]) -> Line<'static> {
    if names.is_empty() {
        return Line::styled("nothing", SECONDARY).italic();
    }
    let mut spans = Vec::new();
    for (at, name) in names.iter().enumerate() {
        if at > 0 {
            spans.push(Span::styled(" · ", UNASCENT));
        }
        spans.push(Span::styled(*name, PRIMARY));
    }
    Line::from(spans)
}

/// The question and its two buttons, or why the repo can't be generated.
fn render_answer(state: &State, frame: &mut Frame, area: Rect) {
    let app = &state.app;
    let target = app.dest.join(&app.name);
    let blocker = if let Some((a, b)) = app.conflict() {
        let (a, b): (&str, &str) = (a.into(), b.into());
        Some(format!("✕ {a} conflicts with {b}"))
    } else if target.exists() {
        Some(format!("✕ {} already exists", target.display()))
    } else {
        None
    };
    if let Some(blocker) = blocker {
        let line = Line::styled(blocker, ERROR).bold().centered();
        frame.render_widget(Paragraph::new(line), area);
        return;
    }
    let [question, buttons] =
        Layout::vertical([Constraint::Length(1), Constraint::Length(3)]).areas(area);
    let question_line = Line::from(vec![
        Span::styled("Generate ", PRIMARY),
        Span::styled(app.name.clone(), ASCENT).bold(),
        Span::styled("?", PRIMARY),
    ]);
    frame.render_widget(Paragraph::new(question_line.centered()), question);
    let [yes, no] = Layout::horizontal([Constraint::Length(9); 2])
        .flex(Flex::Center)
        .spacing(4)
        .areas(buttons);
    button(frame, yes, "yes", state.confirm_yes);
    button(frame, no, "no", !state.confirm_yes);
}

/// A boxed button, filled when `selected`.
fn button(frame: &mut Frame, area: Rect, label: &'static str, selected: bool) {
    let (border, text) = if selected {
        (ASCENT, Style::new().fg(BACKGROUND).bg(ASCENT).bold())
    } else {
        (ASCENT_BIS, Style::new().fg(SECONDARY))
    };
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(border);
    let line = Line::styled(format!(" {label} "), text).centered();
    frame.render_widget(Paragraph::new(line).block(block), area);
}
