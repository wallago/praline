use std::path::Path;

use crate::app::{RepoBuilder, Tool, tool::category::Category};
use crate::prelude::*;

/// Finds broken hyperlinks and mail addresses in websites and Markdown, HTML, and other file formats.
#[derive(Debug)]
pub(crate) struct Lychee;

impl Tool for Lychee {
    fn name(&self) -> String {
        "lychee".to_string()
    }

    fn desc(&self) -> String {
        "Finds broken hyperlinks and mail addresses in websites and Markdown, HTML, and other file formats.".to_string()
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
