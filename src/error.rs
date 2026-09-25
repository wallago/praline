use std::path::PathBuf;

use thiserror::Error as ThisError;

/// Type alias for the standard [`Result`] type.
pub type Result<T> = std::result::Result<T, Error>;

/// Crate-wide error: one variant per module that can fail.
#[derive(Debug, ThisError)]
pub enum Error {
    /// I/O outside any module below.
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    /// Rendering templates failed.
    #[error("template engine: {0}")]
    Engine(#[from] EngineError),
    /// Loading `config.toml` failed.
    #[error("config: {0}")]
    Config(#[from] ConfigError),
}

#[derive(Debug, ThisError)]
pub enum EngineError {
    /// A template marker names an option that doesn't exist.
    #[error("unknown option `{0}` in a template")]
    UnknownOption(String),
    /// Two active options render the same file.
    #[error("`{0}` is written by two options")]
    PathCollision(PathBuf),
    /// Export was asked for before anything was rendered.
    #[error("nothing generated yet")]
    NotGenerated,
    /// Export target is already on disk.
    #[error("`{0}` already exists")]
    TargetExists(PathBuf),
}

#[derive(Debug, ThisError)]
pub enum ConfigError {
    /// Error that may occur while parsing key from config file.
    #[error("Failed to parse Keybindings from config file: `{0}`")]
    KeyParse(String),
    /// Error that may occur while deserialize toml file.
    #[error("Failed to deserialize toml file: `{0}`")]
    TomlDeserialize(String),
}

#[cfg(test)]
mod tests {
    use std::io::Error as IoError;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_error() {
        let message = "your computer is on fire!";
        let error = Error::from(IoError::other(message));
        assert_eq!(format!("IO error: `{message}`"), error.to_string());
        assert_eq!(
            format!("\"IO error: `{message}`\""),
            format!("{:?}", error.to_string())
        );
    }
}
