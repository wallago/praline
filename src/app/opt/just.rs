use include_dir::{Dir, include_dir};

use crate::app::{
    category::Category,
    opt::{Emit, OptDef, OptId},
};

/// `just` templates.
static JUST: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/just");

impl OptId {
    /// Just option definition.
    pub(super) fn def_just(self) -> OptDef {
        OptDef {
            parent: None,
            requires: &[],
            _desc: "Task runner recipes: check, lint, fmt, ci.",
            _category: Category::Env,
            default: true,
            emit: Emit::Dir(&JUST),
        }
    }
}
