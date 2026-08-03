use std::{fs, path::Path};

use include_dir::Dir;

use crate::{
    app::{RepoBuilder, tool::category::Category},
    error::Result,
};

pub(crate) mod category;
pub(super) mod claude;
pub(super) mod cliff;
pub(super) mod clippy;
pub(super) mod codecov;
pub(super) mod committed;
pub(super) mod deny;
pub(super) mod editorconfig;
pub(super) mod envrc;
pub(super) mod flake;
pub(super) mod git;
pub(super) mod just;
pub(super) mod rust;
pub(super) mod rustfmt;
pub(super) mod taplo;
pub(super) mod typos;

pub(crate) trait Tool: std::fmt::Debug {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn category(&self) -> Category;
    fn default_setup(&self) -> bool;
    fn gen_template(&self, root: &Path, repo: &RepoBuilder) -> Result<()>;
}

fn write_dir(dir: &Dir<'_>, root: &Path, repo: &RepoBuilder) -> Result<()> {
    for file in dir.files() {
        let name = file.path().to_string_lossy();
        let bytes = match file.contents_utf8() {
            Some(text) => substitute(text, repo),
            None => file.contents().to_vec(),
        };
        write_entry(root, &name, &bytes)?;
    }
    for sub in dir.dirs() {
        write_dir(sub, root, repo)?;
    }
    Ok(())
}

fn write_entry(root: &Path, name: &str, content: &[u8]) -> Result<()> {
    let path = root.join(name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, content)?;
    Ok(())
}

fn substitute(content: &str, repo: &RepoBuilder) -> Vec<u8> {
    content
        .replace("{name}", &repo.name)
        .replace("{desc}", &repo.desc)
        .replace("{owner}", &repo.owner)
        .into_bytes()
}
