//! demo-desc

/// Main application.
pub mod app;
/// Command-line arguments parser.
pub mod args;
/// Can layer.
pub mod can;
/// Config file.
pub mod config;
/// Error handler implementation.
pub mod error;
/// Helper functions.
pub mod help;
/// Common types that can be glob-imported for convenience.
pub mod prelude;
/// Web layer.
pub mod server;

use prelude::*;

use crate::config::Config;

/// Runs `demo-app`.
///
/// # Errors
///
/// Returns an [`Error`] if the run fa
pub async fn run(args: &Args) -> Result<()> {
    better_panic::install();
    let _config = Config::load(args.config.as_deref())?;

    let can = can::Can::new(
        "vcan0",
        Some(&std::collections::HashSet::from(["100", "101"])),
    )?;
    let (sink, source) = tokio::sync::broadcast::channel::<can::CanMessage>(100);

    tracing::info!("starting server");
    tokio::select! {
        result = server::start("0.0.0.0", "3000") => {
            tracing::info!("Server stopped: {:?}", result);
        },
        _ = can.listening(sink) => {
          tracing::info!("CAN listening stopped");
        },
        _ = can.writing(source) => {
          tracing::info!("CAN writing stopped");
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn run_succeeds() {
        assert_eq!(run(Args::default()).is_ok(), true);
    }
}
