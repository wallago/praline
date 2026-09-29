use std::collections::HashSet;

use include_dir::{Dir, include_dir};

use crate::app::category::Category;

/// `just` templates.
static JUST: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/just");
/// `flake` templates.
static FLAKE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/flake");
/// `rust.firmware` templates.
static RUST_FIRMWARE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.firmware");
/// `rust` templates.
static RUST: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust");
/// `rust.common` templates.
static RUST_COMMON: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.common");
/// `rust.server` templates.
static RUST_SERVER: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.server");
/// `rust.server.can` templates.
static RUST_SERVER_CAN: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.server.can");

/// A selectable option, serialized as the name of its `templates/` folder.
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
    /// `justfile` with the check, lint, fmt and ci recipes.
    #[strum(serialize = "just")]
    Just,
    /// `flake` and `direnv`, setup dependencies and environments.
    #[strum(serialize = "flake")]
    Flake,
    /// Parent of the Rust options; writes deps for other options.
    #[strum(serialize = "rust")]
    Rust,
    /// Base crate: `Cargo.toml`, entry point, args, config, error and prelude.
    #[strum(serialize = "rust.common")]
    RustCommon,
    /// `no_std` firmware crate, flashed with probe-rs.
    #[strum(serialize = "rust.firmware")]
    RustFirmware,
    /// Server module and its Cargo dependencies.
    #[strum(serialize = "rust.server")]
    RustServer,
    /// CAN bus support for the server; no templates yet.
    #[strum(serialize = "rust.server.can")]
    RustServerCan,
}

impl OptId {
    /// This option's row in the table.
    pub(crate) fn def(self) -> OptDef {
        match self {
            Self::Just => OptDef {
                parent: None,
                requires: &[],
                _desc: "Task runner recipes: check, lint, fmt, ci.",
                _category: Category::Env,
                default: true,
                emit: Emit::Dir(&JUST),
            },
            Self::Flake => OptDef {
                parent: None,
                requires: &[],
                _desc: "Dependencies and environments setup.",
                _category: Category::Env,
                default: true,
                emit: Emit::Dir(&FLAKE),
            },
            Self::Rust => OptDef {
                parent: None,
                requires: &[],
                _desc: ".",
                _category: Category::Build,
                default: false,
                emit: Emit::Dir(&RUST),
            },
            Self::RustCommon => OptDef {
                parent: Some(Self::Rust),
                requires: &[],
                _desc: ".",
                _category: Category::Build,
                default: false,
                emit: Emit::Dir(&RUST_COMMON),
            },
            Self::RustFirmware => OptDef {
                parent: Some(Self::Rust),
                requires: &[],
                _desc: "no_std firmware crate, flashed with probe-rs.",
                _category: Category::Build,
                default: false,
                emit: Emit::Dir(&RUST_FIRMWARE),
            },
            Self::RustServer => OptDef {
                parent: Some(Self::Rust),
                requires: &[],
                _desc: ".",
                _category: Category::Build,
                default: false,
                emit: Emit::Dir(&RUST_SERVER),
            },
            Self::RustServerCan => OptDef {
                parent: Some(Self::RustServer),
                requires: &[],
                _desc: ".",
                _category: Category::Build,
                default: false,
                emit: Emit::Dir(&RUST_SERVER_CAN),
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

/// How an option fits in the tree and what it writes.
pub(crate) struct OptDef {
    /// Option this one sits under; it only takes part if the parent does.
    pub parent: Option<OptId>,
    /// Other options that must also take part for this one to.
    pub requires: &'static [OptId],
    /// One-line summary shown in the UI.
    pub _desc: &'static str,
    /// Group it's listed under.
    pub _category: Category,
    /// Whether it starts checked.
    pub default: bool,
    /// What it writes when it's active.
    pub emit: Emit,
}
