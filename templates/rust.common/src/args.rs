//! Command-line arguments parser.

use clap::Parser;
use tracing::Level;

/// Argument parser powered by [`clap`].
#[derive(Clone, Debug, Default, Parser)]
#[clap(
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
    pub config: Option<std::path::PathBuf>,
}

impl Args {
    /// Maps `-v` repetitions to the most verbose [`Level`] to log.
    #[must_use]
    pub fn log_level(&self) -> Level {
        match self.verbose {
            0 => Level::WARN, // default: warnings + errors only
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE, // -vvv and beyond
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use clap::CommandFactory;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_args() {
        Args::command().debug_assert();
    }

    #[test]
    fn defaults_without_flags() -> Result<(), clap::Error> {
        let args = Args::try_parse_from(["{name}"])?;
        assert_eq!(args.verbose, 0);
        assert_eq!(args.config, None);
        Ok(())
    }

    #[test]
    fn parses_config_path() -> Result<(), clap::Error> {
        let args = Args::try_parse_from(["{name}", "--config", "app.toml"])?;
        assert_eq!(args.config.as_deref(), Some(Path::new("app.toml")));
        Ok(())
    }

    #[test]
    fn rejects_unknown_flag() {
        assert!(Args::try_parse_from(["{name}", "--nope"]).is_err());
    }

    #[rstest]
    #[case::default(&[], Level::WARN)]
    #[case::v(&["-v"], Level::INFO)]
    #[case::vv(&["-vv"], Level::DEBUG)]
    #[case::vvv(&["-vvv"], Level::TRACE)]
    #[case::beyond(&["-v", "-v", "-v", "--verbose"], Level::TRACE)]
    fn verbosity_sets_log_level(
        #[case] flags: &[&str],
        #[case] level: Level,
    ) -> Result<(), clap::Error> {
        let args = Args::try_parse_from(std::iter::once("{name}").chain(flags.iter().copied()))?;
        assert_eq!(args.log_level(), level);
        Ok(())
    }
}
