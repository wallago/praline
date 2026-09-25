use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Gauge, HighlightSpacing, List, ListState, Padding},
};

use crate::ui::prelude::*;

pub(super) fn render(frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(Direction::Vertical, [Constraint::Ratio(1, 3); 3]).split(chunk);
    {
        render_navigate(frame, chunks[1]);
        let title = Line::from(vec![
            Span::raw(" "),
            Span::styled("cook", ASCENT).bold(),
            Span::raw(" "),
        ]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title)
            .border_style(ASCENT_BIS);
        frame.render_widget(block, chunks[0]);
        render_summary(frame, chunks[2]);
    }
}

fn render_navigate(frame: &mut Frame, chunk: Rect) {
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("navigate", ASCENT).bold(),
        Span::raw(" "),
    ]);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .border_style(ASCENT_BIS)
        .padding(Padding::new(1, 1, 1, 0));
    let list = List::new(["dashboard", "details", "settings"])
        .block(block)
        .style(SECONDARY)
        .highlight_style(Style::new().fg(ASCENT).bg(ASCENT_BIS).bold())
        .highlight_symbol(Span::styled("▌ ", ASCENT))
        .highlight_spacing(HighlightSpacing::Always);
    let mut state = ListState::default().with_selected(Some(0));
    frame.render_stateful_widget(list, chunk, &mut state);
}

fn render_summary(frame: &mut Frame, chunk: Rect) {
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("summary", SECONDARY).bold(),
        Span::raw(" "),
    ]);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .border_style(ASCENT_BIS)
        .padding(Padding::new(1, 1, 1, 0));
    let inner = block.inner(chunk);
    frame.render_widget(block, chunk);
    let [header, content, _, bar] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1), // spacer
        Constraint::Length(1),
    ])
    .areas(inner);
    let [bar, _] = Layout::horizontal([Constraint::Ratio(2, 3), Constraint::Fill(1)]).areas(bar);
    let total_header = Line::from(Span::styled("OPTIONS", SECONDARY));
    let total_content = Line::from(vec![
        Span::styled("1/3 done", PRIMARY).bold(),
        Span::raw("    "),
        Span::styled("33%", PRIMARY).bold(),
    ]);
    let gauge = Gauge::default()
        .gauge_style(Style::new().fg(ASCENT).bg(ASCENT_BIS))
        .ratio(1.0 / 3.0)
        .label("")
        .use_unicode(true);

    frame.render_widget(total_header, header);
    frame.render_widget(total_content, content);
    frame.render_widget(gauge, bar);
}
