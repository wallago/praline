---
name: adopt
description: Align an existing repo with the tooling Praline generates (justfile, Nix flake, CI, lints, checks, CLAUDE.md). Use when asked to make a repo look like a Praline one, adopt Praline, or bring a repo up to Praline's standard.
---

# Adopt Praline in an existing repo

Praline generates a repo whose tooling is known to work: it compiles, its
tests pass and its `just ci` gate is green. This skill renders that repo for
the options that match the current one, compares the two, and brings the
current repo's tooling in line. The repo's own code stays the repo's.

Work in the current repo. Never write into it before the user has seen the
plan (step 4). If the repo has its own rules (`CLAUDE.md`, `.claude/rules/`),
they win over this skill: a "propose before writing" or "no commits" rule
still holds at every step.

## 1. Get Praline

Run `praline --version`. If it is missing, use
`nix run github:wallago/praline --` in its place, or ask the user to install
it (`cargo install praline`). Read `praline --help`: its `--opt` list is the
set of options this version knows. Don't use an option that isn't in it.

## 2. Pick the options

Read the repo and map what it has to Praline options. Tooling the repo lacks
is not a reason to leave an option out: adding it is the point.

| The repo has                                       | Options                          |
| -------------------------------------------------- | -------------------------------- |
| `Cargo.toml` (a binary or library crate)           | `rust`, `rust.common`            |
| `#![no_std]`, probe-rs, `memory.x`                 | `rust.firmware` instead          |
| axum / an HTTP server                              | `rust.server` (+ `rust.server.ws`) |
| rmcp / an MCP server                               | `rust.mcp` or `rust.mcp.http`    |
| a websocket client, CAN bus, desktop notifications | `rust.client_ws`, `rust.can`, `rust.notif` |
| `package.json` with Vite and React                 | `web`, `web.react` (+ `web.pages`, `web.pwa`) |
| any Rust crate                                     | `cargo.deny`, `cargo.audit`, `cargo.udeps` |
| any repo                                           | `just`, `flake`, `editorconfig`, `typos`, `lychee`, `git`, `github`, `github.ci`, `claude` |

Leave out `readme` when the repo already has a `README.md`.

Several apps in one repo (a Cargo workspace, an `apps/` folder, a server and
a client side by side) become one `--app <name>:<opts>` each. App options
(`rust.*`, `web.*`) go in `--app`; repo-wide ones (`just`, `flake`,
`cargo.*`, `github.*`, the checks) go in `-o`. Praline says which is which
if a call mixes them up.

Take the name from `Cargo.toml` or `package.json`, the owner from the
`origin` remote, the description from the README's first line.

Show the user the option list, with one line on each guess that isn't
obvious, and the exact command of step 3. Go on once they agree.

## 3. Render the reference

Into a fresh temp folder, never inside the repo:

```sh
ref=$(mktemp -d)
praline --headless --name <name> --owner <owner> --desc "<desc>" -D "$ref" \
  -o just -o flake ... [--app <name>:<opts> ...]
```

The reference is `$ref/<name>`. Praline refuses a target that exists, so use
a new temp folder for each run.

## 4. Compare and plan

```sh
diff -rq --exclude=target --exclude=node_modules --exclude=.git \
  --exclude=.direnv --exclude=dist --exclude='*.lock' \
  --exclude=pnpm-lock.yaml "$ref/<name>" .
```

Sort every path into one group:

- **Missing**: tooling only the reference has (`deny.toml`, `typos.toml`,
  `.editorconfig`, `committed.toml`, `cliff.toml`, `rustfmt.toml`,
  `clippy.toml`, `.github/workflows/ci.yml`, `just/*.just`...). Candidate to
  add as is, after reading it: drop the parts that name things this repo
  doesn't have.
- **Differs**: files both have (`justfile`, `flake.nix`, `Cargo.toml`,
  `.gitignore`, `CLAUDE.md`, CI). Read both and merge: keep the repo's
  recipes, dependencies and settings, add what the reference has that the
  repo lacks (a recipe, a lint table, a CI job, a dev-shell tool).
- **Source**: `src/`, `tests/`, `apps/*/src/`, anything the template only
  scaffolds. Never copied. Note a structural gap only when it matters (the
  reference has `error.rs` with `thiserror` and the repo panics on errors).
- **Repo only**: files the reference doesn't have. Leave them.

Write the plan: one line per file, its group, and what changes. Put the
changes that bring new checks (lints, `typos`, `cargo deny`) last and say
they may surface failures in existing code. Stop and wait for the user's go.
They may cut the list.

## 5. Apply

In the order of the plan, missing files first, then merges. Follow the
repo's rules on how changes get shown and written.

- A dependency or Nix input added by a merge changes `Cargo.lock` or
  `flake.lock` as a side effect; that goes in its own commit.
- A placeholder in a reference file (`<!-- TODO -->`, the example config)
  gets filled from the repo or left out, never copied as is.

Then run the repo's gate (`just ci` once there is a justfile). Fix what the
new checks find in the code, or, where a fix is not small, list it for the
user instead. Two tries on the same failure, then stop and report.

## 6. Report

What was added, what was merged, what was left out and why, the gate's
result, and the follow-ups (failures the new checks found, source gaps).
