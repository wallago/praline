use std::path::Path;

use include_dir::{Dir, include_dir};

use crate::app::{RepoBuilder, Tool, tool::category::Category};
use crate::prelude::*;

/// Nix file templates path.
static NIX: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/nix");

/// Nix pinning the toolchain and dev shell.
#[derive(Debug)]
pub(crate) struct Nix;

impl Tool for Nix {
    fn name(&self) -> String {
        "nix".to_string()
    }

    fn desc(&self) -> String {
        "Nix pinning the toolchain and dev shell.".to_string()
    }

    fn category(&self) -> Category {
        Category::Env
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, repo: &RepoBuilder) -> Result<()> {
        write_dir(&NIX, root, repo)?;
        Ok(())
    }
}
