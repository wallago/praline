use thiserror::Error as ThisError;

/// Custom error type.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),
    /// Error that may occur while receiving messages from the channel.
    #[error("Channel receive error: `{0}`")]
    ChannelReceive(#[from] std::sync::mpsc::RecvError),
    /// Error that may occur while loading the application config file.
    #[error("Config file error: `{0}`")]
    Config(String),
    /// Error that may occur while parsing serde.
    #[error("Failed to parse serde: `{0}`")]
    SerdeParse(#[from] serde_json::Error),
    /// Error that may occur while parsing key from config file.
    #[error("Failed to parse Keybindings from config file: `{0}`")]
    KeyParse(String),
    /// Error that may occur while handling event.
    #[error("Failed to handle event: `{0}`")]
    Event(String),
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
