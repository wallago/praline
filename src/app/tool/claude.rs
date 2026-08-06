use std::path::Path;

use include_dir::{Dir, include_dir};

use crate::app::{RepoBuilder, Tool, tool::category::Category};
use crate::prelude::*;

/// Claude file templates path.
static CLAUDE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/claude");

/// Claude Code setup: CLAUDE.md, agents, commands.
#[derive(Debug)]
pub(crate) struct Claude;

impl Tool for Claude {
    fn name(&self) -> String {
        "claude".to_string()
    }

    fn desc(&self) -> String {
        "Claude Code setup: CLAUDE.md, agents, commands.".to_string()
    }

    fn category(&self) -> Category {
        Category::Env
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, repo: &RepoBuilder) -> Result<()> {
        write_dir(&CLAUDE, root, repo)?;
        Ok(())
    }
}
