use std::path::Path;

use serde::Deserialize;

use crate::{
    config::binds::Keybindings,
    error::{Error, Result},
};

pub(crate) mod binds;

/// Configuration loaded from `config.toml`.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// Key bindings.
    pub keybindings: Keybindings,
}

impl Config {
    /// Loads the config: explicit `--config` path, else
    /// `$XDG_CONFIG_HOME/flamingo/config.toml` if present, else defaults.
    pub fn load(cli_path: Option<&Path>) -> Result<Self> {
        if let Some(path) = cli_path {
            return Self::from_file(path);
        }
        match dirs::config_dir().map(|dir| dir.join("flamingo/config.toml")) {
            Some(path) if path.exists() => Self::from_file(&path),
            _ => Ok(Self::default()),
        }
    }

    /// Reads and parses a config file; any failure is a hard error.
    pub fn from_file(path: &Path) -> Result<Self> {
        // Get raw content
        let raw = std::fs::read_to_string(path)
            .map_err(|error| Error::ConfigError(format!("{}: {error}", path.display())))?;
        // Deserialize content in TOML format
        toml::from_str(&raw)
            .map_err(|error| Error::ConfigError(format!("{}: {error}", path.display())))
    }
}
