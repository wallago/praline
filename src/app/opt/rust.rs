use include_dir::{Dir, include_dir};

use crate::app::{
    category::Category,
    opt::{Emit, OptDef, OptId},
};

/// `rust.firmware` templates.
static RUST_FIRMWARE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.firmware");
/// `rust` templates.
static RUST: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust");
/// `rust.common` templates.
static RUST_COMMON: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.common");
/// `rust.server` templates.
static RUST_SERVER: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.server");
/// `rust.server.ws` templates.
static RUST_SERVER_WS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.server.ws");
/// `rust.can` templates.
static RUST_CAN: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.can");
/// `rust.client_ws` templates.
static RUST_CLIENT_WS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.client_ws");
/// `rust.notify` templates.
static RUST_NOTIF: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.notif");

impl OptId {
    /// Rust option definition.
    pub(super) fn def_rust(self) -> OptDef {
        OptDef {
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST),
            ..OptDef::default()
        }
    }

    /// Rust common option definition.
    pub(super) fn def_rust_common(self) -> OptDef {
        OptDef {
            parent: Some(Self::Rust),
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_COMMON),
            ..OptDef::default()
        }
    }

    /// Rust Firmware option definition.
    pub(super) fn def_rust_firmware(self) -> OptDef {
        OptDef {
            parent: Some(Self::Rust),
            conflicts: &[Self::RustCommon],
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_FIRMWARE),
            ..OptDef::default()
        }
    }

    /// Rust Server option definition.
    pub(super) fn def_rust_server(self) -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            conflicts: &[Self::RustClientWS],
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_SERVER),
            ..OptDef::default()
        }
    }

    /// Rust Server Websocket layer option definition.
    pub(super) fn def_rust_server_ws(self) -> OptDef {
        OptDef {
            parent: Some(Self::RustServer),
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_SERVER_WS),
            ..OptDef::default()
        }
    }

    /// Rust CAN layer option definition.
    pub(super) fn def_rust_can(self) -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_CAN),
            ..OptDef::default()
        }
    }

    /// Rust client Websocket option definition.
    pub(super) fn def_rust_client_ws(self) -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_CLIENT_WS),
            ..OptDef::default()
        }
    }

    /// Rust notify option definition.
    pub(super) fn def_rust_notif(self) -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_NOTIF),
            ..OptDef::default()
        }
    }
}
