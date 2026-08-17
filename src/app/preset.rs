/// A named bundle of tools that can be applied to the option list in one go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Preset {
    /// Version control and editor hygiene, nothing else.
    Minimal,
    /// Language-agnostic repo scaffolding: CI, env, changelog, docs.
    Repo,
    /// An everyday Rust project: crate layout, formatter, linter.
    Rust,
    /// Every tool praline ships, supply-chain and coverage included.
    Full,
}

/// Tools `Minimal` turns on.
const MINIMAL: &[&str] = &["git", "editorconfig", "typos"];

/// Tools `Repo` adds on top of [`MINIMAL`].
const REPO: &[&str] = &[
    "envrc",
    "nix",
    "justfile",
    "claude",
    "cliff",
    "committed",
    "lychee",
];

/// Tools `Rust` adds on top of [`REPO`].
const RUST: &[&str] = &["rust", "clippy", "rustfmt", "taplo"];

impl Preset {
    /// Every preset, in the order they appear in the preset row.
    pub(crate) const ALL: [Self; 4] = [Self::Minimal, Self::Repo, Self::Rust, Self::Full];

    /// Identifier shown in the preset row.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::Repo => "repo",
            Self::Rust => "rust",
            Self::Full => "full",
        }
    }

    /// One-line blurb shown beside the name in the summary.
    pub(crate) const fn desc(self) -> &'static str {
        match self {
            Self::Minimal => "git and editor hygiene only",
            Self::Repo => "language-agnostic repo scaffolding",
            Self::Rust => "everyday Rust project setup",
            Self::Full => "every tool praline ships",
        }
    }

    /// Whether this preset selects the tool with the given [`Tool::name`].
    ///
    /// [`Tool::name`]: crate::app::tool::Tool::name
    pub(crate) fn selects(self, tool: &str) -> bool {
        match self {
            Self::Minimal => MINIMAL.contains(&tool),
            Self::Repo => MINIMAL.contains(&tool) || REPO.contains(&tool),
            Self::Rust => MINIMAL.contains(&tool) || REPO.contains(&tool) || RUST.contains(&tool),
            Self::Full => true,
        }
    }
}
