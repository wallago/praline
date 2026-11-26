# {name}

{desc}

<!-- {if:flake} -->

## Development environment

Every tool comes from the Nix flake. Enter the dev shell with:

```sh
nix develop
```

`.envrc` is `use flake`, so with direnv, `direnv allow` once and the shell
loads on `cd`.

<!-- {endif:flake} -->
<!-- {if:just} -->

## Tasks

`just` is the entry point:

```sh
just          # list every recipe
just ci       # every check CI runs; run it before pushing
```

<!-- {endif:just} -->
<!-- {if:apps} -->

## Apps

<!-- {slot:workspace.apps} -->

`just --list <app>` shows what an app can do.

<!-- {endif:apps} -->
<!-- {ifnot:apps} -->
<!-- {if:rust.common} -->

## Usage

```sh
cargo run -- --help
```

Settings are read from a TOML file; `config.example.toml` lists them.

<!-- {endif:rust.common} -->
<!-- {if:web.react} -->

## Usage

```sh
just dev      # serve the site with hot reload
just build    # build it into dist/
```

<!-- {if:web.pages} -->
Every push to `main` deploys `dist/` to GitHub Pages. Turn Pages on once in
the repo settings, with GitHub Actions as the source.

<!-- {endif:web.pages} -->
<!-- {endif:web.react} -->
<!-- {endif:apps} -->
<!-- {if:github.release} -->

## Releasing

A release takes two commands, with a PR between them, from an up-to-date
`main` (both need `gh auth login`):

```sh
just release 1.2.0          # opens the release PR: version bump + CHANGELOG.md
just release-publish 1.2.0  # once it is merged: tags main, creates the GitHub release
```

<!-- {if:rust.common} -->
The tag starts `cd.yml`, which attaches the binaries to the release.
<!-- {if:github.release.crates} -->
It then publishes the crates on crates.io, except for a pre-release
(`1.2.0-rc.1`); it needs a `CARGO_REGISTRY_TOKEN` repository secret.
<!-- {endif:github.release.crates} -->

<!-- {endif:rust.common} -->
<!-- {endif:github.release} -->
<!-- {if:git} -->

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

<!-- {endif:git} -->
