use std::process;
use std::time::Duration;

use clap::Parser;
use ratatui::style::Color;
use repo_builder::args::Args;
use repo_builder::error::Result;
use termbg::Theme;

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
    match repo_builder::run(args) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
