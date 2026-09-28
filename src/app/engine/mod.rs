use std::{collections::BTreeMap, path::PathBuf};

use strum::IntoEnumIterator;

use crate::{
    app::{
        App,
        engine::{context::Ctx, output::render_dir},
        opt::{Emit, OptId},
    },
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
/// `{slot:}` hubs that other options' `_slots/` files fill.
mod slot;
/// Renders one template: slots, then conditionals, then `{key}` placeholders.
mod template;

/// Checks that hold for every option combination.
#[cfg(test)]
mod tests;

pub(super) use output::write_tree;

/// A rendered repo: path relative to the repo root → file bytes.
pub(crate) type Tree = BTreeMap<PathBuf, Vec<u8>>;

/// Renders every active option's folder into memory.
pub(super) fn generate(app: &App) -> Result<Tree> {
    let ctx = Ctx::new(app);
    let mut tree = Tree::new();
    for id in OptId::iter().filter(|id| ctx.active.contains(id)) {
        if let Emit::Dir(files) = id.def().emit {
            render_dir(files, &ctx, &mut tree)?;
        }
    }
    Ok(tree)
}
