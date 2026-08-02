use ratatui::style::Color;
use strum::VariantArray;

use crate::app::RepoBuilder;

/// Tool categories.
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
    /// Get color for a Category.
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

/// Available tools.
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
    /// Codecov — code coverage reporting.
    Codecov,
    /// cargo-deny — dependency lint / license checks.
    Deny,
    /// committed — check commit message format.
    Committed,
    /// git-cliff — changelog generator.
    Cliff, // /// cargo-machete — find unused dependencies.
           // Machete,
           // /// cargo-nextest — next-generation test runner.
           // Nextest,
           // /// cargo-audit — vulnerability scanning of dependencies.
           // Audit,
}

impl Tool {
    /// Get label for a Tool.
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::RustFmt => "rustfmt",
            Self::EditorConfig => "editor config",
            Self::Taplo => "taplo",
            Self::Clippy => "clippy",
            Self::Typos => "typos",
            Self::Codecov => "codecov",
            Self::Deny => "deny",
            Self::Committed => "committed",
            Self::Cliff => "cliff",
            // Self::Machete => "machete",
            // Self::Audit => "audit",
            // Self::Nextest => "nextest",
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
            Self::Codecov => "Coverage reporting.",
            Self::Deny => "Dependency / License / Advisory bans.",
            Self::Committed => "Conventional commit message linting.",
            Self::Cliff => "Changelog generation from git history.",
            // Self::Machete => "Unused-dependency detection.",
            // Self::Audit => "RUSTSEC advisory scanning.",
            // Self::Nextest => "A next-generation test runner for Rust.",
        }
    }

    /// Get category for a Tool
    pub(crate) fn category(self) -> Category {
        match self {
            Self::RustFmt | Self::EditorConfig | Self::Taplo => Category::Format,
            Self::Clippy | Self::Typos => Category::Lint,
            Self::Codecov => Category::Test,
            Self::Deny => Category::Security,
            Self::Committed | Self::Cliff => Category::Release,
        }
    }

    /// Get check status for a Tool
    pub(crate) fn default_checked(self) -> bool {
        matches!(
            self,
            Self::Taplo
                | Self::Clippy
                | Self::Codecov
                | Self::Deny
                | Self::Committed
                | Self::RustFmt
                | Self::Cliff
                | Self::EditorConfig
                | Self::Typos
        )
    }

    /// Get relative path of the file this tool emits, if any.
    pub(crate) fn filename(self) -> &'static str {
        match self {
            Self::RustFmt => "rustfmt.toml",
            Self::EditorConfig => ".editorconfig",
            Self::Taplo => "taplo.toml",
            Self::Clippy => "clippy.toml",
            Self::Typos => "typos.toml",
            Self::Deny => "deny.toml",
            Self::Codecov => "codecov.yml",
            Self::Committed => "committed.toml",
            Self::Cliff => "cliff.toml",
        }
    }

    /// Raw template body, embedded at compile time.
    pub(crate) fn template(self) -> &'static str {
        match self {
            Self::RustFmt => include_str!("../../../templates/rustfmt.toml"),
            Self::EditorConfig => include_str!("../../../templates/.editorconfig"),
            Self::Taplo => include_str!("../../../templates/taplo.toml"),
            Self::Clippy => include_str!("../../../templates/clippy.toml"),
            Self::Typos => include_str!("../../../templates/typos.toml"),
            Self::Deny => include_str!("../../../templates/deny.toml"),
            Self::Codecov => include_str!("../../../templates/codecov.yml"),
            Self::Committed => include_str!("../../../templates/committed.toml"),
            Self::Cliff => include_str!("../../../templates/cliff.toml"),
        }
    }

    /// Template with the variant's fields substituted in.
    pub(crate) fn render(self, repo: &RepoBuilder) -> String {
        match self {
            Self::Cliff => self
                .template()
                .replace("{name}", &repo.name)
                .replace("{owner}", &repo.owner),
            _ => self.template().to_string(),
        }
    }

    /// The just recipe this tool contributes, if any.
    pub(crate) fn recipe(self) -> Option<&'static str> {
        Some(match self {
            Self::Typos => include_str!("../../../templates/just/tool/typos.just"),
            Self::Deny => include_str!("../../../templates/just/tool/deny.just"),
            Self::Committed => include_str!("../../../templates/just/tool/committed.just"),
            Self::Cliff => include_str!("../../../templates/just/tool/cliff.just"),
            // config-only tools contribute no recipe:
            _ => return None,
        })
    }
}
