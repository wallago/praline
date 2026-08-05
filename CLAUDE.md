# CLAUDE.md

praline is a TUI that scaffolds new repos. It ships a set of config-file
templates and writes the selected ones into a fresh project.

## Commands

Use `just` — it is the entry point, not raw cargo. `just --list` for the rest.

- `just check` — `cargo check --all-targets`
- `just lint` — clippy with `-D warnings`
- `just fmt` — rustfmt + nixfmt
- `just ci` — the full local gate; run this before pushing
- `just udeps` / `just audit` / `just deny` — dependency hygiene

## Generated files — do not edit directly

`.github/workflows/ci.yml` and `.github/workflows/cd.yml` are **generated** from
`templates/git/.github/workflows/`. Edit the template, then regenerate the root
copy by applying the same transform `substitute()` does: strip `{if:<tool>}`
blocks for unselected tools, then replace `{name}` → `praline`, `{owner}` →
`wallago`. Editing the root copy directly silently desyncs it from the template.

The same applies to `.github/PULL_REQUEST_TEMPLATE.md`, `LICENSE-*`, and the
`.github/ISSUE_TEMPLATE/` files — all have a source under `templates/git/`.

## The golden-reference invariant

Root config files (`clippy.toml`, `deny.toml`, `typos.toml`, `taplo.toml`,
`rustfmt.toml`, `committed.toml`, `codecov.yml`, `cliff.toml`, `flake.nix`,
`.editorconfig`) are emitted near-verbatim into every generated repo. praline
uses its own output. A change to one is a change to every repo praline will ever
create — keep the root copy and `templates/` copy in sync.

## Template substitution

`src/app/tool/mod.rs` rewrites every UTF-8 template file on the way out:

- `{name}`, `{desc}`, `{owner}` — replaced with the repo being generated.
- `{if:<tool>}` … `{endif:<tool>}` — the enclosed lines survive only if that
  tool is selected. Marker lines are always dropped. The marker is matched
  anywhere in the line, so it can hide behind any comment syntax (`#`, `//`,
  `<!-- -->`). Tool names are the strings returned by `Tool::name()`.

A tool that is registered but commented out in `src/app/mod.rs` counts as
unselected, so its `{if:}` blocks are stripped.

**Substitution is not applied everywhere.** Only the directory-based tools
(`git`, `rust`, `flake` — they go through `write_dir`) plus `claude` and `cliff`
call `substitute()`. The rest (`clippy`, `codecov`, `committed`, `deny`,
`editorconfig`, `envrc`, `just`, `rustfmt`, `taplo`, `typos`) write
`include_str!(...).as_bytes()` raw, so a `{name}` or `{if:}` marker added to
those templates is emitted **literally**. Route the tool through `substitute()`
first if you need placeholders in it.

## Code constraints

`Cargo.toml` sets `unwrap_used`, `expect_used`, `panic`, `todo`, `print_stdout`,
and `missing_docs_in_private_items` to warn, and CI runs clippy with
`-D warnings`. So: no `.unwrap()`, no `.expect()`, no bare `panic!`, and every
private item needs a doc comment. `unwrap_or`/`unwrap_or_default` are fine.

Arithmetic on `usize` state (scroll offsets, indices) panics on overflow in dev
builds — use `saturating_sub`/`saturating_add`. `clamp` panics when `min > max`,
so clamp the floor to the ceiling first when both are computed.

Edition 2024, MSRV 1.85 (CI enforces it via cargo-msrv).

## Verifying changes

The binary is a full-screen TUI, so it cannot be driven headlessly — `cargo run`
takes over the terminal and there is no scripted-input mode. Changes to
`src/tui/` get verified by reading, by `cargo check`, and by asking the user to
run it. Do not claim a UI change works without saying how it was checked.

Workflow changes can be verified: `actionlint` catches YAML, expression, and
embedded-shell errors, and it is worth running on the stripped-down render (no
optional tools selected) as well as the full one.

## Environment

Nix flake + direnv (`.envrc` is `use flake`). Tools like `taplo`, `actionlint`,
and `nixfmt` come from the flake or `nix run nixpkgs#<tool>`; don't assume they
are on the bare system. `.direnv/` is gitignored and contains nixpkgs sources —
exclude it from any repo-wide file scan.
