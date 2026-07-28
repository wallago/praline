use strum::VariantArray;

use crate::app::Tag::Test;

/// Repo builder.
#[derive(Debug)]
pub struct RepoBuilder {
    /// Repo name.
    pub name: String,
    /// Repo description.
    pub desc: String,
    /// Options available.
    pub options: Vec<Opt>,
}

// Selectable repo option.
#[derive(Debug)]
pub struct Opt {
    pub tool: Tool,
    pub checked: bool,
}

#[derive(Debug)]
enum Tag {
    Format,
    Lint,
    Test,
    Security,
    Build,
    Release,
    Git,
    DevOps,
    Env,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, VariantArray)]
pub enum Tool {
    RustFmt,
    EditorConfig,
    Taplo,
    Clippy,
    Typos,
    Machete,
    Codecov,
    Nextest,
    Deny,
    Audit,
}

impl Tool {
    pub fn label(self) -> &'static str {
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

    pub fn desc(self) -> &'static str {
        match self {
            Self::RustFmt => "Format Rust code.",
            Self::EditorConfig => {
                "EditorConfig helps maintain consistent coding styles across editors."
            }
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

    pub fn tag(self) -> Tag {
        match self {
            Self::RustFmt | Self::EditorConfig | Self::Taplo => Tag::Format,
            Self::Clippy | Self::Machete | Self::Typos => Tag::Lint,
            Self::Nextest | Self::Codecov => Tag::Test,
            Self::Deny | Self::Audit => Tag::Security,
        }
    }

    pub fn default_checked(self) -> bool {
        matches!(self, Self::Taplo | Self::Clippy)
    }
}

impl Default for RepoBuilder {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            desc: "".to_string(),
            options: Tool::VARIANTS
                .iter()
                .map(|&tool| Opt {
                    tool,
                    checked: tool.default_checked(),
                })
                .collect(),
        }
    }
}
