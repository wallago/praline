use std::path::Path;

use crate::app::{RepoBuilder, Tool, tool::category::Category};
use crate::prelude::*;

/// Task runner recipes: check, lint, fmt, ci.
#[derive(Debug)]
pub(crate) struct Just;

impl Tool for Just {
    fn name(&self) -> String {
        "justfile".to_string()
    }

    fn desc(&self) -> String {
        "Task runner recipes: check, lint, fmt, ci.".to_string()
    }

    fn category(&self) -> Category {
        Category::Env
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, repo: &RepoBuilder) -> Result<()> {
        let content = substitute(include_str!("../../../templates/justfile"), repo);
        write_entry(root, "justfile", &content)?;
        Ok(())
    }
}
