use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Text,
    text::{Line, Span},
    widgets::{Block, BorderType, HighlightSpacing, List, ListItem, Padding},
};

use crate::{
    app::{APPS_DIR, View, opt::Scope},
    ui::{
        prelude::*,
        state::{
            bind::app_name_error,
            mode::{AppEdit, DashboardPane},
        },
        view::render_input,
    },
};

/// Draws the repo, its apps and "add app"; with a name being typed, the
/// field for it at the bottom.
pub(super) fn render_apps(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let editing = state
        .dashboard
        .app_edit
        .zip(state.dashboard.editing.as_ref());
    let (list_area, input_area) = if editing.is_some() {
        let [list, input] =
            Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).areas(chunk);
        (list, Some(input))
    } else {
        (chunk, None)
    };
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("apps", SECONDARY).bold(),
        Span::raw(" "),
    ]);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .padding(Padding::new(1, 1, 1, 0))
        .border_style(super::border(state, DashboardPane::Apps));
    let app = &state.app;
    let mut items = vec![ListItem::new(Text::from(vec![
        Line::from("repo"),
        Line::styled("shared tooling", SECONDARY),
    ]))];
    for (at, unit) in app.apps.iter().enumerate() {
        let place = if app.is_multi() {
            format!("{APPS_DIR}/{}/", unit.name)
        } else {
            "root".into()
        };
        // What the app is: its checked top-level stacks, `rust`, `web`.
        let kind: Vec<&str> = app
            .checked(View::App(at))
            .into_iter()
            .filter(|id| id.def().parent.is_none() && id.def().scope == Scope::App)
            .map(<&'static str>::from)
            .collect();
        let kind = if kind.is_empty() {
            "empty".to_string()
        } else {
            kind.join("+")
        };
        items.push(ListItem::new(Text::from(vec![
            Line::from(unit.name.clone()),
            Line::styled(format!("{kind} · {place}"), SECONDARY),
        ])));
    }
    items.push(ListItem::new(Line::styled("+ add app", ASCENT)));
    let list = List::new(items)
        .block(block)
        .style(PRIMARY)
        .highlight_style(Style::new().bg(ASCENT_BIS).bold())
        .highlight_symbol(Span::styled("▌ ", ASCENT))
        .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(list, list_area, &mut state.dashboard.apps);
    if let (Some((edit, input)), Some(area)) = (editing, input_area) {
        let error = app_name_error(&state.app, input.value(), edit);
        let title = match edit {
            AppEdit::New => "new app",
            AppEdit::Rename(_) => "rename",
        };
        render_input(frame, area, title, "", true, Some(input), error);
    }
}
