use std::path::Path;

use crate::app::{RepoBuilder, Tool, tool::category::Category};
use crate::prelude::*;

/// TOML formatting.
#[derive(Debug)]
pub(crate) struct Taplo;

impl Tool for Taplo {
    fn name(&self) -> String {
        "taplo".to_string()
    }

    fn desc(&self) -> String {
        "TOML formatting.".to_string()
    }

    fn category(&self) -> Category {
        Category::Format
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, _: &RepoBuilder) -> Result<()> {
        let content = include_str!("../../../templates/taplo.toml");
        write_entry(root, "taplo.toml", content.as_bytes())?;
        Ok(())
    }
}
