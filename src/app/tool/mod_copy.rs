use std::{fs, path::Path};

use include_dir::{Dir, include_dir};
use strum::VariantArray;

use crate::{
    app::{RepoBuilder, tool::category::Category},
    error::Result,
};

mod category;

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

    /// Define default setup `Tool`
    pub(crate) fn default_setup(self) -> bool {
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

    /// Get `Dir`.
    pub fn dir(&self) -> Option<Dir<'static>> {
        Some(match self {
            Self::RustFmt => include_dir!("$CARGO_MANIFEST_DIR/"),
            // Self::EditorConfig => "editor config",
            // Self::Taplo => "taplo",
            // Self::Clippy => "clippy",
            // Self::Typos => "typos",
            // Self::Codecov => "codecov",
            // Self::Deny => "deny",
            // Self::Committed => "committed",
            // Self::Cliff => "cliff",
            // Self::Machete => "machete",
            // Self::Audit => "audit",
            // Self::Nextest => "nextest",
            _ => return None,
        })
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
    pub(crate) fn template(self, repo: &RepoBuilder) -> Option<&'static str> {
        let Some(dir) = repo.dir else {
            return None;
        };
        Some(match self {
            Self::RustFmt => write_dir("../../../templates/rustfmt.toml", dir, repo),
            Self::EditorConfig => include_str!("../../../templates/.editorconfig"),
            Self::Taplo => include_str!("../../../templates/taplo.toml"),
            Self::Clippy => include_str!("../../../templates/clippy.toml"),
            Self::Typos => include_str!("../../../templates/typos.toml"),
            Self::Deny => include_str!("../../../templates/deny.toml"),
            Self::Codecov => include_str!("../../../templates/codecov.yml"),
            Self::Committed => include_str!("../../../templates/committed.toml"),
            Self::Cliff => include_str!("../../../templates/cliff.toml"),
        })
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
            Self::Typos => include_str!("../../../templates/just/typos.just"),
            Self::Deny => include_str!("../../../templates/just/deny.just"),
            Self::Committed => include_str!("../../../templates/just/committed.just"),
            Self::Cliff => include_str!("../../../templates/just/cliff.just"),
            Self::Codecov => include_str!("../../../templates/just/codecov.just"),
            Self::Clippy => include_str!("../../../templates/just/clippy.just"),
            Self::Taplo => include_str!("../../../templates/just/taplo.just"),
            Self::EditorConfig => include_str!("../../../templates/just/editorconfig.just"),
            // Self::Machete => include_str!("../../../templates/just/machete.just"),
            // Self::Audit => include_str!("../../../templates/just/audit.just"),
            Self::RustFmt => return None,
        })
    }

    /// The CI check this tool contributes to `just ci`, if any.
    pub(crate) fn ci(self) -> Option<&'static str> {
        Some(match self {
            Self::RustFmt => "cargo fmt --check",
            Self::Taplo => "taplo fmt --check",
            Self::Clippy => "cargo clippy --all-targets -- -D warnings",
            Self::Typos => "typos",
            Self::Deny => "cargo deny check",
            Self::Committed => "committed -vv HEAD",
            Self::EditorConfig => "editorconfig-checker",
            Self::Codecov | Self::Cliff => return None,
        })
    }
}

fn write_dir(dir: &Dir<'_>, root: &Path, repo: &RepoBuilder) -> Result<()> {
    for file in dir.files() {
        let name = file.path().to_string_lossy();
        let bytes = match file.contents_utf8() {
            Some(text) => text
                .replace("{name}", &repo.name)
                .replace("{desc}", &repo.desc)
                .replace("{owner}", &repo.owner)
                .into_bytes(),
            None => file.contents().to_vec(),
        };
        super::write_entry(root, &name, &bytes)?;
    }
    for sub in dir.dirs() {
        write_dir(sub, root, repo)?;
    }
    Ok(())
}

fn write_entry(root: &Path, name: &str, content: &[u8]) -> Result<()> {
    let path = root.join(name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, content)?;
    Ok(())
}

trait ToolTrait {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn category(&self);
    fn default_setup(&self) -> bool;
    fn gen_template(&self, root: &Path);
}
