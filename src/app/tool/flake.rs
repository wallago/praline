use std::path::Path;

use include_dir::{Dir, include_dir};

use crate::app::{RepoBuilder, Tool, tool::category::Category};
use crate::prelude::*;

/// Nix file templates path.
static FLAKE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/nix");

/// Nix flake pinning the toolchain and dev shell.
#[derive(Debug)]
pub(crate) struct Flake;

impl Tool for Flake {
    fn name(&self) -> String {
        "flake".to_string()
    }

    fn desc(&self) -> String {
        "Nix flake pinning the toolchain and dev shell.".to_string()
    }

    fn category(&self) -> Category {
        Category::Env
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, repo: &RepoBuilder) -> Result<()> {
        write_dir(&FLAKE, root, repo)?;
        Ok(())
    }
}
