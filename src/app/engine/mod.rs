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

mod condition;
mod context;
mod marker;
mod output;
mod slot;
mod template;

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
