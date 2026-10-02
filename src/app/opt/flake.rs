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
    pub(super) fn def_flake() -> OptDef {
        OptDef {
            _desc: "Dependencies and environments setup.",
            _category: Some(Category::Env),
            default: true,
            emit: Emit::Dir(&FLAKE),
            ..OptDef::default()
        }
    }
    /// Flake package option definition.
    pub(super) fn def_flake_package() -> OptDef {
        OptDef {
            parent: Some(Self::Flake),
            _desc: ".",
            _category: Some(Category::Env),
            emit: Emit::Flag,
            ..OptDef::default()
        }
    }
    /// Flake home module option definition.
    pub(super) fn def_flake_module_home() -> OptDef {
        OptDef {
            parent: Some(Self::FlakePackage),
            _desc: "Dependencies and environments setup.",
            _category: Some(Category::Env),
            emit: Emit::Dir(&FLAKE_MODULE_HOME),
            ..OptDef::default()
        }
    }
    /// Flake nixos module option definition.
    pub(super) fn def_flake_module_nixos() -> OptDef {
        OptDef {
            parent: Some(Self::FlakePackage),
            _desc: "Dependencies and environments setup.",
            _category: Some(Category::Env),
            emit: Emit::Dir(&FLAKE_MODULE_NIXOS),
            ..OptDef::default()
        }
    }
}
