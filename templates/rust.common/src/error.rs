//! Error handler implementation.

use thiserror::Error as ThisError;

/// Errors that can occur while running [`run`].
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    /// Error that may occur while loading the application config file.
    #[error("Config file error: `{0}`")]
    Config(String),
    // {slot:rust.error.deps}
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use std::io::Error as IoError;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn io_error_converts_and_displays() {
        let message = "your computer is on fire!";
        let error = Error::from(IoError::other(message));
        assert!(matches!(error, Error::Io(_)));
        assert_eq!(error.to_string(), format!("IO error: `{message}`"));
    }

    #[test]
    fn config_error_displays() {
        let error = Error::Config("config.toml: bad".to_string());
        assert_eq!(error.to_string(), "Config file error: `config.toml: bad`");
    }
}
