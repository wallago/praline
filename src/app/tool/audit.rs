use std::path::Path;

use crate::{
    app::{RepoBuilder, Tool, tool::category::Category},
    error::Result,
};

#[derive(Debug)]
pub(crate) struct Audit;

impl Tool for Audit {
    fn name(&self) -> String {
        "audit".to_string()
    }

    fn desc(&self) -> String {
        "Scans dependencies for RustSec advisories.".to_string()
    }

    fn category(&self) -> Category {
        Category::Security
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, _: &Path, _: &RepoBuilder) -> Result<()> {
        Ok(())
    }
}
