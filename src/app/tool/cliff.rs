use std::path::Path;

use super::write_entry;
use crate::{
    app::{
        RepoBuilder, Tool,
        tool::{category::Category, substitute},
    },
    error::Result,
};

#[derive(Debug)]
pub(crate) struct Cliff;

impl Tool for Cliff {
    fn name(&self) -> String {
        "cliff".to_string()
    }

    fn desc(&self) -> String {
        "Changelog generated from conventional commits.".to_string()
    }

    fn category(&self) -> Category {
        Category::Release
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, repo: &RepoBuilder) -> Result<()> {
        let content = substitute(include_str!("../../../templates/cliff.toml"), repo);
        write_entry(root, "cliff.toml", &content)?;
        Ok(())
    }
}
