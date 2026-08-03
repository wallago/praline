//! {desc}

/// Errors that can occur while running [`run`].
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {}
    }
}

impl std::error::Error for Error {}

/// A [`Result`](std::result::Result) alias for this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Runs `{name}`.
///
/// # Errors
///
/// Returns an [`Error`] if the run fa
pub fn run() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::run;

    #[test]
    fn run_succeeds() {
        assert_eq!(run().is_ok(), true);
    }
}
