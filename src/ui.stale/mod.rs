use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyEventKind},
};

use crate::{config::binds::Keybindings, error::Result};

/// Stand-in tools, presets and files until the UI is wired to `app`.
mod placeholder;
/// What the UI shows and how keys change it.
mod state;
/// Sprout's palette and shared widgets.
mod theme;
/// Drawing.
mod view;

/// Runs the UI until the user quits, restoring the terminal either way.
pub(crate) fn run(keys: Keybindings) -> Result<()> {
    let mut terminal = ratatui::try_init()?;
    let result = event_loop(&mut terminal, state::State::new(keys));
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
