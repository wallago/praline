use std::path::Path;

use crate::app::{RepoBuilder, Tool, tool::category::Category};
use crate::prelude::*;

/// License, advisory and crate ban policy.
#[derive(Debug)]
pub(crate) struct Deny;

impl Tool for Deny {
    fn name(&self) -> String {
        "deny".to_string()
    }

    fn desc(&self) -> String {
        "License, advisory and crate ban policy.".to_string()
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, _: &RepoBuilder) -> Result<()> {
        let content = include_str!("../../../templates/deny.toml");
        write_entry(root, "deny.toml", content.as_bytes())?;
        Ok(())
    }
}
