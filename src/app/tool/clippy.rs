use std::path::Path;

use crate::app::{RepoBuilder, Tool, tool::category::Category};
use crate::prelude::*;

/// Rust lint rules and their severity.
#[derive(Debug)]
pub(crate) struct Clippy;

impl Tool for Clippy {
    fn name(&self) -> String {
        "clippy".to_string()
    }

    fn desc(&self) -> String {
        "Rust lint rules and their severity.".to_string()
    }

    fn category(&self) -> Category {
        Category::Lint
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, _: &RepoBuilder) -> Result<()> {
        let content = include_str!("../../../templates/clippy.toml");
        write_entry(root, "clippy.toml", content.as_bytes())?;
        Ok(())
    }
}
