/// Group a tool is listed and colored under.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Category {
    /// Version control.
    Git,
    /// Environment and tooling setup.
    Env,
    /// Build the project.
    Build,
    /// Format code.
    Format,
    /// Lint code.
    Lint,
    /// Run tests.
    Test,
    /// Scan for security issues.
    Security,
    /// Publish or release.
    Release,
}

impl Category {
    /// Label shown in the CATEGORY column.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Git => "git",
            Self::Env => "env",
            Self::Build => "build",
            Self::Format => "format",
            Self::Lint => "lint",
            Self::Test => "test",
            Self::Security => "security",
            Self::Release => "release",
        }
    }
}

/// A selectable tool, shaped like a row of the engine's option table.
#[derive(Debug)]
pub(crate) struct Tool {
    /// Key used by presets and `needs`; `rust.clippy` hangs off `rust`.
    pub(crate) id: &'static str,
    /// Group the tool is listed and colored under.
    pub(crate) category: Category,
    /// One-line blurb.
    pub(crate) desc: &'static str,
    /// Files it writes; empty for a flag that only switches `{if:}` blocks.
    pub(crate) writes: &'static [&'static str],
    /// Tool that must be on too for this one to count.
    pub(crate) needs: Option<&'static str>,
    /// Files of other tools it adds lines to.
    pub(crate) also_in: &'static [&'static str],
}

impl Tool {
    /// A tool with no requirement that only touches its own files.
    const fn new(
        id: &'static str,
        category: Category,
        desc: &'static str,
        writes: &'static [&'static str],
    ) -> Self {
        Self {
            id,
            category,
            desc,
            writes,
            needs: None,
            also_in: &[],
        }
    }

    /// Only counts while `id` is on too.
    const fn needs(mut self, id: &'static str) -> Self {
        self.needs = Some(id);
        self
    }

    /// Adds lines to `files`, which other tools write.
    const fn also_in(mut self, files: &'static [&'static str]) -> Self {
        self.also_in = files;
        self
    }

    /// Name shown in the list: the id without its parent.
    pub(crate) fn name(&self) -> &'static str {
        self.id.rsplit_once('.').map_or(self.id, |(_, name)| name)
    }

    /// Id of the tool this one hangs off, if any.
    pub(crate) fn parent(&self) -> Option<&'static str> {
        self.id.rsplit_once('.').map(|(parent, _)| parent)
    }
}

/// Every tool, in list order; children follow their parent.
pub(crate) static TOOLS: [Tool; 20] = [
    Tool::new(
        "git",
        Category::Git,
        ".gitignore, CI workflows, issue and PR templates.",
        &[
            ".gitignore",
            ".github/workflows/ci.yml",
            ".github/workflows/cd.yml",
            ".github/dependabot.yml",
            ".github/PULL_REQUEST_TEMPLATE.md",
            ".github/ISSUE_TEMPLATE/bug_report.md",
            ".github/ISSUE_TEMPLATE/feature_request.md",
            "LICENSE-MIT",
            "LICENSE-APACHE",
        ],
    ),
    Tool::new(
        "editorconfig",
        Category::Format,
        "Editor-agnostic whitespace and indent rules.",
        &[".editorconfig"],
    ),
    Tool::new(
        "typos",
        Category::Lint,
        "Source spell-checker with a project word list.",
        &["typos.toml"],
    )
    .also_in(&["justfile", ".github/workflows/ci.yml"]),
    Tool::new(
        "lychee",
        Category::Lint,
        "Markdown link checker, run in CI.",
        &[],
    )
    .also_in(&[".github/workflows/ci.yml"]),
    Tool::new(
        "nix",
        Category::Env,
        "Flake with dev shell, package and home-manager module.",
        &["flake.nix", "nix/package.nix", "nix/hm-module.nix"],
    ),
    Tool::new(
        "envrc",
        Category::Env,
        "direnv hook that loads the flake dev shell.",
        &[".envrc"],
    )
    .needs("nix"),
    Tool::new(
        "just",
        Category::Build,
        "Task runner recipes: check, lint, fmt, ci.",
        &[
            "justfile",
            "just/common.just",
            "just/rust.just",
            "just/nix.just",
        ],
    ),
    Tool::new(
        "claude",
        Category::Env,
        "Claude Code setup: CLAUDE.md, rules, settings.",
        &[
            "CLAUDE.md",
            ".claude/settings.json",
            ".claude/rules/rust.md",
            ".claude/rules/nix.md",
        ],
    ),
    Tool::new(
        "cliff",
        Category::Release,
        "Changelog generated from Conventional Commits.",
        &["cliff.toml"],
    ),
    Tool::new(
        "committed",
        Category::Release,
        "Checks commit messages against Conventional Commits.",
        &["committed.toml"],
    )
    .also_in(&[".github/workflows/ci.yml"]),
    Tool::new(
        "taplo",
        Category::Format,
        "TOML formatter settings.",
        &["taplo.toml"],
    ),
    Tool::new(
        "rust",
        Category::Build,
        "Cargo manifest and starter crate layout.",
        &["Cargo.toml", "src/main.rs", "src/lib.rs"],
    ),
    Tool::new(
        "rust.common",
        Category::Build,
        "CLI scaffold: clap args, config, errors, prelude.",
        &[
            "src/args.rs",
            "src/config/mod.rs",
            "src/error.rs",
            "src/prelude.rs",
        ],
    ),
    Tool::new(
        "rust.firmware",
        Category::Build,
        "no_std firmware crate, flashed with probe-rs.",
        &["memory.x", ".cargo/config.toml", "just/firmware.just"],
    )
    .also_in(&["justfile"]),
    Tool::new(
        "rust.clippy",
        Category::Lint,
        "Clippy thresholds; pedantic warnings denied in CI.",
        &["clippy.toml"],
    )
    .also_in(&["justfile", ".github/workflows/ci.yml"]),
    Tool::new(
        "rust.rustfmt",
        Category::Format,
        "Formatter settings: width, import grouping.",
        &["rustfmt.toml"],
    ),
    Tool::new(
        "rust.deny",
        Category::Security,
        "cargo-deny: licenses, bans, sources, advisories.",
        &["deny.toml"],
    ),
    Tool::new(
        "rust.audit",
        Category::Security,
        "cargo-audit against the RustSec advisory DB.",
        &[],
    )
    .also_in(&["justfile", ".github/workflows/ci.yml"]),
    Tool::new(
        "rust.machete",
        Category::Lint,
        "Fails the build on unused dependencies.",
        &[],
    )
    .also_in(&["justfile"]),
    Tool::new(
        "rust.codecov",
        Category::Test,
        "Coverage upload settings for cargo llvm-cov.",
        &["codecov.yml"],
    )
    .also_in(&[".github/workflows/ci.yml"]),
];

/// Index of the tool with `id` in [`TOOLS`].
pub(crate) fn position(id: &str) -> Option<usize> {
    TOOLS.iter().position(|tool| tool.id == id)
}

/// A named bundle of tools applied in one go.
#[derive(Debug)]
pub(crate) struct Preset {
    /// Name shown in the picker and the repo line.
    pub(crate) name: &'static str,
    /// One-line blurb.
    pub(crate) desc: &'static str,
    /// Ids of the tools it turns on; `None` means every tool.
    pub(crate) tools: Option<&'static [&'static str]>,
}

impl Preset {
    /// Whether the preset turns on the tool with `id`.
    pub(crate) fn includes(&self, id: &str) -> bool {
        self.tools.is_none_or(|tools| tools.contains(&id))
    }

    /// How many tools the preset turns on.
    pub(crate) fn count(&self) -> usize {
        self.tools.map_or(TOOLS.len(), <[_]>::len)
    }
}

/// Every preset, smallest first.
pub(crate) static PRESETS: [Preset; 4] = [
    Preset {
        name: "minimal",
        desc: "git and editor hygiene only",
        tools: Some(&["git", "editorconfig", "typos"]),
    },
    Preset {
        name: "repo",
        desc: "language-agnostic repo scaffolding",
        tools: Some(&[
            "git",
            "editorconfig",
            "typos",
            "lychee",
            "nix",
            "envrc",
            "just",
            "claude",
            "cliff",
            "committed",
        ]),
    },
    Preset {
        name: "rust",
        desc: "everyday Rust project setup",
        tools: Some(&[
            "git",
            "editorconfig",
            "typos",
            "lychee",
            "nix",
            "envrc",
            "just",
            "claude",
            "cliff",
            "committed",
            "taplo",
            "rust",
            "rust.common",
            "rust.clippy",
            "rust.rustfmt",
        ]),
    },
    Preset {
        name: "full",
        desc: "every tool praline ships",
        tools: None,
    },
];

/// Body shown for `path`, before `{owner}`, `{name}`, `{desc}`, `{path}` and
/// `{tool}` are filled in.
pub(crate) fn body(path: &str) -> &'static str {
    match path {
        "justfile" => JUSTFILE,
        "Cargo.toml" => CARGO_TOML,
        "src/main.rs" => MAIN_RS,
        ".editorconfig" => EDITORCONFIG,
        "clippy.toml" => CLIPPY_TOML,
        "flake.nix" => FLAKE_NIX,
        ".github/workflows/ci.yml" => CI_YML,
        _ => GENERIC,
    }
}

/// Fallback body for files without a sample.
const GENERIC: &str = "\
placeholder for {path}

The template engine isn't wired in yet.
Once it is, this shows what `{tool}` renders here.
";

/// Sample `justfile`.
const JUSTFILE: &str = "\
# {name} — {desc}
import 'just/common.just'
import 'just/rust.just'
import 'just/nix.just'

# List all commands
default:
    @just --list

# The full local gate: run this before pushing
ci: fmt-check lint test
";

/// Sample `Cargo.toml`.
const CARGO_TOML: &str = r#"[package]
name         = "{name}"
version      = "0.1.0"
description  = "{desc}"
authors      = ["{owner}"]
license      = "MIT OR Apache-2.0"
edition      = "2024"
rust-version = "1.88"

[dependencies]
clap = { version = "4", features = ["derive"] }

[lints.clippy]
pedantic = { level = "warn", priority = -1 }
"#;

/// Sample `src/main.rs`.
const MAIN_RS: &str = r#"//! {name}: {desc}

fn main() {
    println!("Hello from {name}!");
}
"#;

/// Sample `.editorconfig`.
const EDITORCONFIG: &str = "\
# https://editorconfig.org
root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true

[*.{rs,toml}]
indent_style = space
indent_size = 4
";

/// Sample `clippy.toml`.
const CLIPPY_TOML: &str = r#"# Thresholds for clippy; pedantic warnings are denied in CI.
msrv = "1.88"
too-many-arguments-threshold = 8
allow-unwrap-in-tests = true
allow-expect-in-tests = true
"#;

/// Sample `flake.nix`.
const FLAKE_NIX: &str = r#"{
  description = "{desc}";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: {
    # dev shell, package and home-manager module for {name}
  };
}
"#;

/// Sample `.github/workflows/ci.yml`.
const CI_YML: &str = "\
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: DeterminateSystems/nix-installer-action@main
      - run: nix develop --command just ci
";
