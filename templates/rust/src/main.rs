//! Command line entry point for `{name}`.

use std::process::ExitCode;

/// Runs the crate and maps the outcome to a process exit code.
fn main() -> ExitCode {
    let mut args = Args::parse();
    let level = match args.verbose {
        0 => Level::WARN, // default: warnings + errors only
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE, // -vvv and beyond
    };
    tracing_subscriber::fmt().with_max_level(level).init();
    match {name}::run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{name}: {err}");
            ExitCode::FAILURE
        }
    }
}
