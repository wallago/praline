use std::path::Path;

use include_dir::{Dir, include_dir};

use crate::{
    app::{
        RepoBuilder, Tool,
        tool::{category::Category, write_dir},
    },
    error::Result,
};
static GIT: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/git");

#[derive(Debug)]
pub(crate) struct Git;

impl Tool for Git {
    fn name(&self) -> String {
        "git".to_string()
    }

    fn desc(&self) -> String {
        "Gitignore, CI/CD workflows, issue and PR templates.".to_string()
    }

    fn category(&self) -> Category {
        Category::Env
    }

    fn default_setup(&self) -> bool {
        true
    }

    fn gen_template(&self, root: &Path, repo: &RepoBuilder) -> Result<()> {
        write_dir(&GIT, root, repo)?;
        Ok(())
    }
}
