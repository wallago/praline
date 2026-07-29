use std::fs;

use strum::VariantArray;
use tempfile::{TempDir, tempdir};

use crate::{app::tool::Tool, error::Result};

/// Tool
pub(crate) mod tool;

/// Repo builder.
#[derive(Debug)]
pub struct RepoBuilder {
    /// Repo name.
    pub name: String,
    /// Repo description.
    pub desc: String,
    /// Options available.
    pub(crate) options: Vec<Opt>,

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

        for opt in self.options.iter().filter(|opt| opt.checked) {
            let (Some(name), Some(content)) = (opt.tool.filename(), opt.tool.template()) else {
                continue;
            };
            let path = dir.path().join(name);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, content)?;
        }

        self.dir = Some(dir);

        Ok(())
    }

    /// Whether all conditions are met to generate the repo
    pub(crate) fn check(&self) -> bool {
        !self.name.is_empty() && !self.desc.is_empty() && self.options.iter().any(|opt| opt.checked)
    }
}
