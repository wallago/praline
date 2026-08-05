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

/// Marker opening a tool-conditional template block.
const IF_MARKER: &str = "{if:";
/// Marker closing a tool-conditional template block.
const ENDIF_MARKER: &str = "{endif:";

fn substitute(content: &str, repo: &RepoBuilder) -> Vec<u8> {
    strip_conditionals(content, repo)
        .replace("{name}", &repo.name)
        .replace("{desc}", &repo.desc)
        .replace("{owner}", &repo.owner)
        .into_bytes()
}

/// Extracts the tool name out of a `{<marker><tool>}` line, if it has one.
///
/// The marker is matched anywhere in the line, so it can sit behind whatever
/// comment syntax the template's file format uses.
fn marker_tool<'a>(line: &'a str, marker: &str) -> Option<&'a str> {
    let start = line.find(marker)? + marker.len();
    let end = start + line[start..].find('}')?;
    Some(line[start..end].trim())
}

/// Drops `{if:<tool>}` / `{endif:<tool>}` blocks whose tool is not selected,
/// keeping the body of the ones whose tool is. The marker lines themselves are
/// removed either way.
fn strip_conditionals(content: &str, repo: &RepoBuilder) -> String {
    if !content.contains(IF_MARKER) {
        return content.to_string();
    }
    let mut out = String::with_capacity(content.len());
    let mut skipping: Option<&str> = None;
    for line in content.lines() {
        if let Some(tool) = marker_tool(line, ENDIF_MARKER) {
            if skipping == Some(tool) {
                skipping = None;
            }
            continue;
        }
        if let Some(tool) = marker_tool(line, IF_MARKER) {
            if skipping.is_none() && !repo.is_selected(tool) {
                skipping = Some(tool);
            }
            continue;
        }
        if skipping.is_none() {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}
