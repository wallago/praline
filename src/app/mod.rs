use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use tempfile::{TempDir, tempdir};

use crate::app::preset::Preset;
use crate::app::tool::{
    Tool, audit::Audit, claude::Claude, cliff::Cliff, clippy::Clippy, codecov::Codecov,
    committed::Committed, deny::Deny, editorconfig::EditorConfig, envrc::Envrc, git::Git,
    just::Just, lychee::Lychee, machete::Machete, nix::Nix, rust::Rust, rustfmt::RustFmt,
    taplo::Taplo, typos::Typos,
};
use crate::prelude::*;

/// Optional tools.
pub(crate) mod tool;

/// Named bundles of tools selectable in one keystroke.
pub(crate) mod preset;

/// Repo builder.
#[derive(Debug)]
pub struct RepoBuilder {
    /// Repo name.
    pub name: String,
    /// Repo description.
    pub desc: String,
    /// Repo owner.
    pub owner: String,
    /// Options available.
    pub(crate) options: Vec<Opt>,
    /// Directory to test my stuff
    pub dir: Option<TempDir>,
}

/// Selectable repo option.
#[derive(Debug)]
pub(crate) struct Opt {
    /// Tool.
    pub tool: Box<dyn Tool>,
    /// Status to know if it will be added.
    pub checked: bool,
}

impl Default for RepoBuilder {
    fn default() -> Self {
        let tools: Vec<Box<dyn Tool>> = vec![
            Box::new(Taplo),
            Box::new(Claude),
            Box::new(Typos),
            Box::new(RustFmt),
            Box::new(EditorConfig),
            Box::new(Clippy),
            Box::new(Cliff),
            Box::new(Codecov),
            Box::new(Deny),
            Box::new(Committed),
            Box::new(Just),
            Box::new(Envrc),
            Box::new(Git),
            Box::new(Nix),
            Box::new(Rust),
            Box::new(Audit),
            Box::new(Machete),
            Box::new(Lychee),
        ];
        Self {
            name: String::new(),
            desc: String::new(),
            owner: String::new(),
            options: tools
                .into_iter()
                .map(|tool| Opt {
                    checked: tool.default_setup(),
                    tool,
                })
                .collect(),
            dir: None,
        }
    }
}

impl RepoBuilder {
    /// Generate repo.
    ///
    /// # Errors
    ///
    /// Returns an error if the temporary directory cannot be created, or if
    /// writing any selected tool's template file to disk fails.
    pub fn generate(&mut self) -> Result<()> {
        let dir = tempdir()?;

        // Optional tools — only the checked ones.
        for opt in self.options.iter().filter(|opt| opt.checked) {
            opt.tool.gen_template(dir.path(), self)?;
        }

        self.dir = Some(dir);

        Ok(())
    }

    /// Copies the staged repo out of the temporary directory into `dest`.
    ///
    /// # Errors
    ///
    /// Returns an error if nothing has been generated yet, if the target already
    /// exists, or if any file copy fails.
    pub fn create(&self, dest: &Path) -> Result<()> {
        let Some(dir) = self.dir.as_ref() else {
            return Err(Error::Config("nothing generated yet".to_string()));
        };
        let target = dest.join(&self.name);
        if target.exists() {
            return Err(Error::Config(format!(
                "{} already exists",
                target.display()
            )));
        }
        copy_dir_all(dir.path(), &target)
    }

    /// Whether the tool with the given name is selected.
    pub(crate) fn is_selected(&self, tool: &str) -> bool {
        self.options
            .iter()
            .any(|opt| opt.checked && opt.tool.name() == tool)
    }

    /// Whether all conditions are met to generate the repo.
    pub(crate) fn check(&self) -> bool {
        !self.name.is_empty() && !self.desc.is_empty() && self.options.iter().any(|opt| opt.checked)
    }

    /// Get content of stage dir with a `IndexMap` of path and associated content.
    pub(crate) fn inspect_stage(&mut self) -> Option<IndexMap<String, (String, PathBuf)>> {
        let root = self.dir.as_ref().map(|dir| dir.path().to_path_buf())?;
        let mut entries = IndexMap::new();
        collect_files(&root, &root, &mut entries)?;
        Some(entries)
    }

    /// Checks exactly the tools `preset` selects, unchecking every other one.
    pub(crate) fn apply_preset(&mut self, preset: Preset) {
        for opt in &mut self.options {
            opt.checked = preset.selects(&opt.tool.name());
        }
    }

    /// The preset whose tool set matches the current selection exactly, if any.
    ///
    /// `None` means the user has hand-picked a set no preset describes.
    pub(crate) fn active_preset(&self) -> Option<Preset> {
        Preset::ALL.into_iter().find(|preset| {
            self.options
                .iter()
                .all(|opt| opt.checked == preset.selects(&opt.tool.name()))
        })
    }
}
