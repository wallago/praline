use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{
        Block, Clear, List, ListItem, Padding, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Wrap,
    },
};

use crate::tui::{
    state::{State, form::FormFocus, screen::Screen},
    ui::utils::{badge, render_input},
};

/// Left column: form.
pub(super) fn render_form(state: &mut State, frame: &mut Frame, rect: Rect) {
    let inner = rect.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [owner, name, desc, options] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
    ])
    .areas(inner);

    render_input(
        frame,
        owner,
        "Owner",
        &state.repo.owner,
        state.form_focus == FormFocus::Owner,
        state.screen_mode == Screen::Editing,
    );
    render_input(
        frame,
        name,
        "Name",
        &state.repo.name,
        state.form_focus == FormFocus::Name,
        state.screen_mode == Screen::Editing,
    );
    render_input(
        frame,
        desc,
        "Desc",
        &state.repo.desc,
        state.form_focus == FormFocus::Desc,
        state.screen_mode == Screen::Editing,
    );
    render_options(
        state,
        frame,
        options,
        state.form_focus == FormFocus::Options,
    );
}

/// A list of options.
fn render_options(state: &mut State, frame: &mut Frame, rect: Rect, focused: bool) {
    let title = String::from(" Options ").fg(Color::Gray).bold();
    let mut block = Block::bordered().title(title).padding(Padding::top(1));
    if focused {
        block = block.border_style(Color::Yellow);
    }
    let items: Vec<ListItem> = state
        .repo
        .options
        .iter()
        .map(|opt| {
            let glyph = if opt.checked { "󰄲 " } else { "󰄱 " };

            let spans = vec![
                Span::raw(format!("{glyph}{}", opt.tool.name())),
                Span::raw(" "),
                badge(&opt.tool.category()),
            ];

            let line = Line::from(spans);
            let content = if opt.checked {
                line.fg(Color::Green)
            } else {
                line
            };

            // two-line item: content + blank spacer row
            ListItem::new(vec![content, Line::from("")])
        })
        .collect();

    let list = List::new(items).block(block).highlight_symbol("> ");
    frame.render_stateful_widget(list, rect, &mut state.option_list);

    let mut sb = ScrollbarState::new(state.repo.options.len())
        .position(state.option_list.selected().unwrap_or(0));
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight),
        rect.inner(Margin {
            horizontal: 0,
            vertical: 1,
        }),
        &mut sb,
    );
    render_option_desc(state, frame, rect);
}

/// Floating popup showing the selected option's description.
fn render_option_desc(state: &State, frame: &mut Frame, area: Rect) {
    if state.form_focus != FormFocus::Options {
        return;
    }
    let Some(opt) = state
        .option_list
        .selected()
        .and_then(|i| state.repo.options.get(i))
    else {
        return;
    };

    let width = 32.min(area.width);
    let height = 6.min(area.height);
    let popup = Rect {
        x: area.x + area.width.saturating_sub(width),
        y: area.y + 1,
        width,
        height,
    };

    let title = format!(" {} ", opt.tool.name()).fg(Color::Gray).bold();
    let block = Block::bordered().title(title).border_style(Color::Yellow);
    let paragraph = Paragraph::new(opt.tool.desc())
        .block(block)
        .wrap(Wrap { trim: true });
    let popup = popup.inner(Margin {
        vertical: 0,
        horizontal: 2,
    });
    frame.render_widget(Clear, popup);
    frame.render_widget(paragraph, popup);
}
