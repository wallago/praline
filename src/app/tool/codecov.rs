use std::path::Path;

use super::write_entry;
use crate::{
    app::{RepoBuilder, Tool, tool::category::Category},
    error::Result,
};

#[derive(Debug)]
pub(crate) struct Codecov;

impl Tool for Codecov {
    fn name(&self) -> String {
        "codecov".to_string()
    }

    fn desc(&self) -> String {
        "TBD.".to_string()
    }

    fn category(&self) -> Category {
        Category::Test
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, _: &RepoBuilder) -> Result<()> {
        let content = include_str!("../../../templates/codecov.yml");
        write_entry(root, "codecov.yml", content.as_bytes())?;
        Ok(())
    }
}
