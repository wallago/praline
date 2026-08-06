use std::path::Path;

use crate::app::{RepoBuilder, Tool, tool::category::Category};
use crate::prelude::*;

/// Catches misspellings in code and docs.
#[derive(Debug)]
pub(crate) struct Typos;

impl Tool for Typos {
    fn name(&self) -> String {
        "typos".to_string()
    }

    fn desc(&self) -> String {
        "Catches misspellings in code and docs.".to_string()
    }

    fn category(&self) -> Category {
        Category::Lint
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, _: &RepoBuilder) -> Result<()> {
        let content = include_str!("../../../templates/typos.toml");
        write_entry(root, "typos.toml", content.as_bytes())?;
        Ok(())
    }
}
