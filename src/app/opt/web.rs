use include_dir::{Dir, include_dir};

use crate::app::{
    category::Category,
    opt::{Emit, OptDef, OptId},
};

/// `web.react` templates.
static WEB_REACT: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/web.react");
/// `web.pages` templates.
static WEB_PAGES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/web.pages");

impl OptId {
    /// Web option definition: a group, switching the `{if:web}` blocks the
    /// repo-wide files share between every web stack.
    pub(super) fn def_web() -> OptDef {
        OptDef {
            // Both own `src/` and the flake package.
            conflicts: &[Self::Rust, Self::FlakePackage],
            _desc: "Website or web app, built with Node and pnpm.",
            _category: Some(Category::Build),
            emit: Emit::Flag,
            ..OptDef::default()
        }
    }

    /// Web React option definition.
    pub(super) fn def_web_react() -> OptDef {
        OptDef {
            parent: Some(Self::Web),
            _desc: "Vite + React + TypeScript app, tested with Vitest, linted by Biome.",
            _category: Some(Category::Build),
            default: true,
            emit: Emit::Dir(&WEB_REACT),
            ..OptDef::default()
        }
    }

    /// Web Pages option definition.
    pub(super) fn def_web_pages() -> OptDef {
        OptDef {
            parent: Some(Self::WebReact),
            requires: &[Self::GithubCi],
            _desc: "Deploy the built site to GitHub Pages on every push to main.",
            _category: Some(Category::Build),
            emit: Emit::Dir(&WEB_PAGES),
            ..OptDef::default()
        }
    }
}
