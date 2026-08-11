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

impl Category {
    /// Every category, in the order they appear in the summary.
    pub(crate) const ALL: [Self; 8] = [
        Self::Git,
        Self::Env,
        Self::Build,
        Self::Format,
        Self::Lint,
        Self::Test,
        Self::Security,
        Self::Release,
    ];

    /// Get color for a Category.
    pub(crate) fn color(self) -> Color {
        match self {
            Self::Format => Color::Blue,
            Self::Lint => Color::Red,
            Self::Test => Color::Magenta,
            Self::Security => Color::Green,
            Self::Build => Color::LightBlue,
            Self::Release => Color::Cyan,
            Self::Git => Color::Yellow,
            Self::Env => Color::LightRed,
        }
    }
}
