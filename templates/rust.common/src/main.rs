//! Command line entry point for `{ident}`.

use std::process::exit;

use clap::Parser;
use {ident}::{prelude::*, version};

/// Runs the crate and maps the outcome to a process exit code.
#[tokio::main]
async fn main() {
    let args = Args::parse();
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
    // The TUI owns the terminal, so logs go to a file instead of stderr.
    match log_file() {
        Ok(file) => tracing_subscriber::fmt()
            .with_max_level(level)
            .with_ansi(false)
            .with_writer(std::sync::Mutex::new(file))
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

/// Opens (appending) `<state dir>/<crate>/<crate>.log`, creating the folder.
///
/// # Errors
///
/// Returns an error if the folder or the file can't be created.
fn log_file() -> std::io::Result<std::fs::File> {
    let dir = dirs::state_dir()
        .or_else(dirs::cache_dir)
        .unwrap_or_else(std::env::temp_dir)
        .join(env!("CARGO_PKG_NAME"));
    std::fs::create_dir_all(&dir)?;
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join(concat!(env!("CARGO_PKG_NAME"), ".log")))
}
// {endif:rust.tui}
