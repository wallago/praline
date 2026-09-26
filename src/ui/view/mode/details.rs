use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph},
};

use crate::ui::prelude::*;

pub(in crate::ui::view) fn render(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(Direction::Horizontal, [Constraint::Min(0)])
        .spacing(2)
        .split(chunk);
    {
        row_1(state, frame, chunks[0]);
    }
}

fn row_1(state: &mut State, frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(Direction::Vertical, [Constraint::Min(0)]).split(chunk);

    {
        let first = state
            .app
            .staged
            .as_ref()
            .and_then(|tree| tree.iter().next());

        let (name, body) = match first {
            Some((path, bytes)) => (
                path.display().to_string(),
                Text::raw(String::from_utf8_lossy(bytes)),
            ),
            None => (String::from("tdf"), Text::raw("nothing staged")),
        };
        let title = Line::from(vec![
            Span::raw(" "),
            Span::styled(name, SECONDARY).bold(),
            Span::raw(" "),
        ]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);

        frame.render_widget(Paragraph::new(body).block(block), chunks[0]);
    }
}
