use std::path::Path;

use super::write_entry;
use crate::{
    app::{RepoBuilder, Tool, tool::category::Category},
    error::Result,
};

#[derive(Debug)]
pub(crate) struct RustFmt;

impl Tool for RustFmt {
    fn name(&self) -> String {
        "rustfmt".to_string()
    }

    fn desc(&self) -> String {
        "Rust formatting style.".to_string()
    }

    fn category(&self) -> Category {
        Category::Format
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, _: &RepoBuilder) -> Result<()> {
        let content = include_str!("../../../templates/rustfmt.toml");
        write_entry(root, "rustfmt.toml", content.as_bytes())?;
        Ok(())
    }
}
