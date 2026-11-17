use std::fs;
use std::path::Path;

use include_dir::Dir;

use crate::app::engine::Tree;
use crate::app::engine::context::Ctx;
use crate::app::engine::slot::SLOTS_DIR;
use crate::app::engine::template::substitute;
use crate::error::EngineError::PathCollision;
use crate::prelude::*;

/// Renders `dir` into `tree` under `prefix`, minus `_slots/` and minus files
/// that render empty.
pub(super) fn render_dir(dir: &Dir<'_>, ctx: &Ctx, prefix: &Path, tree: &mut Tree) -> Result<()> {
    for file in dir.files() {
        let bytes = match file.contents_utf8() {
            Some(text) => substitute(text, ctx)?,
            None => file.contents().to_vec(),
        };
        // A file wrapped whole in an `{if:}` that's off renders to nothing.
        if bytes.iter().all(u8::is_ascii_whitespace) {
            continue;
        }
        let path = prefix.join(file.path());
        if tree.contains_key(&path) {
            return Err(PathCollision(path).into());
        }
        tree.insert(path, bytes);
    }
    for sub in dir.dirs() {
        // Contributions for hub files, not part of the repo.
        if sub.path() == Path::new(SLOTS_DIR) {
            continue;
        }
        render_dir(sub, ctx, prefix, tree)?;
    }
    Ok(())
}

/// Writes every file of `tree` under `root`, creating parent directories.
pub(in crate::app) fn write_tree(tree: &Tree, root: &Path) -> Result<()> {
    for (path, bytes) in tree {
        let target = root.join(path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target, bytes)?;
    }
    Ok(())
}
