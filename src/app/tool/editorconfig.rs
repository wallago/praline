use std::path::Path;

use crate::app::{RepoBuilder, Tool, tool::category::Category};
use crate::prelude::*;

/// Indent and whitespace rules shared by editors.
#[derive(Debug)]
pub(crate) struct EditorConfig;

impl Tool for EditorConfig {
    fn name(&self) -> String {
        "editorconfig".to_string()
    }

    fn desc(&self) -> String {
        "Indent and whitespace rules shared by editors.".to_string()
    }

    fn category(&self) -> Category {
        Category::Format
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, _: &RepoBuilder) -> Result<()> {
        let content = include_str!("../../../templates/.editorconfig");
        write_entry(root, ".editorconfig", content.as_bytes())?;
        Ok(())
    }
}
