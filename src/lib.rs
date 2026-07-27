//! **praline** - Build your repo like a lazy boss

/// Error handler implementation.
pub mod error;

/// Command-line arguments parser.
pub mod args;

/// Terminal user interface.
pub mod tui;

/// Possible commands.
pub mod command;

/// Common types that can be glob-imported for convenience.
pub mod prelude;

use std::io;

use args::Args;
use prelude::*;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    command::Command,
    tui::{
        backend::Tui,
        event::{Event, EventHandler},
        state::State,
    },
};

/// Runs praline.
pub fn run(args: Args) -> Result<()> {
    start_tui(args)
}

/// Starts the terminal user interface.
pub fn start_tui(args: Args) -> Result<()> {
    // Create an application.
    let mut state = State::new(args.accent_color)?;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while state.running {
        // Render the user interface.
        tui.draw(&mut state)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => {
                let command = Command::from(key_event);
                state.run_command(command, tui.events.sender.clone())?;
            }
            Event::Mouse(mouse_event) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
