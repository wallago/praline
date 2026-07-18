//! **repo-builder** - Build your repo like a lazy boss

/// Error handler implementation.
pub mod error;

/// Command-line arguments parser.
pub mod args;

/// Common types that can be glob-imported for convenience.
pub mod prelude;

use args::Args;
use prelude::*;

/// Runs binsider.
pub fn run(mut args: Args) -> Result<()> {
    Ok(())
}
