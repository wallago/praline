use strum::VariantArray;

use crate::app::RepoBuilder;

/// Core code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, VariantArray)]
pub(crate) enum Core {
    /// Cargo — the Rust package manager.
    Cargo,
    /// Gitignore — ignore specify files/dirs
    Gitignore,
    /// direnv — per-directory environment.
    Envrc,
    /// just — task runner (assembled from selected tools).
    Justfile,
}

impl Core {
    /// Get relative path of the file this tool emits, if any.
    pub(crate) fn filename(self) -> &'static str {
        match self {
            Self::Cargo => "Cargo.toml",
            Self::Gitignore => ".gitignore",
            Self::Envrc => ".envrc",
            Self::Justfile => "justfile",
        }
    }

    /// Raw template body, embedded at compile time.
    pub(crate) fn template(self) -> &'static str {
        match self {
            Self::Cargo => include_str!("../../templates/Cargo.toml"),
            Self::Gitignore => include_str!("../../templates/.gitignore"),
            Self::Envrc => include_str!("../../templates/.envrc"),
            Self::Justfile => include_str!("../../templates/justfile"),
        }
    }

    /// Template with the variant's fields substituted in.
    pub(crate) fn render(self, repo: &RepoBuilder) -> String {
        match self {
            Self::Cargo => self
                .template()
                .replace("{name}", &repo.name)
                .replace("{desc}", &repo.desc)
                .replace("{owner}", &repo.owner),
            Self::Justfile => build_justfile(repo),
            _ => self.template().to_string(),
        }
    }
}

/// Assemble the justfile from the base header plus each selected tool's recipe.
fn build_justfile(repo: &RepoBuilder) -> String {
    use std::fmt::Write;

    // Selected tools, in a stable (enum) order.
    let selected: Vec<_> = repo
        .options
        .iter()
        .filter(|opt| opt.checked)
        .map(|opt| opt.tool)
        .collect();

    // 1. Static base: `default:`, run/build/check/test, etc.
    let mut out = Core::Justfile.template().to_string();

    // 2. Per-tool recipes (each fragment carries its own [group(...)]).
    for tool in &selected {
        if let Some(recipe) = tool.recipe() {
            out.push('\n');
            out.push_str(recipe);
        }
    }

    // 3. Synthesized `ci:` — tracks the selection automatically.
    out.push_str("\n[group('ci')]\nci:\n");
    for tool in &selected {
        if let Some(step) = tool.ci_step() {
            let _ = writeln!(out, "    @echo '▶ {}'\n    {step}", tool.label());
        }
    }
    let _ = writeln!(out, "    @echo '✅ CI mirror passed'");

    out
}
