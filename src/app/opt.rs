use std::collections::HashSet;

use include_dir::{Dir, include_dir};

use crate::app::category::Category;

/// `just` templates.
static JUST: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/just");
/// `rust.firmware` templates.
static RUST_FIRMWARE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.firmware");

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum::EnumString,
    strum::EnumIter,
    strum::IntoStaticStr,
)]
pub(crate) enum OptId {
    #[strum(serialize = "just")]
    Just,
    #[strum(serialize = "rust")]
    Rust,
    #[strum(serialize = "rust.common")]
    RustCommon,
    #[strum(serialize = "rust.firmware")]
    RustFirmware,
}

impl OptId {
    /// This option's row in the table.
    pub(crate) fn def(self) -> OptDef {
        match self {
            Self::Just => OptDef {
                parent: None,
                requires: &[],
                desc: "Task runner recipes: check, lint, fmt, ci.",
                category: Category::Build,
                default: true,
                emit: Emit::Dir(&JUST),
            },
            Self::Rust => OptDef {
                parent: None,
                requires: &[],
                desc: ".",
                category: Category::Build,
                default: false,
                emit: Emit::Flag, // TODO => Change
            },
            Self::RustCommon => OptDef {
                parent: Some(Self::Rust),
                requires: &[],
                desc: ".",
                category: Category::Build,
                default: false,
                emit: Emit::Flag, // TODO => Change
            },
            Self::RustFirmware => OptDef {
                parent: Some(Self::Rust),
                requires: &[],
                desc: "no_std firmware crate, flashed with probe-rs.",
                category: Category::Build,
                default: false,
                emit: Emit::Dir(&RUST_FIRMWARE),
            },
        }
    }
}

impl OptId {
    /// Whether `id` takes part: checked, and so is everything it hangs off.
    pub(crate) fn is_active(self, checked: &HashSet<OptId>) -> bool {
        let def = self.def();
        checked.contains(&self)
            && def.parent.is_none_or(|parent| parent.is_active(checked))
            && def.requires.iter().all(|&req| req.is_active(checked))
    }
}

/// What an option writes.
#[derive(Debug)]
pub(crate) enum Emit {
    /// Nothing: only switches `{if:}` blocks elsewhere.
    Flag,
    /// A folder mirroring the repo root; its `_slots/` feeds hub files.
    Dir(&'static Dir<'static>),
}

pub(crate) struct OptDef {
    pub parent: Option<OptId>,
    pub requires: &'static [OptId],
    pub desc: &'static str,
    pub category: Category,
    pub default: bool,
    pub emit: Emit,
}
