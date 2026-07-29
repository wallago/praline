//! Passing CLI info and run **repo-builder**

use std::process;
use std::time::Duration;

use clap::Parser;
use praline::args::Args;
use praline::error::Result;
use ratatui::style::Color;
use termbg::Theme;

/// Main function to run the app
fn main() -> Result<()> {
    let mut args = Args::parse();
    if args.accent_color.is_none() {
        args.accent_color = termbg::theme(Duration::from_millis(10))
            .map(|theme| {
                if theme == Theme::Dark {
                    Color::White
                } else {
                    Color::Black
                }
            })
            .ok();
    }
    match praline::run(&args) {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
