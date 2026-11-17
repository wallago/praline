use std::collections::HashSet;

use include_dir::Dir;
use strum::IntoEnumIterator;

use crate::app::category::Category;

/// Flake options.
mod flake;
/// Just options.
mod just;
/// Repo-wide options: checks, docs, git, GitHub and Claude.
mod repo;
/// Rust options.
mod rust;
/// Web options.
mod web;

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
    /// Notifier.
    #[strum(serialize = "rust.notif")]
    RustNotif,
    /// MCP server over stdio, built on `rmcp`.
    #[strum(serialize = "rust.mcp")]
    RustMcp,
    /// Serve the MCP server over Streamable HTTP on the server, not stdio.
    #[strum(serialize = "rust.mcp.http")]
    RustMcpHttp,
    /// `cargo machete`: unused dependencies.
    #[strum(serialize = "rust.common.udeps")]
    RustCommonUdeps,
    /// `cargo deny`: licences, bans, sources and advisories.
    #[strum(serialize = "rust.common.deny")]
    RustCommonDeny,
    /// `cargo audit` against the `RustSec` advisory DB.
    #[strum(serialize = "rust.common.audit")]
    RustCommonAudit,
    /// Test coverage with cargo-llvm-cov, uploaded to Codecov from CI.
    #[strum(serialize = "rust.common.codecov")]
    RustCommonCodecov,

    //------------------------------------------------------------------//
    //                               Web                                //
    //------------------------------------------------------------------//
    /// Parent of the web stacks; switches the repo-wide `{if:web}` blocks.
    #[strum(serialize = "web")]
    Web,
    /// Vite + React + TypeScript app, with Vitest and Biome.
    #[strum(serialize = "web.react")]
    WebReact,
    /// GitHub Pages deploy of the built site.
    #[strum(serialize = "web.pages")]
    WebPages,

    //------------------------------------------------------------------//
    //                               Repo                               //
    //------------------------------------------------------------------//
    /// `.editorconfig`, checked by editorconfig-checker.
    #[strum(serialize = "editorconfig")]
    Editorconfig,
    /// `typos.toml`, spell-check.
    #[strum(serialize = "typos")]
    Typos,
    /// `lychee.toml`, link check.
    #[strum(serialize = "lychee")]
    Lychee,
    /// `README.md`.
    #[strum(serialize = "readme")]
    Readme,
    /// `.gitignore`, licences, commit format and changelog.
    #[strum(serialize = "git")]
    Git,
    /// `.github/` templates and dependabot.
    #[strum(serialize = "github")]
    Github,
    /// `.github/workflows/ci.yml`, running `just ci` recipe by recipe.
    #[strum(serialize = "github.ci")]
    GithubCi,
    /// `CLAUDE.md` and `.claude/`.
    #[strum(serialize = "claude")]
    Claude,
}

impl OptId {
    /// This option's row in the table.
    pub(crate) fn def(self) -> OptDef {
        match self {
            Self::Just => Self::def_just(),
            Self::Flake => Self::def_flake(),
            Self::FlakePackage => Self::def_flake_package(),
            Self::FlakeModuleHome => Self::def_flake_module_home(),
            Self::FlakeModuleNixos => Self::def_flake_module_nixos(),
            Self::Rust => Self::def_rust(),
            Self::RustCommon => Self::def_rust_common(),
            Self::RustFirmware => Self::def_rust_firmware(),
            Self::RustServer => Self::def_rust_server(),
            Self::RustServerWS => Self::def_rust_server_ws(),
            Self::RustCan => Self::def_rust_can(),
            Self::RustClientWS => Self::def_rust_client_ws(),
            Self::RustNotif => Self::def_rust_notif(),
            Self::RustMcp => Self::def_rust_mcp(),
            Self::RustMcpHttp => Self::def_rust_mcp_http(),
            Self::RustCommonUdeps => Self::def_rust_common_udeps(),
            Self::RustCommonDeny => Self::def_rust_common_deny(),
            Self::RustCommonAudit => Self::def_rust_common_audit(),
            Self::RustCommonCodecov => Self::def_rust_common_codecov(),
            Self::Web => Self::def_web(),
            Self::WebReact => Self::def_web_react(),
            Self::WebPages => Self::def_web_pages(),
            Self::Editorconfig => Self::def_editorconfig(),
            Self::Typos => Self::def_typos(),
            Self::Lychee => Self::def_lychee(),
            Self::Readme => Self::def_readme(),
            Self::Git => Self::def_git(),
            Self::Github => Self::def_github(),
            Self::GithubCi => Self::def_github_ci(),
            Self::Claude => Self::def_claude(),
        }
    }
}

impl OptId {
    /// Whether the TUI starts with it checked: a default under defaults only,
    /// so no option shows checked under an unchecked parent.
    pub(crate) fn starts_checked(self) -> bool {
        let def = self.def();
        def.default && def.parent.is_none_or(Self::starts_checked)
    }

    /// Whether `id` takes part: checked, and so is everything it hangs off.
    pub(crate) fn is_active(self, checked: &HashSet<OptId>) -> bool {
        let def = self.def();
        checked.contains(&self)
            && def.parent.is_none_or(|parent| parent.is_active(checked))
            && def.requires.iter().all(|&req| req.is_active(checked))
    }

    /// Whether `self` and `other` can't take part together, declared on either side.
    pub(crate) fn conflicts_with(self, other: OptId) -> bool {
        self.def().conflicts.contains(&other) || other.def().conflicts.contains(&self)
    }

    /// Sorted list options.
    pub(crate) fn sorted_list(list: &mut Vec<Self>, parent: Option<Self>) {
        for opt in Self::iter().filter(|opt| opt.def().parent == parent) {
            list.push(opt);
            Self::sorted_list(list, Some(opt));
        }
    }
}

/// What an option writes.
#[derive(Debug, Default)]
pub(crate) enum Emit {
    /// Nothing: only switches `{if:}` blocks elsewhere.
    #[default]
    Flag,
    /// A folder mirroring the repo root; its `_slots/` feeds hub files.
    Dir(&'static Dir<'static>),
}

/// How an option fits in the tree and what it writes.
#[derive(Default)]
pub(crate) struct OptDef {
    /// Option this one sits under; it only takes part if the parent does.
    pub parent: Option<OptId>,
    /// Other options that must also take part for this one to.
    pub requires: &'static [OptId],
    /// Other options that can't take part with this one to.
    pub conflicts: &'static [OptId],
    /// One-line summary shown in the UI.
    pub _desc: &'static str,
    /// Group it's listed under.
    pub _category: Option<Category>,
    /// Whether it gets checked with its parent: at start for a top-level
    /// option, and in the TUI when the parent gets checked.
    pub default: bool,
    /// What it writes when it's active.
    pub emit: Emit,
}
