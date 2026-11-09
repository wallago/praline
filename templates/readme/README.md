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
<!-- {if:git} -->

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

<!-- {endif:git} -->
