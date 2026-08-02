use std::fmt::Write;

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

/// Macro to send a list of path, valid at compile time.
macro_rules! push_just {
    ($out:expr, $($path:literal),+ $(,)?) => {
        $(
            $out.push('\n');
            $out.push_str(include_str!($path));
        )+
    };
}

/// Assemble the justfile from the base header plus each selected tool's recipe.
fn build_justfile(repo: &RepoBuilder) -> String {
    // use std::fmt::Write;

    let tools: Vec<_> = repo
        .options
        .iter()
        .filter(|opt| opt.checked)
        .map(|opt| opt.tool)
        .collect();

    let mut out = Core::Justfile.template().to_string();
    push_just!(
        out,
        "../../templates/just/jj.just",
        "../../templates/just/rust.just",
        "../../templates/just/nix.just",
    );

    for &tool in &tools {
        if let Some(recipe) = tool.recipe() {
            out.push('\n');
            out.push_str(recipe);
        }
    }

    out.push_str("\n# Mirror the full CI pipeline locally. Run before pushing.\n");
    out.push_str("[group('ci')]\nci:\n");
    for &tool in &tools {
        if let Some(step) = tool.ci() {
            let _ = write!(out, "    @echo \"▶ {}\"\n    {step}\n", tool.label());
        }
    }
    out.push_str("    @echo \"▶ test\"\n    cargo nextest run\n");
    out.push_str("    @echo \"▶ nix\"\n    nix flake check --print-build-logs --all-systems\n");
    out.push_str("    @echo \"✅ CI mirror passed\"\n");

    out
}
