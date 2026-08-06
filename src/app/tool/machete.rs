use std::path::Path;

use crate::app::{RepoBuilder, Tool, tool::category::Category};
use crate::prelude::*;

/// Finds unused dependencies in Cargo.toml.
#[derive(Debug)]
pub(crate) struct Machete;

impl Tool for Machete {
    fn name(&self) -> String {
        "machete".to_string()
    }

    fn desc(&self) -> String {
        "Finds unused dependencies in Cargo.toml.".to_string()
    }

    fn category(&self) -> Category {
        Category::Lint
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, _: &Path, _: &RepoBuilder) -> Result<()> {
        Ok(())
    }
}
