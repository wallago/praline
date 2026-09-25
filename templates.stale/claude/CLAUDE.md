# {name}

{desc}

<!-- TODO — three or four lines on shape: the entry point, the two or three
     modules that matter, and where a typical change lands. Claude reads this
     before every task, so a wrong description costs more than an empty one.
     Delete this comment when written. -->

## Rules

Standalone rules live in `.claude/rules/` and are imported here — only files
reachable from `CLAUDE.md` get loaded, so a new rule needs a line below.

@.claude/rules/propose-before-writing.md
@.claude/rules/no-repo-mutation.md
@.claude/rules/no-hanging-commands.md
@.claude/rules/prevent-looping-fail.md

<!-- {if:rust} -->

@.claude/rules/rust.md

<!-- {endif:rust} -->
<!-- {if:nix} -->

@.claude/rules/nix.md

<!-- {endif:nix} -->
<!-- {if:cliff} -->

@.claude/rules/release.md

<!-- {endif:cliff} -->

## Commands

<!-- {if:justfile} -->

`just` is the entry point, not the raw toolchain. `just --list` for the rest.

- `just check` — fast type-check, no binary
- `just test` — test suite
- `just fmt` — format sources in place
- `just lint` — lint with warnings denied
- `just ci` — the full local gate; run this before pushing
<!-- {endif:justfile} -->

<!-- {ifnot:justfile} -->
<!-- {if:rust} -->

- `cargo check --all-targets`
- `cargo nextest run`
- `cargo fmt`
- `cargo clippy --all-targets -- -D warnings`

<!-- {endif:rust} -->
<!-- {endif:justfile} -->

<!-- TODO — anything above that is wrong here, and any command that exists only
     in this repo: seeding, fixtures, a dev server, a hardware target. -->

## Constraints

<!-- {if:typos} -->

- `typos` runs over the source. Add a real term to `typos.toml` rather than
  rewording around it.

<!-- {endif:typos} -->

<!-- {if:committed} -->

- Commit messages must be Conventional Commits; `committed` checks them in CI.

<!-- {endif:committed} -->

<!-- {if:lychee} -->

- Links in Markdown are checked in CI. A placeholder URL fails the build.

<!-- {endif:lychee} -->

## Verifying a change

<!-- {if:justfile} -->

`just ci-light` is the gate. Green means green.

<!-- {endif:justfile} -->

<!-- TODO — what the gate *cannot* catch here, and how it gets checked instead.
     Be specific. "The binary is a full-screen TUI so it can't be driven
     headlessly — UI changes get verified by reading, by `cargo check`, and by
     asking me to run it" is the useful kind. Never claim a change works
     without saying how it was checked. -->
