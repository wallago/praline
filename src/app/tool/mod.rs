use std::path::Path;

use crate::app::{RepoBuilder, tool::category::Category};
use crate::prelude::*;

/// Tool emitting no files; gates `{if:audit}` blocks elsewhere.
pub(super) mod audit;
/// Categories tools are grouped and colored by.
pub(crate) mod category;
/// Tool emitting `templates/claude/` — `CLAUDE.md`, agents, commands.
pub(super) mod claude;
/// Tool emitting `cliff.toml`.
pub(super) mod cliff;
/// Tool emitting `clippy.toml`.
pub(super) mod clippy;
/// Tool emitting `codecov.yml`.
pub(super) mod codecov;
/// Tool emitting `committed.toml`.
pub(super) mod committed;
/// Tool emitting `deny.toml`.
pub(super) mod deny;
/// Tool emitting `.editorconfig`.
pub(super) mod editorconfig;
/// Tool emitting `.envrc`.
pub(super) mod envrc;
/// Tool emitting `templates/git/` — gitignore, workflows, issue and PR templates.
pub(super) mod git;
/// Tool emitting `justfile`.
pub(super) mod just;
/// Tool emitting no files; gates `{if:lychee}` blocks elsewhere.
pub(super) mod lychee;
/// Tool emitting no files; gates `{if:machete}` blocks elsewhere.
pub(super) mod machete;
/// Tool emitting `templates/nix/` — `flake.nix` and lockfile.
pub(super) mod nix;
/// Tool emitting `templates/rust/` — `Cargo.toml` and starter crate layout.
pub(super) mod rust;
/// Tool emitting `rustfmt.toml`.
pub(super) mod rustfmt;
/// Tool emitting `taplo.toml`.
pub(super) mod taplo;
/// Tool emitting `typos.toml`.
pub(super) mod typos;

/// A config-file template praline can write into a generated repo.
pub(crate) trait Tool: std::fmt::Debug {
    /// Identifier shown in the TUI and matched by `{if:<tool>}` markers.
    fn name(&self) -> String;
    /// One-line blurb shown beside the name in the tool list.
    fn desc(&self) -> String;
    /// Group the tool is listed and colored under.
    fn category(&self) -> Category;
    /// Whether the tool starts checked.
    fn default_setup(&self) -> bool;
    /// Writes this tool's files into `root`.
    ///
    /// A tool whose only effect is gating `{if:}` blocks elsewhere writes
    /// nothing and returns `Ok(())`.
    fn gen_template(&self, root: &Path, repo: &RepoBuilder) -> Result<()>;
}
