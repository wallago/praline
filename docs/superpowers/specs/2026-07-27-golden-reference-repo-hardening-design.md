# Golden-Reference Repo Hardening — Design

**Date:** 2026-07-27
**Status:** Approved (pending spec review)

## Context

`repo-builder` is a TUI tool that generates fully-configured repositories. The
config files in *this* repo are the **golden reference** the tool embeds and
emits (with a handful of tokens substituted per generated project). Because the
tool copies these files verbatim, any bug here propagates into every generated
repo. This pass makes the reference correct, consistent, and best-in-class.

Two settled decisions frame the work:

- **Release target matrix:** Linux `x86_64` + `aarch64`, **musl-static via
  `cross`**. Drop `riscv64` and the broken gnu matrix.
- **Publish tooling:** keep the **jujutsu**-based `just publish` recipe.

Out of scope: reading/refactoring `src/**` (the tool's own code); distribution
mechanism of the templates (the tool already embeds them).

## Goals

1. Every config file is correct — no leftover-from-another-project values, no
   settings that would fail `cargo publish`, `cargo msrv`, or a release build.
2. Config is internally consistent (`main` everywhere, matrix matches CI↔CD,
   MSRV declared where it is verified).
3. New quality/security tooling that is on-theme for a repo that emits CI.
4. The substitution tokens the TUI will inject are catalogued so the embedding
   step is unambiguous.

## Group A — Correctness fixes

These are defects, not preferences.

### A1. CI build job (`.github/workflows/ci.yml`)
- Replace the 3 gnu targets (which cannot link without cross-compilation) with
  **2 musl targets** built through `cross`:
  `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`.
- Install `cross` (via `taiki-e/install-action`) and build with
  `cross build --locked --target <target>`.
- Fix the leftover `binsider` artifact **name** (`binsider-…-assets`) and
  **path** (`target/debug/binsider*`) → `repo-builder`.

### A2. CD workflow (`.github/workflows/cd.yml`)
- Replace the archived/deprecated `actions-rs/toolchain` and `actions-rs/cargo`
  with `dtolnay/rust-toolchain@stable` + `cross`.
- Matrix → the same 2 musl targets as CI.
- Preserve the existing GPG detached signing, `sha512` checksums, and
  `git-cliff` release-notes generation.

### A3. `Cargo.toml`
- `categories = ["tui repo builder tool"]` is **invalid** (not a crates.io
  category slug) and breaks `cargo publish`. Replace with valid slugs:
  `["command-line-utilities", "development-tools"]`.
- Add `rust-version = "1.85"` (edition-2024 floor) so the MSRV CI job has a
  declared version to verify against.

### A4. `codecov.yml`
- `status.project.default.branches: [master]` → `[main]` (repo uses `main`).

### A5. `justfile` (`just/rust.just`)
- `fmt` references `flake.nix package.nix module.nix` — wrong paths, and
  `module.nix` does not exist. Fix to `flake.nix nix/*.nix`.
- `run` default references `--config config.toml … statement.csv bagels.db`
  from another project. Replace with a sane generic default (`cargo run --`).

### A6. `README.md` + `CHANGELOG.md`
- Neither file exists, yet all three of `Cargo.toml` (`readme =`, `include`),
  `cargo publish`, and CD's asset-copy step require them. Create template
  versions:
  - `README.md`: title, badges (CI, codecov, crates.io, license), one-line
    description, install, usage, license note.
  - `CHANGELOG.md`: git-cliff-compatible header so the first `changelog` run
    appends cleanly.

## Group B — Best-practice hardening

### B1. `deny.toml`
- Add a `[bans]` section: `multiple-versions = "warn"`, `wildcards = "deny"`
  (currently absent).

### B2. CI concurrency
- Add a `concurrency` block (group per workflow+ref, `cancel-in-progress: true`)
  so superseded PR runs cancel themselves.

## Group C — New tooling

All four get: a `just` recipe, inclusion in `just ci` + the CI workflow, and the
binary added to the Nix dev shell (`flake.nix`).

### C1. `actionlint` + `zizmor`
- Lint and security-audit the GitHub workflows themselves. On-theme: the repo's
  purpose is emitting well-formed, safe CI. `just actionlint` runs both; a CI
  job runs them on PRs touching `.github/`.

### C2. `taplo`
- TOML format + lint across all config TOML. Joins `just fmt` (write) and the
  `fmt --check` / CI format gate (`taplo fmt --check`, `taplo lint`).

### C3. `cargo-semver-checks`
- Fails CI on breaking public-API changes (the crate ships a `lib.rs` that
  downstream repos depend on). CI job on PRs; `just semver` locally.

### C4. `lefthook`
- Git-hook manager. `lefthook.yml` wires **pre-commit** (fmt, clippy, typos,
  taplo) and **pre-push** (`just ci` subset). Installed via `lefthook install`;
  binary added to the dev shell. Documented in README contributor section.

**Explicitly not added:** `release-plz`, `cargo-mutants`, `cargo-dist`,
`cargo-hack`, CodeQL, OpenSSF Scorecard.

## Group D — Substitution-token manifest

Catalogue (in this spec, for the future embedding step) every value the TUI must
substitute when it emits a generated repo. Concrete reference values in `()`.

| Token | Appears in | Reference value |
|-------|-----------|-----------------|
| project name | `Cargo.toml`, workflows, CD asset names, README, cliff.toml | `repo-builder` |
| author name + email | `Cargo.toml` `authors` | `wallago <henrotte.hugo@gmail.com>` |
| GitHub owner | `Cargo.toml` `repository`, `cliff.toml`, CODEOWNERS, FUNDING | `wallago` |
| description | `Cargo.toml` `description`, README | "Build your repo like a lazy boss" |
| keywords | `Cargo.toml` `keywords` | tui/repo/builder/tool/devops |
| categories | `Cargo.toml` `categories` | command-line-utilities, development-tools |
| license copyright holder | `LICENSE-MIT`, `LICENSE-APACHE` | wallago |
| funding handles | `.github/FUNDING.yml` | github/patreon/buymeacoffee = wallago |

No code change in this pass — this table is the contract the tool will honor.

## Testing / verification

- `cargo publish --dry-run --locked` succeeds (validates categories, readme,
  include, rust-version).
- `just ci` (extended with the new gates) passes locally where the toolchain is
  available; note any gate requiring network/secrets that cannot run locally.
- `cross build --locked --target x86_64-unknown-linux-musl` builds the release
  binary.
- `actionlint` + `zizmor` report clean on the rewritten workflows.
- `taplo fmt --check` and `taplo lint` pass on all TOML.
- `cargo msrv verify` passes against the declared `rust-version`.

## Risks / notes

- musl + `cross` for `aarch64` must build the TUI deps (`ratatui`, `termbg`) —
  expected clean (pure-Rust, no C deps), verified during implementation.
- `zizmor` may flag the existing GPG/token steps in CD; address findings rather
  than suppress unless a suppression is clearly justified and commented.
- Some CI gates (codecov upload, crates.io publish) need secrets and only run in
  CI, not locally — expected and documented, not a failure.
