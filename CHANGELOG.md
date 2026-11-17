# Changelog

## 0.3.0 - 2026-11-17

### 🚀 Features

- [**breaking**] Rebuild praline on the option engine by @wallago in [#30](https://github.com/wallago/praline/pull/30)
- *(templates/web)* Add web, web.react and web.pages options by @claude
- *(templates)* Add dev tooling and CI options by @claude
- *(templates/rust)* Add per-option Cargo keywords and categories by @claude
- *(templates/rust.common/Cargo.toml)* Add slots by @wallago
- *(mcp.http)* Add config by @wallago
- *(rust)* Set config as mandatory by @wallago
- *(opt)* Add mcp and mcp.http by @wallago
- *(args)* Add params to build headless repo by @wallago
- Add conflicts and deps for opts by @wallago
- Add config file support for rust by @wallago
- *(ui/dashboard)* Enhance ui by @wallago
- Improve templates by @wallago
- *(client_ws)* Add stuffs by @wallago
- Continue refactor by @wallago
- *(ui/picker)* Init picker dest by @wallago
- Set form behaviors by @wallago
- Add form by @wallago
- *(ui)* Add form for app info by @wallago
- *(app)* Generate tree from ui by @wallago
- *(ui)* Link with app options by @wallago
- *(ui)* Improve movement by @wallago
- *(ui)* Improve sidebar by @wallago
- *(ui)* Sprout-style redesign on placeholder data by @wallago
- *(engine)* Redesign how it work by @wallago
- *(help)* Adding test by @wallago
- *(gitignore)* Adding dev extension by @wallago
- *(claude)* Set manual mode as default one by @wallago
- *(flake)* Add banner by @wallago
- *(ui)* Add following cursor while editing by @wallago
- *(ui)* Add blinking cursor by @wallago
- *(flake/package)* Adding mit license by @wallago
- *(flake)* Define nixos module by @wallago
- *(templates/justfile)* Enhance justfile contents by @wallago
- *(templates/git/workflows)* Add claude review by @wallago
- *(workflows)* Adding claude review by @wallago
- *(help)* Add filtering condition by @wallago

### 🐛 Bug Fixes

- *(ui)* Correct typos in the dashboard and key hints by @claude
- *(flake)* Expose the home-manager module by @claude
- *(rust)* Lower rust-version to 1.100 by @claude
- *(ui/details)* Avoid map().unwrap_or_default() flagged by nightly clippy by @claude in [#25](https://github.com/wallago/praline/pull/25)
- *(templates/rust.common)* Make the generated crate packageable by @claude
- *(templates)* Make can, client_ws and notif build, lint and test by @claude
- *(templates/rust.mcp)* Lint warning removed by @wallago
- *(templates)* Make generated repos build, lint and test cleanly by @claude
- *(app)* Make requirement check hardened by @wallago
- *(tests)* Now pass by @wallago
- *(templates)* Update some rust opts by @wallago
- *(claude)* Wrong format for clippy deps by @wallago
- *(flake)* Adding missing lychee deps by @wallago
- *(nix)* Replace correct name in modele by @wallago
- *(codecov)* Adding good args and deps by @wallago
- *(flake/hm-module)* Replace services to programs by @wallago in [#11](https://github.com/wallago/praline/pull/11)
- *(templates/flake)* Change end filter name by @wallago
- *(filter)* Solve condition issue by @wallago
- *(templates/flake)* Add filter by @wallago
- *(templates/flake)* Filter rust part by @wallago

### 🚜 Refactor

- *(args)* Remove color ascent cause useless by @wallago
- *(opts)* Make it more clean by @wallago
- *(clippy)* Make it more clear and add tests for engine by @wallago
- All project and remove unused files by @wallago
- *(committed)* Enhance just command by @wallago
- *(flake/module)* Switch from nixos to hm by @wallago
- *(name)* Set correct name by @wallago
- *(flake)* Lint desc by @wallago
- *(claude/settings)* Change how filter are set by @wallago
- *(just)* Change how just is handle by @wallago
- *(templates/claude/rules)* Add filter into rules by @wallago
- *(templates/claude)* Rename flake filter to nix by @wallago
- *(templates/git)* Adding filter by @wallago
- *(nix)* Rename flake to nix by @wallago
- *(claude)* Reduce CLAUDE.md to rule imports by @wallago
- *(templates/claude)* Replace agents and plugins with rule files by @wallago

### 🧪 Testing

- *(rust.common)* Cover args, config and the binary in the template by @claude in [#23](https://github.com/wallago/praline/pull/23)
- *(e2e)* Add rust tests by @wallago

### ⚙️ Miscellaneous Tasks

- *(repo)* Build praline's tooling from its own templates by @claude in [#29](https://github.com/wallago/praline/pull/29)
- *(just)* Add a gh-based release tool by @claude in [#28](https://github.com/wallago/praline/pull/28)
- *(just)* Move template recipes out of rust.just by @claude
- *(rust)* Bump version by @wallago in [#21](https://github.com/wallago/praline/pull/21)
- *(editorconfig)* Refactor it by @wallago
- *(rust)* Bump it by @wallago
- *(claude/rules)* Make execption for repo mutation by @wallago
- *(claude)* Save conv by @wallago
- Make buildable rust repo by @wallago
- *(claude)* Change model and effort by @wallago
- *(flake)* Bump lock by @wallago
- *(claude)* Add rule set, permissions and diff-first style by @wallago


## New Contributors
* @claude made their first contribution in [#29](https://github.com/wallago/praline/pull/29)

**Full Changelog**: https://github.com/wallago/praline/compare/v0.2.0...v0.3.0

## 0.2.0 - 2026-08-16

### 🚀 Features

- *(flake/package)* Adding mit license by @wallago
- *(flake)* Define nixos module by @wallago
- *(flake)* Adding missing tool from deps by @wallago
- *(templates/flake)* Adding missing tools by @wallago
- *(cd)* Adding a stop publish for rust crate by @wallago
- *(preset)* Adding it by @wallago
- Wip by @wallago
- Local ci pass by @wallago
- *(rust)* Generate working project by @wallago
- No more clippy warn by @wallago
- Add docs by @wallago
- Ok we have something by @wallago
- Improve by @wallago
- Improving by @wallago
- Improving rust by @wallago
- Huge advance by @wallago
- Allow multi file and dir by @wallago
- Add just by @wallago
- Improve by @wallago
- Improve by @wallago
- Improve by @wallago
- Improving by @wallago
- Update by @wallago
- Improving by @wallago
- Improve by @wallago
- Improving by @wallago
- Update by @wallago
- Make repo enhanced by @wallago
- Init by @wallago

### 🐛 Bug Fixes

- *(flake/hm-module)* Replace services to programs by @wallago
- *(ci/committed)* Change how merging is handle by @wallago
- *(ci)* Change args for local light ci test  by @wallago
- *(cargo)* Bump lock (ci failed due to this) by @wallago
- *(cd)* Cliff now add prerelease tag by @wallago
- *(doc)* Missing right to access a struct by @wallago
- *(flake)* Wrong name for nix pckg by @wallago
- *(just/ci)* Adding missing ci tests by @wallago
- *(clippy)* Solve all warn by @wallago

### 🚜 Refactor

- *(flake/module)* Switch from nixos to hm by @wallago
- *(name)* Set correct name by @wallago
- *(flake)* Lint desc by @wallago
- *(committed)* Change PR/merging handling by @wallago
- *(cliff)* Remove change flag for beta/alpha by @wallago
- *(ci)* Try a new one by @wallago

### ⚙️ Miscellaneous Tasks

- *(templates)* Sync action versions with root by @wallago in [#10](https://github.com/wallago/praline/pull/10)
- *(release)* V0.2.0-alpha.7 by @wallago in [#1](https://github.com/wallago/praline/pull/1)
- *(release)* V0.2.0-alpha.6 by @wallago
- *(release)* V0.2.0-alpha.5 by @wallago
- *(release)* V0.2.0-alpha.4 by @wallago
- *(release)* V0.2.0-alpha.3 by @wallago
- *(release)* V0.2.0-alpha.2 by @wallago
- README by @wallago

### Wip

- *(cd)* Try to make it universal by @wallago


## New Contributors
* @dependabot[bot] made their first contribution in [#8](https://github.com/wallago/praline/pull/8)

<!-- generated by git-cliff -->
