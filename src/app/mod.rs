use std::{
    fs,
    path::{Path, PathBuf},
};

use indexmap::IndexMap;
use tempfile::{TempDir, tempdir};

use crate::{
    app::tool::{
        Tool, audit::Audit, claude::Claude, cliff::Cliff, clippy::Clippy, codecov::Codecov,
        committed::Committed, deny::Deny, editorconfig::EditorConfig, envrc::Envrc, flake::Flake,
        git::Git, just::Just, machete::Machete, rust::Rust, rustfmt::RustFmt, taplo::Taplo,
        typos::Typos,
    },
    error::{Error, Result},
};

/// Optional tools.
pub(crate) mod tool;

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
            Box::new(Flake),
            Box::new(Rust),
            Box::new(Audit),
            Box::new(Machete),
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
}

fn collect_files(
    root: &Path,
    dir: &Path,
    entries: &mut IndexMap<String, (String, PathBuf)>,
) -> Option<()> {
    for entry in fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let ty = entry.file_type().ok()?;
        if ty.is_dir() {
            collect_files(root, &entry.path(), entries)?; // recurse
        } else if ty.is_file() {
            let content = fs::read_to_string(entry.path()).ok()?;
            let key = entry
                .path()
                .strip_prefix(root)
                .ok()?
                .to_string_lossy()
                .into_owned();
            entries.insert(key, (content, entry.path()));
        }
    }
    Some(())
}

/// Recursively copies `src` into `dst`, creating `dst` and any missing parents.
///
/// `read_dir` yields dot-entries, so hidden files and directories are included.
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let target = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}
