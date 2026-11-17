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
/// `rust.mcp` templates.
static RUST_MCP: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.mcp");
/// `rust.mcp.http` templates.
static RUST_MCP_HTTP: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.mcp.http");
/// `rust.common.deny` templates.
static RUST_COMMON_DENY: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.common.deny");
/// `rust.common.audit` templates.
static RUST_COMMON_AUDIT: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.common.audit");
/// `rust.common.udeps` templates.
static RUST_COMMON_UDEPS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/rust.common.udeps");
/// `rust.common.codecov` templates.
static RUST_COMMON_CODECOV: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/rust.common.codecov");

impl OptId {
    /// Rust option definition.
    pub(super) fn def_rust() -> OptDef {
        OptDef {
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST),
            ..OptDef::default()
        }
    }

    /// Rust common option definition.
    pub(super) fn def_rust_common() -> OptDef {
        OptDef {
            parent: Some(Self::Rust),
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_COMMON),
            ..OptDef::default()
        }
    }

    /// Rust Firmware option definition.
    pub(super) fn def_rust_firmware() -> OptDef {
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
    pub(super) fn def_rust_server() -> OptDef {
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
    pub(super) fn def_rust_server_ws() -> OptDef {
        OptDef {
            parent: Some(Self::RustServer),
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_SERVER_WS),
            ..OptDef::default()
        }
    }

    /// Rust CAN layer option definition.
    pub(super) fn def_rust_can() -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_CAN),
            ..OptDef::default()
        }
    }

    /// Rust client Websocket option definition.
    pub(super) fn def_rust_client_ws() -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_CLIENT_WS),
            ..OptDef::default()
        }
    }

    /// Rust notify option definition.
    pub(super) fn def_rust_notif() -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            _desc: ".",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_NOTIF),
            ..OptDef::default()
        }
    }

    /// Rust MCP server option definition.
    pub(super) fn def_rust_mcp() -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            // Both own stdin: the MCP client speaks over it.
            conflicts: &[Self::RustClientWS],
            _desc: "MCP server over stdio, built on rmcp.",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_MCP),
            ..OptDef::default()
        }
    }

    /// Rust MCP over Streamable HTTP option definition.
    pub(super) fn def_rust_mcp_http() -> OptDef {
        OptDef {
            parent: Some(Self::RustMcp),
            requires: &[Self::RustServer],
            _desc: "Serve MCP over HTTP at /mcp, for remote clients.",
            _category: Some(Category::Build),
            emit: Emit::Dir(&RUST_MCP_HTTP),
            ..OptDef::default()
        }
    }

    /// Rust cargo-deny option definition.
    pub(super) fn def_rust_common_deny() -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            _desc: "Check licences, banned crates, sources and advisories with cargo-deny.",
            _category: Some(Category::Security),
            default: true,
            emit: Emit::Dir(&RUST_COMMON_DENY),
            ..OptDef::default()
        }
    }

    /// Rust cargo-audit option definition.
    pub(super) fn def_rust_common_audit() -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            _desc: "Audit dependencies against the RustSec advisory DB with cargo-audit.",
            _category: Some(Category::Security),
            default: true,
            emit: Emit::Dir(&RUST_COMMON_AUDIT),
            ..OptDef::default()
        }
    }

    /// Rust cargo-machete option definition.
    pub(super) fn def_rust_common_udeps() -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            _desc: "Fail on unused dependencies with cargo-machete.",
            _category: Some(Category::Lint),
            default: true,
            emit: Emit::Dir(&RUST_COMMON_UDEPS),
            ..OptDef::default()
        }
    }

    /// Rust Codecov option definition.
    pub(super) fn def_rust_common_codecov() -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            // The upload is a CI job; it needs the repo's CODECOV_TOKEN secret.
            requires: &[Self::GithubCi],
            _desc: "Coverage with cargo-llvm-cov, uploaded to Codecov from CI.",
            _category: Some(Category::Test),
            emit: Emit::Dir(&RUST_COMMON_CODECOV),
            ..OptDef::default()
        }
    }
}
