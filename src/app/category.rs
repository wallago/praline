use ratatui::style::Color;

/// Tool categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::Display)]
pub(crate) enum Category {
    /// Format code.
    Format,
    /// Lint code.
    Lint,
    /// Run tests.
    Test,
    /// Scan for security issues.
    Security,
    /// Build the project.
    Build,
    /// Publish or release.
    Release,
    /// Version control.
    Git,
    /// Environment and tooling setup.
    Env,
}
