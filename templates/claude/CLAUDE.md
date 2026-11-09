# {name}

{desc}

## Layout

<!-- {slot:claude.layout} -->

<!-- TODO: what this project is for, and where a typical change lands. Claude
     reads this file before every task, so keep it true. -->

## Rules

Standalone rules live in `.claude/rules/` and are imported here. Only files
reachable from `CLAUDE.md` get loaded, so a new rule needs a line below.

<!-- {if:flake} -->
@.claude/rules/nix.md
<!-- {endif:flake} -->
<!-- {if:just} -->

## Commands

`just` is the entry point, not the raw toolchain; `just --list` shows every
recipe.

- `just ci`: the gate. It runs every check CI runs, recipe by recipe, so a
  green `just ci` is a green CI. Run it before calling a change done.
<!-- {if:rust.common} -->
- `just check`, `just test`, `just lint`, `just fmt`: the fast loop.
<!-- {endif:rust.common} -->
<!-- {if:web.react} -->
- `just check`, `just test`, `just lint`, `just fmt`: the fast loop.
  `just dev` serves the site; it doesn't exit, so ask the user to run it.
<!-- {endif:web.react} -->
<!-- {endif:just} -->

## Constraints

<!-- {slot:claude.constraints} -->

## Verifying a change

<!-- {if:just} -->
`just ci` green is the bar. Say which recipes ran when reporting a change.
<!-- {endif:just} -->

<!-- TODO: what the gate cannot catch here (hardware, a UI, a remote service),
     and how it gets checked instead. -->
