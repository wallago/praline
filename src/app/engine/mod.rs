use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use strum::IntoEnumIterator;

use crate::{
    app::{
        App,
        engine::{context::Ctx, output::render_dir},
        opt::{Emit, OptId, Scope},
    },
    error::EngineError::PathCollision,
    prelude::*,
};

/// `{if:}`/`{ifnot:}`/`{endif:}` blocks, kept or dropped by selected tool.
mod condition;
/// Read-only snapshot of the builder that every template renders against.
mod context;
/// Pulls the name out of a `{<marker><name>}` line.
mod marker;
/// Renders option folders into a [`Tree`] and writes it to disk.
mod output;
/// `github.release`'s slots, which only the list of apps can fill.
mod release;
/// `{slot:}` hubs that other options' `_slots/` files fill.
mod slot;
/// Renders one template: slots, then conditionals, then `{key}` placeholders.
mod template;
/// The root `Cargo.toml` of a repo with several apps.
mod workspace;

/// Checks that hold for every option combination.
#[cfg(test)]
mod tests;

pub(super) use output::write_tree;

/// A rendered repo: path relative to the repo root → file bytes.
pub(crate) type Tree = BTreeMap<PathBuf, Vec<u8>>;

/// Renders every app, then the files they share at the root.
///
/// With one app the root is that app's folder, so everything renders with
/// its context, as a plain single-crate repo. With several, each app's own
/// options (and its `justfile`) render in `apps/<name>/`, and the repo-wide
/// options render once at the root, plus a Cargo workspace over the Rust apps.
pub(super) fn generate(app: &App) -> Result<Tree> {
    let mut tree = Tree::new();
    let multi = app.is_multi();
    for at in 0..app.apps.len() {
        let ctx = Ctx::for_app(app, at);
        let prefix = app.prefix(at);
        for id in OptId::iter().filter(|id| ctx.active.contains(id)) {
            let own = !multi || id.def().scope == Scope::App || id == OptId::Just;
            if let (true, Emit::Dir(files)) = (own, id.def().emit) {
                render_dir(files, &ctx, &prefix, &mut tree)?;
            }
        }
    }
    if !multi {
        return Ok(tree);
    }
    let ctx = Ctx::for_root(app);
    for id in OptId::iter().filter(|id| ctx.active.contains(id)) {
        if let (Scope::Repo, Emit::Dir(files)) = (id.def().scope, id.def().emit) {
            render_dir(files, &ctx, Path::new(""), &mut tree)?;
        }
    }
    if ctx.workspace {
        insert(
            &mut tree,
            PathBuf::from("Cargo.toml"),
            workspace::manifest(app),
        )?;
    }
    Ok(tree)
}

/// Adds `bytes` at `path`, unless something is already there.
fn insert(tree: &mut Tree, path: PathBuf, bytes: Vec<u8>) -> Result<()> {
    if tree.contains_key(&path) {
        return Err(PathCollision(path).into());
    }
    tree.insert(path, bytes);
    Ok(())
}
