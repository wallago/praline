use std::env::current_dir;
use std::path::PathBuf;

use crate::app::engine::{Tree, write_tree};
use crate::app::opt::OptId;
use crate::error::EngineError::{NotGenerated, TargetExists};
use crate::prelude::*;

/// Optional tools.
mod opt;

/// Optional category.
mod category;

/// Free-text fields and the rules their values follow.
mod field;

/// Renders the active options' templates into a file tree.
mod engine;

/// App types the rest of the crate uses often.
pub(crate) mod prelude {
    pub(crate) use super::field::Field;
    pub(crate) use super::opt::OptId;
}

/// Repo builder.
#[derive(Debug)]
pub(crate) struct App {
    /// Repo name.
    pub(crate) name: String,
    /// Repo description.
    pub(crate) desc: String,
    /// Repo owner.
    pub(crate) owner: String,
    /// Repo destination.
    pub(crate) dest: PathBuf,
    /// Options available.
    pub(crate) options: Vec<Opt>,
    /// Last render, shown in the preview and written on export.
    pub(crate) staged: Option<Tree>,
}

/// Selectable repo option.
#[derive(Debug, Clone)]
pub(crate) struct Opt {
    /// Which option: the key into the table.
    pub(crate) id: OptId,
    /// Ticked by the user (it can still be inactive, see `Ctx::active`).
    pub(crate) checked: bool,
    /// Conflicted by other options.
    pub(crate) conflicted: bool,
}

impl Default for App {
    fn default() -> Self {
        let mut list = Vec::new();
        OptId::sorted_list(&mut list, None);
        Self {
            name: String::from("demo-app"),
            desc: String::from("demo-desc"),
            owner: String::from("demo"),
            dest: current_dir().unwrap_or_default(),
            options: list
                .into_iter()
                .map(|id| Opt {
                    id,
                    checked: id.def().default,
                    conflicted: false,
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
    pub(crate) fn generate(&mut self) -> Result<()> {
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
    pub(crate) fn create(&self) -> Result<()> {
        let Some(tree) = &self.staged else {
            return Err(NotGenerated.into());
        };
        let target = self.dest.join(&self.name);
        if target.exists() {
            return Err(TargetExists(target).into());
        }
        write_tree(tree, &target)
    }

    /// Flags every unchecked option that clashes with a checked one.
    pub(crate) fn mark_conflicts(&mut self) {
        let checked: Vec<OptId> = self
            .options
            .iter()
            .filter(|o| o.checked)
            .map(|o| o.id)
            .collect();
        for opt in &mut self.options {
            opt.conflicted = !opt.checked && checked.iter().any(|&c| opt.id.conflicts_with(c));
        }
    }

    /// Check current option and parent if not already check.
    pub(crate) fn nested_check(&mut self, id: OptId, check: Option<bool>) {
        let Some(opt) = self.options.iter_mut().find(|o| o.id == id) else {
            return;
        };
        opt.checked = check.unwrap_or(!opt.checked);
        let Some(parent) = id.def().parent else {
            return;
        };

        if opt.checked {
            self.nested_check(parent, Some(true));
        }
    }
}
