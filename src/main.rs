//! Passing CLI info and run **repo-builder**

use std::process;

use clap::Parser;
use praline::args::Args;
use praline::error::Result;

/// Main function to run the app
fn main() -> Result<()> {
    let mut args = Args::parse();
    match praline::run(&args) {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
