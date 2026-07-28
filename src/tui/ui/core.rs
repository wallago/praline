use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{
        Block, Clear, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Wrap,
    },
};

use crate::tui::state::{FormFocus, State};

/// Renders the core.
pub fn render_core(state: &mut State, frame: &mut Frame, rect: Rect) {
    frame.render_widget(Block::bordered(), rect);
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(rect);
    render_form(state, frame, left);
    render_preview(state, frame, right);
}

/// Left column: form.
fn render_form(state: &mut State, frame: &mut Frame, rect: Rect) {
    let inner = rect.inner(Margin {
        horizontal: 2,
        vertical: 1,
    });
    let [name, desc, options] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
    ])
    .areas(inner);

    render_input(
        frame,
        name,
        "Name",
        &state.repo.name,
        state.form_focus == FormFocus::Name,
        state.input_mode,
    );
    render_input(
        frame,
        desc,
        "Desc",
        &state.repo.desc,
        state.form_focus == FormFocus::Desc,
        state.input_mode,
    );
    render_options(
        state,
        frame,
        options,
        state.form_focus == FormFocus::Options,
    );
}

/// Right column: read-only summary derived from the form.
fn render_preview(state: &mut State, frame: &mut Frame, rect: Rect) {
    let checked = state.repo.options.iter().filter(|o| o.checked).count();

    let name = if state.repo.name.is_empty() {
        "No name define yet."
    } else {
        state.repo.name.as_str()
    };
    let desc = if state.repo.desc.is_empty() {
        "No desc define yet."
    } else {
        state.repo.desc.as_str()
    };
    let lines = vec![
        Line::from(name).bold(),
        Line::from(""),
        Line::from(desc),
        Line::from(""),
        Line::from(format!("Selected options : {checked}")),
    ];

    frame.render_widget(
        Paragraph::new(lines),
        rect.inner(Margin {
            vertical: 1,
            horizontal: 2,
        }),
    );
}

/// A single bordered text field.
fn render_input(
    frame: &mut Frame,
    rect: Rect,
    title: &str,
    value: &str,
    focused: bool,
    is_editable: bool,
) {
    let title = format!(" {title} ").fg(Color::Gray).bold();
    let mut block = Block::bordered().title(title);
    if focused && !is_editable {
        block = block.border_style(Color::Yellow);
    } else if focused && is_editable {
        block = block.border_style(Color::Green);
    }
    frame.render_widget(Paragraph::new(value).block(block), rect);
}

/// A list of options.
fn render_options(state: &mut State, frame: &mut Frame, rect: Rect, focused: bool) {
    let title = format!(" Options ").fg(Color::Gray).bold();
    let mut block = Block::bordered().title(title);
    if focused {
        block = block.border_style(Color::Yellow);
    }
    let items: Vec<ListItem> = state
        .repo
        .options
        .iter()
        .map(|opt| {
            let glyph = if opt.checked { "󰄲 " } else { "󰄱 " };
            let line = Line::from(format!("{glyph}{}", opt.tool.label()));
            if opt.checked {
                ListItem::new(line.fg(Color::Green))
            } else {
                ListItem::new(line)
            }
        })
        .collect();

    let list = List::new(items).block(block).highlight_symbol("> ");
    frame.render_stateful_widget(list, rect, &mut state.options_list);

    let mut sb = ScrollbarState::new(state.repo.options.len())
        .position(state.options_list.selected().unwrap_or(0));
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
        .options_list
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

    let block = Block::bordered()
        .title(opt.tool.label())
        .border_style(Color::Yellow);
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
