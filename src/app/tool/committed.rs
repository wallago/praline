use std::path::Path;

use super::write_entry;
use crate::{
    app::{RepoBuilder, Tool, tool::category::Category},
    error::Result,
};

#[derive(Debug)]
pub(crate) struct Committed;

impl Tool for Committed {
    fn name(&self) -> String {
        "committed".to_string()
    }

    fn desc(&self) -> String {
        "TBD.".to_string()
    }

    fn category(&self) -> Category {
        Category::Release
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, _: &RepoBuilder) -> Result<()> {
        let content = include_str!("../../../templates/committed.toml");
        write_entry(root, "committed.toml", content.as_bytes())?;
        Ok(())
    }
}
