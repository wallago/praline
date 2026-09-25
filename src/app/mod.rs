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

mod engine;

pub(crate) mod prelude {
    pub(crate) use super::category::Category;
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
            name: String::new(),
            desc: String::new(),
            owner: String::new(),
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
    pub fn generate(&mut self) -> Result<()> {
        self.staged = Some(engine::generate(self)?);
        Ok(())
    }

    /// Writes the last render to `dest/<name>`.
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

    // /// Generate repo.
    // ///
    // /// # Errors
    // ///
    // /// Returns an error if the temporary directory cannot be created, or if
    // /// writing any selected tool's template file to disk fails.
    // pub fn generate(&mut self) -> Result<()> {
    //     self.dir = Some(engine::generate(self)?);
    //     Ok(())
    // }

    // /// Copies the staged repo out of the temporary directory into `dest`.
    // ///
    // /// # Errors
    // ///
    // /// Returns an error if nothing has been generated yet, if the target already
    // /// exists, or if any file copy fails.
    // pub fn create(&self, dest: &Path) -> Result<()> {
    //     let Some(dir) = self.dir.as_ref() else {
    //         return Err(Error::Config("nothing generated yet".to_string()));
    //     };
    //     let target = dest.join(&self.name);
    //     if target.exists() {
    //         return Err(Error::Config(format!(
    //             "{} already exists",
    //             target.display()
    //         )));
    //     }
    //     copy_dir_all(dir.path(), &target)
    // }

    // /// Whether the tool with the given name is selected.
    // pub(crate) fn is_selected(&self, tool: &str) -> bool {
    //     self.options
    //         .iter()
    //         .any(|opt| opt.checked && opt.tool.name() == tool)
    // }

    // /// Whether all conditions are met to generate the repo.
    // pub(crate) fn check(&self) -> bool {
    //     !self.name.is_empty() && !self.desc.is_empty() && self.options.iter().any(|opt| opt.checked)
    // }
    //
    // /// Get content of stage dir with a `IndexMap` of path and associated content.
    // pub(crate) fn inspect_stage(&mut self) -> Option<IndexMap<String, (String, PathBuf)>> {
    //     let root = self.dir.as_ref().map(|dir| dir.path().to_path_buf())?;
    //     let mut entries = IndexMap::new();
    //     collect_files(&root, &root, &mut entries)?;
    //     Some(entries)
    // }

    // /// Checks exactly the tools `preset` selects, unchecking every other one.
    // pub(crate) fn apply_preset(&mut self, preset: Preset) {
    //     for opt in &mut self.options {
    //         opt.checked = preset.selects(&opt.tool.name());
    //     }
    // }

    // /// The preset whose tool set matches the current selection exactly, if any.
    // ///
    // /// `None` means the user has hand-picked a set no preset describes.
    // pub(crate) fn active_preset(&self) -> Option<Preset> {
    //     Preset::ALL.into_iter().find(|preset| {
    //         self.options
    //             .iter()
    //             .all(|opt| opt.checked == preset.selects(&opt.tool.name()))
    //     })
    // }
}
