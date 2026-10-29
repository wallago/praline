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

    tracing::info!("welcome into {}", version!());
    match {ident}::run(&args).await {
        Ok(()) => exit(0),
        Err(e) => {
            eprintln!("{e}");
            exit(1)
        }
    }
}
