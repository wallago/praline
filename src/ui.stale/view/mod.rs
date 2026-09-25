use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
};

use crate::ui::{
    placeholder::TOOLS,
    state::{Pane, State},
    theme::{self, BLUE, CYAN, GRAY, GREEN, LIME, RED},
};

/// `[2]-Tools` and `[3]-Files`.
mod lists;
/// Dialogs drawn over the panes.
mod modal;
/// `[4]-Preview`.
mod preview;

/// Draws the whole screen.
pub(crate) fn render(state: &mut State, frame: &mut Frame) {
    let [repo, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(42), Constraint::Fill(1)]).areas(body);
    let [tools, files] =
        Layout::vertical([Constraint::Percentage(55), Constraint::Fill(1)]).areas(left);

    render_repo(state, frame, repo);
    lists::render_tools(state, frame, tools);
    lists::render_files(state, frame, files);
    preview::render(state, frame, right);
    render_footer(state, frame, footer);
    modal::render(state, frame);
}

/// Draws `left` and a right-aligned `right` on one row; `left` wins where
/// they meet.
fn spread(frame: &mut Frame, area: Rect, left: Line, right: Line) {
    frame.render_widget(right.right_aligned(), area);
    frame.render_widget(left, area);
}

/// `[1]-Repo`: identity on the left, what's selected on the right.
fn render_repo(state: &State, frame: &mut Frame, area: Rect) {
    let focused = state.focus == Pane::Repo;
    let block = theme::pane("[1]-Repo", focused);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let name = if state.name.is_empty() {
        Span::styled("<name>", GRAY)
    } else {
        Span::raw(state.name.clone()).bold()
    };
    let mut left = match state.problem() {
        Some(problem) => vec![
            Span::styled("✗ ", RED),
            Span::raw(format!("{}/", state.owner)),
            name,
            Span::styled(format!("  {problem}"), RED),
        ],
        None => vec![
            Span::styled("✓ ", GREEN),
            Span::raw(format!("{}/", state.owner)),
            name,
            Span::styled(format!("  {}", state.desc), GRAY),
        ],
    };
    if focused {
        left.push(Span::raw("  (enter to edit)"));
    }
    let left = if focused {
        Line::from(left).reversed()
    } else {
        Line::from(left)
    };

    let preset = state
        .preset()
        .map_or(Span::styled("custom", LIME), |preset| {
            Span::styled(preset.name, GREEN)
        });
    let right = Line::from(vec![
        Span::styled("preset ", BLUE),
        preset,
        Span::styled(" · ", GRAY),
        Span::styled(format!("{}/{}", state.active(), TOOLS.len()), GREEN),
        Span::styled(" tools", BLUE),
        Span::styled(" · ", GRAY),
        Span::styled(state.files().len().to_string(), GREEN),
        Span::styled(" files", BLUE),
    ]);
    spread(frame, inner, left, right);
}

/// Key hints for the focused pane, as `(keys, what)` pairs.
fn hints(state: &State) -> Vec<(String, &'static str)> {
    let keys = &state.keys;
    let moves = format!("{}/{}", keys.scroll_down, keys.scroll_up);
    match state.focus {
        Pane::Repo => vec![
            (keys.enter.to_string(), "edit"),
            (String::from("tab"), "pane"),
            (String::from("?"), "help"),
        ],
        Pane::Tools => vec![
            (moves, "move"),
            (String::from("space"), "toggle"),
            (keys.generate.to_string(), "generate"),
            (String::from("p"), "preset"),
            (String::from("?"), "help"),
        ],
        Pane::Files => vec![
            (moves, "move"),
            (keys.scroll_right.to_string(), "preview"),
            (keys.generate.to_string(), "generate"),
            (String::from("?"), "help"),
        ],
        Pane::Preview => vec![
            (moves, "scroll"),
            (keys.scroll_left.to_string(), "back"),
            (String::from("tab"), "pane"),
            (String::from("?"), "help"),
        ],
    }
}

/// Bottom line: hints, then the last message and the version, closing the
/// frame like sprout's. Hints that don't fit whole are dropped from the end.
fn render_footer(state: &State, frame: &mut Frame, area: Rect) {
    let right = Line::from(vec![
        Span::styled(
            format!("{}:", state.level.label()),
            theme::level(state.level),
        )
        .bold(),
        Span::raw(format!(" {} ", state.message)),
        Span::styled("─ ", CYAN),
        Span::styled(env!("CARGO_PKG_NAME"), GREEN).bold(),
        Span::styled(concat!(" v", env!("CARGO_PKG_VERSION"), " ╯"), CYAN),
    ]);
    let width = u16::try_from(right.width()).unwrap_or(u16::MAX);
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Length(width)])
            .spacing(2)
            .areas(area);

    let lead = Span::styled("╰─ ", CYAN);
    let room = usize::from(left_area.width).saturating_sub(lead.width());
    let mut pairs = if state.modal.is_none() {
        hints(state)
    } else {
        Vec::new()
    };
    while Line::from(theme::hints(&pairs)).width() > room {
        pairs.pop();
    }
    let mut left = vec![lead];
    left.extend(theme::hints(&pairs));
    frame.render_widget(Line::from(left), left_area);
    frame.render_widget(right, right_area);
}
