use std::path::Path;

use super::write_entry;
use crate::{
    app::{RepoBuilder, Tool, tool::category::Category},
    error::Result,
};

#[derive(Debug)]
pub(crate) struct Envrc;

impl Tool for Envrc {
    fn name(&self) -> String {
        "envrc".to_string()
    }

    fn desc(&self) -> String {
        "TBD.".to_string()
    }

    fn category(&self) -> Category {
        Category::Env
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, _: &RepoBuilder) -> Result<()> {
        let content = include_str!("../../../templates/.envrc");
        write_entry(root, ".envrc", content.as_bytes())?;
        Ok(())
    }
}
