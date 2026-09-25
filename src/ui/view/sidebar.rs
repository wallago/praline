use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType},
};

use crate::ui::theme::ASCENT_BIS;

pub(super) fn render(frame: &mut Frame, chunk: Rect) {
    let chunks = Layout::new(Direction::Vertical, [Constraint::Ratio(1, 3); 3]).split(chunk);
    {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(ASCENT_BIS);
        frame.render_widget(block, chunks[0]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(ASCENT_BIS);
        frame.render_widget(block, chunks[1]);
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(ASCENT_BIS);
        frame.render_widget(block, chunks[2]);
    }
}
