use std::collections::HashSet;

use include_dir::Dir;

use crate::app::category::Category;

/// Flake options.
mod flake;
/// Just options.
mod just;
/// Rust options.
mod rust;

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
    //------------------------------------------------------------------//
    //                               Just                               //
    //------------------------------------------------------------------//
    /// `justfile` with the check, lint, fmt and ci recipes.
    #[strum(serialize = "just")]
    Just,

    //------------------------------------------------------------------//
    //                              Flake                               //
    //------------------------------------------------------------------//
    /// `flake` and `direnv`, setup dependencies and environments.
    #[strum(serialize = "flake")]
    Flake,
    /// `flake` package.
    #[strum(serialize = "flake.package")]
    FlakePackage,
    /// `flake` home module build with package.
    #[strum(serialize = "flake.module.home")]
    FlakeModuleHome,
    /// `flake` nixos module build with package.
    #[strum(serialize = "flake.module.nixos")]
    FlakeModuleNixos,

    //------------------------------------------------------------------//
    //                               Rust                               //
    //------------------------------------------------------------------//
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
    /// Websocket layer for sever module.
    #[strum(serialize = "rust.server.ws")]
    RustServerWS,
    /// CAN bus module and its Cargo dependencies.
    #[strum(serialize = "rust.can")]
    RustCan,
    /// Client Websocket.
    #[strum(serialize = "rust.client_ws")]
    RustClientWS,
}

impl OptId {
    /// This option's row in the table.
    pub(crate) fn def(self) -> OptDef {
        match self {
            Self::Just => self.def_just(),
            Self::Flake => self.def_flake(),
            Self::FlakePackage => self.def_flake_package(),
            Self::FlakeModuleHome => self.def_flake_module_home(),
            Self::FlakeModuleNixos => self.def_flake_module_nixos(),
            Self::Rust => self.def_rust(),
            Self::RustCommon => self.def_rust_common(),
            Self::RustFirmware => self.def_rust_firmware(),
            Self::RustServer => self.def_rust_server(),
            Self::RustServerWS => self.def_rust_server_ws(),
            Self::RustCan => self.def_rust_can(),
            Self::RustClientWS => self.def_rust_client_ws(),
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
