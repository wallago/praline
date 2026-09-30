use include_dir::{Dir, include_dir};

use crate::app::{
    category::Category,
    opt::{Emit, OptDef, OptId},
};

/// `flake` templates.
static FLAKE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/flake");
/// `flake.module.home` templates.
static FLAKE_MODULE_HOME: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/flake.module.home");
/// `flake.module.nixos` templates.
static FLAKE_MODULE_NIXOS: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/flake.module.nixos");

impl OptId {
    /// Flake option definition.
    pub(super) fn def_flake(self) -> OptDef {
        OptDef {
            parent: None,
            requires: &[],
            _desc: "Dependencies and environments setup.",
            _category: Category::Env,
            default: true,
            emit: Emit::Dir(&FLAKE),
        }
    }
    /// Flake package option definition.
    pub(super) fn def_flake_package(self) -> OptDef {
        OptDef {
            parent: Some(Self::Flake),
            requires: &[],
            _desc: ".",
            _category: Category::Build,
            default: false,
            emit: Emit::Flag,
        }
    }
    /// Flake home module option definition.
    pub(super) fn def_flake_module_home(self) -> OptDef {
        OptDef {
            parent: None,
            requires: &[],
            _desc: "Dependencies and environments setup.",
            _category: Category::Env,
            default: false,
            emit: Emit::Dir(&FLAKE_MODULE_HOME),
        }
    }
    /// Flake nixos module option definition.
    pub(super) fn def_flake_module_nixos(self) -> OptDef {
        OptDef {
            parent: None,
            requires: &[],
            _desc: "Dependencies and environments setup.",
            _category: Category::Env,
            default: false,
            emit: Emit::Dir(&FLAKE_MODULE_NIXOS),
        }
    }
}
