//! Config file setup.

use std::path::Path;

use serde::Deserialize;

use crate::prelude::*;

// {slot:rust.config.mods}

/// Configuration loaded from `config.toml`.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Config {
    // {slot:rust.config.params}
}

impl Config {
    /// Loads the config: explicit `--config` path, else
    /// `$XDG_CONFIG_HOME/env!("CARGO_PKG_NAME")/config.toml` if present, else defaults.
    ///  
    /// # Errors
    ///
    /// Returns an error if a config file is found but cannot be read or parsed.
    pub(crate) fn load(cli_path: Option<&Path>) -> Result<Self> {
        if let Some(path) = cli_path {
            return Self::from_file(path);
        }
        match dirs::config_dir()
            .map(|dir| dir.join(format!("{}/config.toml", env!("CARGO_PKG_NAME"))))
        {
            Some(path) if path.exists() => Self::from_file(&path),
            _ => Ok(Self::default()),
        }
    }

    /// Reads and parses a config file; any failure is a hard error.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read, or if its contents are not
    /// valid TOML for this config.
    pub(crate) fn from_file(path: &Path) -> Result<Self> {
        // Get raw content
        let raw = std::fs::read_to_string(path)
            .map_err(|error| Error::Config(format!("{}: {error}", path.display())))?;
        // Deserialize content in TOML format
        toml::from_str(&raw).map_err(|error| Error::Config(format!("{}: {error}", path.display())))
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    /// Writes `contents` to a file unique to `test`, and returns its path.
    fn write_config(test: &str, contents: &str) -> std::io::Result<PathBuf> {
        let dir = std::env::temp_dir().join(format!(
            "{}-{}-{test}",
            env!("CARGO_PKG_NAME"),
            std::process::id()
        ));
        fs::create_dir_all(&dir)?;
        let path = dir.join("config.toml");
        fs::write(&path, contents)?;
        Ok(path)
    }

    #[test]
    fn empty_file_gives_defaults() -> Result<()> {
        let path = write_config("empty", "")?;
        Config::from_file(&path)?;
        Ok(())
    }

    #[test]
    fn explicit_path_is_used() -> Result<()> {
        let path = write_config("explicit", "")?;
        Config::load(Some(&path))?;
        Ok(())
    }

    #[test]
    fn missing_explicit_path_is_an_error() {
        let path = Path::new("/nonexistent/config.toml");
        let result = Config::load(Some(path));
        assert!(
            matches!(&result, Err(Error::Config(msg)) if msg.contains("/nonexistent/config.toml")),
            "{result:?}"
        );
    }

    #[test]
    fn invalid_toml_is_an_error() -> std::io::Result<()> {
        let path = write_config("invalid", "this is = = not toml")?;
        let result = Config::from_file(&path);
        assert!(matches!(&result, Err(Error::Config(_))), "{result:?}");
        Ok(())
    }

    #[test]
    fn unknown_key_is_rejected() {
        let result = toml::from_str::<Config>("not_a_real_setting = 1");
        assert!(result.is_err(), "{result:?}");
    }
}
