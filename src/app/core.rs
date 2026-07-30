use strum::VariantArray;

use crate::app::RepoBuilder;

/// Core code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, VariantArray)]
pub(crate) enum Core {
    /// Cargo — the Rust package manager.
    Cargo,
    /// Gitignore — ignore specify files/dirs
    Gitignore,
    ///
    Envrc,
}

impl Core {
    /// Get relative path of the file this tool emits, if any.
    pub(crate) fn filename(self) -> &'static str {
        match self {
            Self::Cargo => "Cargo.toml",
            Self::Gitignore => ".gitignore",
            Self::Envrc => ".envrc",
        }
    }

    /// Raw template body, embedded at compile time.
    pub(crate) fn template(self) -> &'static str {
        match self {
            Self::Cargo => include_str!("../../templates/Cargo.toml"),
            Self::Gitignore => include_str!("../../templates/.gitignore"),
            Self::Envrc => include_str!("../../templates/.envrc"),
        }
    }

    /// Template with the variant's fields substituted in.
    pub(crate) fn render(self, repo: &RepoBuilder) -> String {
        let template = self.template();
        match self {
            Self::Cargo => template
                .replace("{name}", &repo.name)
                .replace("{desc}", &repo.desc),
            Self::Gitignore => template.to_string(),
            Self::Envrc => template.to_string(),
        }
    }
}
