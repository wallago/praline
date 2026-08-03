//! Command line entry point for `{name}`.

use std::process::ExitCode;

/// Runs the crate and maps the outcome to a process exit code.
fn main() -> ExitCode {
    match {name}::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{name}: {err}");
            ExitCode::FAILURE
        }
    }
}
