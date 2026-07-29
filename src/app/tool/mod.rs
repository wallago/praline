use ratatui::style::Color;
use strum::VariantArray;

/// Tool Categories
#[derive(Debug, strum::Display)]
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
    /// CI/CD and automation.
    DevOps,
    /// Environment and tooling setup.
    Env,
}

impl Category {
    /// Get color for a Category
    pub(crate) fn color(&self) -> Color {
        match self {
            Self::Format => Color::Blue,
            Self::Lint => Color::Red,
            Self::Test => Color::Magenta,
            Self::Security => Color::Green,
            Self::Build => Color::LightBlue,
            Self::Release => Color::Cyan,
            Self::Git => Color::Yellow,
            Self::DevOps => Color::LightGreen,
            Self::Env => Color::DarkGray,
        }
    }
}

/// Available Tools
#[derive(Debug, Clone, Copy, PartialEq, Eq, VariantArray)]
pub(crate) enum Tool {
    /// rustfmt — Rust code formatter.
    RustFmt,
    /// Editor Config — editor style configuration.
    EditorConfig,
    /// Taplo — TOML formatter and linter.
    Taplo,
    /// Clippy — Rust linter.
    Clippy,
    /// typos — source code spell checker.
    Typos,
    /// cargo-machete — find unused dependencies.
    Machete,
    /// Codecov — code coverage reporting.
    Codecov,
    /// cargo-nextest — next-generation test runner.
    Nextest,
    /// cargo-deny — dependency lint / license checks.
    Deny,
    /// cargo-audit — vulnerability scanning of dependencies.
    Audit,
}

impl Tool {
    /// Get label for a Tool
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::RustFmt => "rustfmt",
            Self::EditorConfig => "editor config",
            Self::Taplo => "taplo",
            Self::Clippy => "clippy",
            Self::Typos => "typos",
            Self::Machete => "machete",
            Self::Codecov => "codecov",
            Self::Nextest => "nextest",
            Self::Deny => "deny",
            Self::Audit => "audit",
        }
    }

    /// Get desc for a Tool
    pub(crate) fn desc(self) -> &'static str {
        match self {
            Self::RustFmt => "Format Rust code.",
            Self::EditorConfig => "EditorConfig helps maintain consistent coding styles.",
            Self::Taplo => "TOML formatting.",
            Self::Clippy => "Catches common mistakes and improves code quality via lints.",
            Self::Typos => "Source code spell checker.",
            Self::Machete => "Unused-dependency detection.",
            Self::Codecov => "Coverage reporting.",
            Self::Nextest => "A next-generation test runner for Rust.",
            Self::Deny => "Dependency / License / Advisory bans.",
            Self::Audit => "RUSTSEC advisory scanning.",
        }
    }

    /// Get category for a Tool
    pub(crate) fn category(self) -> Category {
        match self {
            Self::RustFmt | Self::EditorConfig | Self::Taplo => Category::Format,
            Self::Clippy | Self::Machete | Self::Typos => Category::Lint,
            Self::Nextest | Self::Codecov => Category::Test,
            Self::Deny | Self::Audit => Category::Security,
        }
    }

    /// Get check status for a Tool
    pub(crate) fn default_checked(self) -> bool {
        matches!(self, Self::Taplo | Self::Clippy)
    }

    /// Get relative path of the file this tool emits, if any.
    pub(crate) fn filename(self) -> Option<&'static str> {
        Some(match self {
            Self::RustFmt => "rustfmt.toml",
            Self::EditorConfig => ".editorconfig",
            Self::Taplo => "taplo.toml",
            Self::Clippy => "clippy.toml",
            Self::Typos => "typos.toml",
            Self::Deny => "deny.toml",
            Self::Codecov => "codecov.yml",
            Self::Machete | Self::Nextest | Self::Audit => return None,
        })
    }

    /// Get file body, embedded in the binary at compile time.
    pub(crate) fn template(self) -> Option<&'static str> {
        Some(match self {
            Self::RustFmt => include_str!("../../../templates/rustfmt.toml"),
            Self::EditorConfig => include_str!("../../../templates/.editorconfig"),
            Self::Taplo => include_str!("../../../templates/taplo.toml"),
            Self::Clippy => include_str!("../../../templates/clippy.toml"),
            Self::Typos => include_str!("../../../templates/typos.toml"),
            Self::Deny => include_str!("../../../templates/deny.toml"),
            Self::Codecov => include_str!("../../../templates/codecov.yml"),
            _ => return None,
        })
    }
}
