//! **praline** - Build your repo like a lazy boss

/// Error handler implementation.
pub mod error;

/// Command-line arguments parser.
pub mod args;

/// User interface.
mod ui;

/// Config file.
pub mod config;

/// Main application.
pub mod app;

/// Helper functions.
pub mod help;

/// Common types that can be glob-imported for convenience.
pub mod prelude;

use prelude::*;

use crate::config::Config;

/// Runs praline.
///
/// # Errors
///
/// Returns an error if the config fails to load or if the TUI fails to start.
pub fn run(args: &Args) -> Result<()> {
    let config = Config::load(args.config.as_deref())?;
    ui::run(config)
}
