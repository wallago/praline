use std::path::PathBuf;

use clap::Parser;
use strum::IntoEnumIterator;

use crate::prelude::OptId;

/// Argument parser powered by [`clap`].
#[derive(Clone, Debug, Default, Parser)]
#[clap(
    name = "praline",
    version,
    author = clap::crate_authors!("\n"),
    about,
    rename_all_env = "screaming-snake",
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
)]
pub struct Args {
    /// Increase logging verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Path to the application config file (TOML).
    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,
    /// Project description.
    #[arg(long, default_value_t = false)]
    pub headless: bool,
    /// Project name.
    #[arg(long)]
    pub name: Option<String>,
    /// Project owner.
    #[arg(long)]
    pub owner: Option<String>,
    /// Project description.
    #[arg(long)]
    pub desc: Option<String>,
    /// Build destination.
    #[arg(short = 'D', long)]
    pub dest: Option<PathBuf>,
    /// Build options.
    #[arg(
        short,
        long,
        value_parser = clap::builder::PossibleValuesParser::new(
            OptId::iter().map(<&'static str>::from)
        ),
    )]
    pub opt: Vec<String>,
    /// An app of the repo, as `<name>:<option>,<option>`; repeat it for each
    /// app. With two or more, each lands in `apps/<name>/` and `--opt` only
    /// takes repo-wide options.
    #[arg(long, value_name = "NAME:OPTS")]
    pub app: Vec<String>,
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;
    #[test]
    fn test_args() {
        Args::command().debug_assert();
    }
}
