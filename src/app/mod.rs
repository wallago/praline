use std::{
    fs,
    path::{Path, PathBuf},
};

use indexmap::IndexMap;
use tempfile::{TempDir, tempdir};

use crate::{
    app::tool::{Tool, claude::Claude, taplo::Taplo},
    error::Result,
};

/// Optional tools.
pub(crate) mod tool;

// /// Core code.
// pub(crate) mod core;

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
        let tools: Vec<Box<dyn Tool>> = vec![Box::new(Taplo), Box::new(Claude)];
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
            // core: Core::VARIANTS.to_vec(),
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

        // // Core files — always written.
        // for core in &self.core {
        //     let (name, content) = (core.filename(), core.render(self));
        //     write_entry(dir.path(), name, content.as_bytes())?;
        // }

        // Optional tools — only the checked ones.
        for opt in self.options.iter().filter(|opt| opt.checked) {
            opt.tool.gen_template(dir.path(), self)?;
        }

        self.dir = Some(dir);

        Ok(())
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
