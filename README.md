<p align="center">
    <img src="assets/logo.png" width="200">
    <br>
    <img src="assets/praline-text-dark.svg#gh-dark-mode-only" width="300">
    <img src="assets/praline-text-light.svg#gh-light-mode-only" width="300">
    <br>
    <br>
    <em>"Build your repo like a lazy boss."</em>
    <br>
    <br>
    <a href="https://crates.io/crates/praline">
        <img src="https://img.shields.io/crates/v/praline?style=flat&labelColor=1d1d1d&color=e05f78&logo=Rust&logoColor=white"></a>
    <a href="https://github.com/wallago/praline/releases">
        <img src="https://img.shields.io/github/v/release/wallago/praline?style=flat&labelColor=1d1d1d&color=e05f78&logo=GitHub&logoColor=white"></a>
    <br>
    <a href="https://github.com/wallago/praline/actions?query=workflow%3A%22CI%22">
        <img src="https://img.shields.io/github/actions/workflow/status/wallago/praline/ci.yml?style=flat&labelColor=1d1d1d&color=white&logo=GitHub%20Actions&logoColor=white"></a>
    <a href="https://github.com/wallago/praline/actions?query=workflow%3A%22CD%22">
        <img src="https://img.shields.io/github/actions/workflow/status/wallago/praline/cd.yml?style=flat&labelColor=1d1d1d&color=white&logo=GitHub%20Actions&logoColor=white&label=deploy"></a>
    <a href="./LICENSE-MIT">
        <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-e05f78?style=flat&labelColor=1d1d1d"></a>
</p>

🧱 **Praline** builds a well-made repo for your Rust project — scaffolding formatters, linters, tests, security, CI/CD, and governance — all within a user-friendly terminal user interface!

## Quickstart

Install `praline` with `cargo`:

```bash
cargo install praline
```

Then just run it in (or next to) your project:

```bash
praline
```

Pick the tools you want, and `praline` writes the configs for you. 💯

## Options

Options are grouped by subject. Tick them in the TUI, or pass each with `-o`
in headless mode:

```bash
praline --headless --name demo --owner you -D . -o flake -o just -o rust -o rust.common
```

### Environment

- `flake` — Nix dev shell and direnv
- `flake.package` — Nix package of the crate
- `flake.module.home` — Home Manager module
- `flake.module.nixos` — NixOS module
- `just` — `justfile` with the check, lint, fmt and ci recipes

### Rust

- `rust` — parent of the Rust options
- `rust.common` — base crate: `Cargo.toml`, entry point, args, config, error and prelude
- `rust.firmware` — `no_std` firmware crate, flashed with probe-rs
- `rust.server` / `rust.server.ws` — server module, and its websocket layer
- `rust.client_ws` — websocket client
- `rust.can` — CAN bus module
- `rust.notif` — notifier
- `rust.mcp` / `rust.mcp.http` — MCP server over stdio, or over Streamable HTTP
- `cargo.deny` — licences, bans, sources and advisories
- `cargo.audit` — RustSec advisory scan
- `cargo.udeps` — unused dependencies (cargo-machete)
- `cargo.codecov` — coverage with cargo-llvm-cov, uploaded to Codecov

### Web

- `web` — parent of the web stacks
- `web.react` — Vite + React + TypeScript app, with Vitest and Biome
- `web.pages` — GitHub Pages deploy of the built site
- `web.pwa` — installable PWA: manifest, icons and an offline service worker

### Repo

- `editorconfig` — `.editorconfig`, checked by editorconfig-checker
- `typos` — spell-check
- `lychee` — link check
- `readme` — `README.md`
- `git` — `.gitignore`, licences, commit format (committed) and changelog (git-cliff)
- `github` — `.github/` templates and dependabot
- `github.ci` — CI workflow running `just ci` recipe by recipe
- `claude` — `CLAUDE.md` and `.claude/`

## Contributing

Contributions are welcome! Feel free to open an issue or a pull request.

## License

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

🦀 ノ( º \_ º ノ) - respect crables!

## Copyright

Copyright © 2026, [wallago](mailto:henrotte.hugo@gmail.com)
