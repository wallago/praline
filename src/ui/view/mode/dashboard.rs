use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, HighlightSpacing, List, Padding},
};

use crate::ui::{prelude::*, state::mode::DashboardPane, view::render_input};

/// Draws the dashboard page.
pub(in crate::ui::view) fn render(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(65), Constraint::Percentage(35)],
    )
    .spacing(2)
    .split(chunk);
    {
        row_1(state, frame, chunks[0]);
        row_2(state, frame, chunks[1]);
    }
}

/// Draws row 1.
fn row_1(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(65), Constraint::Percentage(35)],
    )
    .split(chunk);

    {
        render_options(state, frame, chunks[0]);
        let title = Line::from(vec![
            Span::raw(" "),
            Span::styled("tdf", SECONDARY).bold(),
            Span::raw(" "),
        ]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);
        // .border_style(border(state, DashboardPane::Tdf));
        frame.render_widget(block, chunks[1]);
    }
}

/// Draws row 2.
fn row_2(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(15),
            Constraint::Percentage(60),
            Constraint::Percentage(25),
        ],
    )
    .split(chunk);

    {
        let title = Line::from(vec![
            Span::raw(" "),
            Span::styled("tdf", SECONDARY).bold(),
            Span::raw(" "),
        ]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);
        // .border_style(border(state, DashboardPane::Preview));
        frame.render_widget(block, chunks[0]);
        render_form(state, frame, chunks[1]);
        let title = Line::from(vec![
            Span::raw(" "),
            Span::styled("tdf", SECONDARY).bold(),
            Span::raw(" "),
        ]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);
        // .border_style(border(state, DashboardPane::Status));
        frame.render_widget(block, chunks[2]);
    }
}

/// Draws form fields.
fn render_form(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("form", SECONDARY).bold(),
        Span::raw(" "),
    ]);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .padding(Padding::new(1, 1, 1, 0))
        .border_style(border(state, DashboardPane::Form));
    frame.render_widget(block, chunk);
    let chunks = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(9),
        ],
    )
    .margin(2)
    .spacing(1)
    .split(chunk);
    {
        render_input(
            frame,
            chunks[0],
            "owner",
            &state.app.owner,
            true,
            true,
            // state.form_focus == FormFocus::Owner,
            // state.screen_mode == Screen::Editing,
        );
        render_input(
            frame,
            chunks[1],
            "app",
            &state.app.name,
            true,
            false,
            // state.form_focus == FormFocus::Owner,
            // state.screen_mode == Screen::Editing,
        );
        render_input(
            frame,
            chunks[2],
            "desc",
            &state.app.desc,
            false,
            false,
            // state.form_focus == FormFocus::Owner,
            // state.screen_mode == Screen::Editing,
        );
    }
    // render_input(
    //     frame,
    //     name,
    //     "Name",
    //     &state.repo.name,
    //     state.form_focus == FormFocus::Name,
    //     state.screen_mode == Screen::Editing,
    // );
    // render_input(
    //     frame,
    //     desc,
    //     "Desc",
    //     &state.repo.desc,
    //     state.form_focus == FormFocus::Desc,
    //     state.screen_mode == Screen::Editing,
    // );
}

/// Draws selectable option list.
fn render_options(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("options", SECONDARY).bold(),
        Span::raw(" "),
    ]);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .padding(Padding::new(1, 1, 1, 0))
        .border_style(border(state, DashboardPane::Options));
    let options = state
        .app
        .options
        .iter()
        .map(|opt| {
            let mark = if opt.checked {
                Span::styled("▬ ", ASCENT)
            } else {
                Span::raw("  ")
            };
            Line::from(vec![mark, Span::raw(<&'static str>::from(opt.id))])
        })
        .collect::<Vec<_>>();
    let list = List::new(options)
        .block(block)
        .style(SECONDARY)
        .highlight_style(Style::new().fg(ASCENT).bg(ASCENT_BIS).bold())
        .highlight_symbol(Span::styled("▌ ", ASCENT))
        .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(list, chunk, &mut state.dashboard.options);
}

/// Border colour for `pane`: accented when it owns the keyboard.
fn border(state: &State, pane: DashboardPane) -> Color {
    if state.focus == Focus::Page && state.dashboard.focus == pane {
        ASCENT
    } else {
        ASCENT_BIS
    }
}
