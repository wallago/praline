# {name}

> {desc}

## Development

This repo is scaffolded by [praline](https://github.com/wallago/praline).
Its toolchain is provided by the Nix dev shell — enter it with `nix develop`
(or automatically via `direnv`).

### Common commands

| Command | What it does |
| --- | --- |
| `just` | List every available task |
| `just ci` | Run the full CI mirror locally — do this before pushing |
| `cargo nextest run` | Run the test suite |
| `cargo clippy --all-targets` | Lint |
| `cargo fmt` | Format Rust |
| `nix flake check` | Run the Nix checks |

## Conventions

- **VCS:** jujutsu (`jj`), colocated with git.
- **Formatting is enforced** (rustfmt, taplo, nixfmt) — run the `just` recipes,
  don't hand-format.
- **Commits** follow the `committed` config; the changelog is generated with
  `git cliff`.
- Keep changes small and run `just ci` before handing work back.

Maintainer: {owner}
