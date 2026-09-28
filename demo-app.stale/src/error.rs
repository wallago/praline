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
    /// Error that may occur parsing Int.
    #[error("Parse Int error: `{0}`")]
    ParseInt(#[from] std::num::ParseIntError),
    /// Error that may occur while sending frame CAN to sink.
    #[error("Send frame CAN to sink: `{0}`")]
    SinkCan(#[from] tokio::sync::broadcast::error::SendError<(u32, Vec<u8>)>),
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = std::result::Result<T, Error>;

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
