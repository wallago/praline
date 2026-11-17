use include_dir::{Dir, include_dir};

use crate::app::{
    category::Category,
    opt::{Emit, OptDef, OptId, Scope},
};

/// `editorconfig` templates.
static EDITORCONFIG: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/editorconfig");
/// `typos` templates.
static TYPOS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/typos");
/// `lychee` templates.
static LYCHEE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/lychee");
/// `readme` templates.
static README: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/readme");
/// `git` templates.
static GIT: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/git");
/// `github` templates.
static GITHUB: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/github");
/// `github.ci` templates.
static GITHUB_CI: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/github.ci");
/// `claude` templates.
static CLAUDE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/claude");

impl OptId {
    /// Editorconfig option definition.
    pub(super) fn def_editorconfig() -> OptDef {
        OptDef {
            desc: "`.editorconfig`, checked by editorconfig-checker.",
            _category: Some(Category::Format),
            default: true,
            emit: Emit::Dir(&EDITORCONFIG),
            scope: Scope::Repo,
            ..OptDef::default()
        }
    }

    /// Typos option definition.
    pub(super) fn def_typos() -> OptDef {
        OptDef {
            desc: "Spell-check the repo with typos.",
            _category: Some(Category::Lint),
            default: true,
            emit: Emit::Dir(&TYPOS),
            scope: Scope::Repo,
            ..OptDef::default()
        }
    }

    /// Lychee option definition.
    pub(super) fn def_lychee() -> OptDef {
        OptDef {
            desc: "Check the links in every Markdown file with lychee.",
            _category: Some(Category::Lint),
            default: true,
            emit: Emit::Dir(&LYCHEE),
            scope: Scope::Repo,
            ..OptDef::default()
        }
    }

    /// README option definition.
    pub(super) fn def_readme() -> OptDef {
        OptDef {
            desc: "`README.md` with setup, tasks, usage and licence.",
            _category: Some(Category::Git),
            default: true,
            emit: Emit::Dir(&README),
            scope: Scope::Repo,
            ..OptDef::default()
        }
    }

    /// Git option definition.
    pub(super) fn def_git() -> OptDef {
        OptDef {
            desc: "`.gitignore`, MIT + Apache licences, committed and git-cliff.",
            _category: Some(Category::Git),
            default: true,
            emit: Emit::Dir(&GIT),
            scope: Scope::Repo,
            ..OptDef::default()
        }
    }

    /// GitHub option definition.
    pub(super) fn def_github() -> OptDef {
        OptDef {
            parent: Some(Self::Git),
            desc: "Issue and PR templates, CODEOWNERS, dependabot.",
            _category: Some(Category::Git),
            default: true,
            emit: Emit::Dir(&GITHUB),
            scope: Scope::Repo,
            ..OptDef::default()
        }
    }

    /// GitHub CI option definition.
    pub(super) fn def_github_ci() -> OptDef {
        OptDef {
            parent: Some(Self::Github),
            // Jobs run `just <recipe>` inside the flake's CI shell.
            requires: &[Self::Flake, Self::Just],
            desc: "CI workflow running every `just ci` recipe in the flake's CI shell.",
            _category: Some(Category::Test),
            default: true,
            emit: Emit::Dir(&GITHUB_CI),
            scope: Scope::Repo,
            ..OptDef::default()
        }
    }

    /// Claude option definition.
    pub(super) fn def_claude() -> OptDef {
        OptDef {
            desc: "`CLAUDE.md`, rules and settings for Claude Code.",
            _category: Some(Category::Env),
            default: true,
            emit: Emit::Dir(&CLAUDE),
            scope: Scope::Repo,
            ..OptDef::default()
        }
    }
}
