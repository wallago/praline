use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyEventKind},
};

use crate::{config::Config, error::Result};

// /// Terminal backend.
// mod backend;
// /// Possible commands.
// mod command;
/// Terminal events handler.
// mod event;
/// Application state handler.
mod state;
mod theme;
/// Widget renderer.
mod view;

mod prelude {
    pub(super) use super::state::*;
    pub(super) use super::theme::*;
}

/// Runs the UI until the user quits, restoring the terminal either way.
pub(crate) fn run(config: Config) -> Result<()> {
    let mut terminal = ratatui::try_init()?;
    let result = event_loop(&mut terminal, state::State::new(config)?);
    ratatui::restore();
    result
}

/// Draws, waits for a key press, applies it, until the state says stop.
fn event_loop(terminal: &mut DefaultTerminal, mut state: state::State) -> Result<()> {
    while state.running {
        terminal.draw(|frame| view::render(&mut state, frame))?;
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            state.on_key(key);
        }
    }
    Ok(())
}
