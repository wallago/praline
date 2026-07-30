use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::{Path, PathBuf},
};

use indexmap::IndexMap;
use strum::VariantArray;
use tempfile::{TempDir, tempdir};

use crate::{
    app::{core::Core, tool::Tool},
    error::Result,
};

/// Optional tools.
pub(crate) mod tool;

/// Core code.
pub(crate) mod core;

/// Repo builder.
#[derive(Debug)]
pub struct RepoBuilder {
    /// Repo name.
    pub name: String,
    /// Repo description.
    pub desc: String,
    /// Options available.
    pub(crate) options: Vec<Opt>,
    /// Core code.
    core: Vec<Core>,

    /// Directory to test my stuff
    pub dir: Option<TempDir>,
}

/// Selectable repo option.
#[derive(Debug)]
pub(crate) struct Opt {
    /// Tool.
    pub tool: Tool,
    /// Status to know if it will be added.
    pub checked: bool,
}

impl Default for RepoBuilder {
    fn default() -> Self {
        Self {
            name: String::new(),
            desc: String::new(),
            options: Tool::VARIANTS
                .iter()
                .map(|&tool| Opt {
                    tool,
                    checked: tool.default_checked(),
                })
                .collect(),
            core: Core::VARIANTS.to_vec(),
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

        // Core files — always written.
        for core in &self.core {
            let (name, content) = (core.filename(), core.render(self));
            write_entry(dir.path(), name, content.as_bytes())?;
        }

        // Optional tools — only the checked ones.
        for opt in self.options.iter().filter(|opt| opt.checked) {
            let (Some(name), Some(content)) = (opt.tool.filename(), opt.tool.template()) else {
                continue;
            };
            write_entry(dir.path(), name, content.as_bytes())?;
        }

        self.dir = Some(dir);

        Ok(())
    }

    /// Whether all conditions are met to generate the repo.
    pub(crate) fn check(&self) -> bool {
        !self.name.is_empty() && !self.desc.is_empty() && self.options.iter().any(|opt| opt.checked)
    }

    /// Get content of stage dir with a HashMap of path and associated content.
    pub(crate) fn inspect_stage(&mut self) -> Option<IndexMap<String, (String, PathBuf)>> {
        let Some(path) = self.dir.as_mut().map(|dir| dir.path()) else {
            return None;
        };
        let mut entries = IndexMap::new();
        for entry in fs::read_dir(path).ok()? {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                let content = fs::read_to_string(entry.path()).ok()?;
                entries.insert(
                    entry.file_name().to_str()?.to_string(),
                    (content, entry.path()),
                );
            }
        }
        Some(entries)
    }
}

fn write_entry(root: &Path, name: &str, content: &[u8]) -> Result<()> {
    let path = root.join(name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, content)?;
    Ok(())
}
