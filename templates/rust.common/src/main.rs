//! Command line entry point for `{ident}`.

use std::process::exit;

use clap::Parser;
// {if:rust.tui}
use tracing_appender::rolling::{Builder, InitError, RollingFileAppender, Rotation};
// {endif:rust.tui}
use {ident}::{prelude::*, version};

/// Runs the crate and maps the outcome to a process exit code.
#[tokio::main]
async fn main() {
    let args = Args::parse();
    // {ifnot:rust.tui}
    let under_journal = std::env::var_os("JOURNAL_STREAM").is_some();
    let fmt = tracing_subscriber::fmt()
        .with_max_level(args.log_level())
        .with_writer(std::io::stderr);
    if under_journal {
        fmt.without_time().with_ansi(false).init();
    } else {
        fmt.init();
    }
    // {endif:rust.tui}
    // {if:rust.tui}
    // The TUI owns the terminal, so logs go to a daily-rotated file instead of stderr.
    match log_file() {
        Ok(file) => tracing_subscriber::fmt()
            .with_max_level(args.log_level())
            .with_ansi(false)
            .with_writer(file)
            .init(),
        Err(e) => eprintln!("logging disabled: {e}"),
    }
    // {endif:rust.tui}

    tracing::info!("welcome into {}", version!());
    match {ident}::run(&args).await {
        Ok(()) => exit(0),
        Err(e) => {
            eprintln!("{e}");
            exit(1)
        }
    }
}
// {if:rust.tui}

/// Log files under `<state dir>/<crate>/`, one per day, the last 7 kept.
///
/// # Errors
///
/// Returns an error if the folder can't be created.
fn log_file() -> std::result::Result<RollingFileAppender, InitError> {
    let dir = dirs::state_dir()
        .or_else(dirs::cache_dir)
        .unwrap_or_else(std::env::temp_dir)
        .join(env!("CARGO_PKG_NAME"));
    Builder::new()
        .rotation(Rotation::DAILY)
        .filename_prefix(env!("CARGO_PKG_NAME"))
        .filename_suffix("log")
        .max_log_files(7)
        .build(dir)
}
// {endif:rust.tui}
