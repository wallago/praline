use std::path::Path;

use include_dir::{Dir, include_dir};

use crate::{
    app::{
        RepoBuilder, Tool,
        tool::{category::Category, write_dir},
    },
    error::Result,
};
static RUST: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust");

#[derive(Debug)]
pub(crate) struct Rust;

impl Tool for Rust {
    fn name(&self) -> String {
        "rust".to_string()
    }

    fn desc(&self) -> String {
        "Cargo manifest and starter crate layout.".to_string()
    }

    fn category(&self) -> Category {
        Category::Env
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, repo: &RepoBuilder) -> Result<()> {
        write_dir(&RUST, root, repo)?;
        Ok(())
    }
}
