use include_dir::{Dir, include_dir};

use crate::app::{
    category::Category,
    opt::{Emit, OptDef, OptId},
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
/// `codecov` templates.
static CODECOV: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/codecov");
/// `claude` templates.
static CLAUDE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/claude");

impl OptId {
    /// Editorconfig option definition.
    pub(super) fn def_editorconfig() -> OptDef {
        OptDef {
            _desc: "`.editorconfig`, checked by editorconfig-checker.",
            _category: Some(Category::Format),
            default: true,
            emit: Emit::Dir(&EDITORCONFIG),
            ..OptDef::default()
        }
    }

    /// Typos option definition.
    pub(super) fn def_typos() -> OptDef {
        OptDef {
            _desc: "Spell-check the repo with typos.",
            _category: Some(Category::Lint),
            default: true,
            emit: Emit::Dir(&TYPOS),
            ..OptDef::default()
        }
    }

    /// Lychee option definition.
    pub(super) fn def_lychee() -> OptDef {
        OptDef {
            _desc: "Check the links in every Markdown file with lychee.",
            _category: Some(Category::Lint),
            default: true,
            emit: Emit::Dir(&LYCHEE),
            ..OptDef::default()
        }
    }

    /// README option definition.
    pub(super) fn def_readme() -> OptDef {
        OptDef {
            _desc: "`README.md` with setup, tasks, usage and licence.",
            _category: Some(Category::Git),
            default: true,
            emit: Emit::Dir(&README),
            ..OptDef::default()
        }
    }

    /// Git option definition.
    pub(super) fn def_git() -> OptDef {
        OptDef {
            _desc: "`.gitignore`, MIT + Apache licences, committed and git-cliff.",
            _category: Some(Category::Git),
            default: true,
            emit: Emit::Dir(&GIT),
            ..OptDef::default()
        }
    }

    /// GitHub option definition.
    pub(super) fn def_github() -> OptDef {
        OptDef {
            parent: Some(Self::Git),
            _desc: "Issue and PR templates, CODEOWNERS, dependabot.",
            _category: Some(Category::Git),
            default: true,
            emit: Emit::Dir(&GITHUB),
            ..OptDef::default()
        }
    }

    /// GitHub CI option definition.
    pub(super) fn def_github_ci() -> OptDef {
        OptDef {
            parent: Some(Self::Github),
            // Jobs run `just <recipe>` inside the flake's CI shell.
            requires: &[Self::Flake, Self::Just],
            _desc: "CI workflow running every `just ci` recipe in the flake's CI shell.",
            _category: Some(Category::Test),
            default: true,
            emit: Emit::Dir(&GITHUB_CI),
            ..OptDef::default()
        }
    }

    /// Codecov option definition.
    pub(super) fn def_codecov() -> OptDef {
        OptDef {
            parent: Some(Self::RustCommon),
            // The upload is a CI job; it needs the repo's CODECOV_TOKEN secret.
            requires: &[Self::GithubCi],
            _desc: "Coverage with cargo-llvm-cov, uploaded to Codecov from CI.",
            _category: Some(Category::Test),
            emit: Emit::Dir(&CODECOV),
            ..OptDef::default()
        }
    }

    /// Claude option definition.
    pub(super) fn def_claude() -> OptDef {
        OptDef {
            _desc: "`CLAUDE.md`, rules and settings for Claude Code.",
            _category: Some(Category::Env),
            default: true,
            emit: Emit::Dir(&CLAUDE),
            ..OptDef::default()
        }
    }
}
