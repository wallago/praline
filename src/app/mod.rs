use std::path::Path;

use strum::IntoEnumIterator;

use crate::app::engine::{Tree, write_tree};
use crate::app::opt::OptId;
use crate::error::EngineError::{NotGenerated, TargetExists};
use crate::prelude::*;

/// Optional tools.
mod opt;

/// Optional category.
mod category;

/// Renders the active options' templates into a file tree.
mod engine;

/// App types the rest of the crate uses often.
pub(crate) mod prelude {
    pub(crate) use super::opt::OptId;
}

/// Repo builder.
#[derive(Debug)]
pub struct App {
    /// Repo name.
    pub name: String,
    /// Repo description.
    pub desc: String,
    /// Repo owner.
    pub owner: String,
    /// Options available.
    pub(crate) options: Vec<Opt>,
    /// Last render, shown in the preview and written on export.
    pub(crate) staged: Option<Tree>,
}

/// Selectable repo option.
#[derive(Debug)]
pub(crate) struct Opt {
    /// Which option: the key into the table.
    pub(crate) id: OptId,
    /// Ticked by the user (it can still be inactive, see `Ctx::active`).
    pub(crate) checked: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: String::from("demo-app"),
            desc: String::from("demo-desc"),
            owner: String::from("demo"),
            options: OptId::iter()
                .map(|id| Opt {
                    id,
                    checked: id.def().default,
                })
                .collect(),
            staged: None,
        }
    }
}

impl App {
    /// Renders the repo into memory.
    ///
    /// # Errors
    ///
    /// Returns an error if a template names an option that doesn't exist, or
    /// if two active options render the same file. `staged` is left untouched.
    pub fn generate(&mut self) -> Result<()> {
        self.staged = Some(engine::generate(self)?);
        Ok(())
    }

    /// Writes the last render to `dest/<name>`.
    ///
    /// # Errors
    ///
    /// Returns an error if nothing has been generated yet, if the target
    /// already exists, or if creating a directory or writing a file fails.
    /// A failure part-way leaves the files already written on disk.
    pub fn create(&self, dest: &Path) -> Result<()> {
        let Some(tree) = &self.staged else {
            return Err(NotGenerated.into());
        };
        let target = dest.join(&self.name);
        if target.exists() {
            return Err(TargetExists(target).into());
        }
        write_tree(tree, &target)
    }
}
