use crate::app::{
    App,
    engine::{context::active_in, slot::Slots},
    opt::OptId,
};

/// Slot of `release.just` holding the commands that set every app's version
/// to `$version`.
const BUMP: &str = "release.bump";
/// Slot of `cd.yml` listing the binaries it packs, one quoted name a line.
const BINS: &str = "release.bins";

/// Fills `github.release`'s slots from the apps: they hold paths and names,
/// which only the list of apps knows.
///
/// Every Rust app is a crate of the repo's workspace (one crate at the root,
/// or the members of the root `Cargo.toml`), so one `cargo set-version`
/// covers them all; each web app gets its own `package.json` line. A firmware
/// has no manifest yet, so it has no version to bump and no binary to ship.
pub(super) fn fill(app: &App, slots: &mut Slots) {
    let multi = app.is_multi();
    let mut bump = Vec::new();
    let mut bins = Vec::new();
    for (at, unit) in app.apps.iter().enumerate() {
        let active = active_in(app, at);
        if active.contains(&OptId::RustCommon) {
            let name = if multi { &unit.name } else { &app.name };
            bins.push(format!("\"{name}\""));
        }
        if active.contains(&OptId::WebReact) {
            bump.push(if multi {
                format!(
                    "(cd {} && npm pkg set version=\"$version\")",
                    app.prefix(at).display()
                )
            } else {
                "npm pkg set version=\"$version\"".to_string()
            });
        }
    }
    if !bins.is_empty() {
        bump.insert(0, "cargo set-version --workspace \"$version\"".to_string());
    }
    if !bump.is_empty() {
        bump.insert(0, "echo \"▶ bump the version to $version\"".to_string());
        slots.insert(BUMP.to_string(), vec![bump.join("\n") + "\n"]);
    }
    if !bins.is_empty() {
        slots.insert(BINS.to_string(), vec![bins.join("\n") + "\n"]);
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::args::Args;

    /// The slots filled for these `--app` specs (none: one flat app checking
    /// `opts`).
    fn filled(apps: &[&str], opts: &[&str]) -> Slots {
        let app = App::new(Args {
            name: Some("demo".into()),
            owner: Some("demo".into()),
            desc: Some("Demo.".into()),
            app: apps.iter().map(|&a| a.into()).collect(),
            opt: opts.iter().map(|&o| o.into()).collect(),
            ..Args::default()
        })
        .unwrap();
        let mut slots = Slots::new();
        fill(&app, &mut slots);
        slots
    }

    /// What was filled into `name`, if anything.
    fn block(slots: &Slots, name: &str) -> Option<String> {
        slots.get(name).map(|blocks| blocks.concat())
    }

    #[test]
    fn a_crate_bumps_with_cargo_and_ships_its_binary() {
        let slots = filled(&[], &["rust", "rust.common"]);
        assert_eq!(
            block(&slots, BUMP).as_deref(),
            Some(
                "echo \"▶ bump the version to $version\"\n\
                 cargo set-version --workspace \"$version\"\n"
            )
        );
        assert_eq!(block(&slots, BINS).as_deref(), Some("\"demo\"\n"));
    }

    #[test]
    fn a_site_bumps_its_package_json_and_ships_nothing() {
        let slots = filled(&[], &["web", "web.react"]);
        assert_eq!(
            block(&slots, BUMP).as_deref(),
            Some("echo \"▶ bump the version to $version\"\nnpm pkg set version=\"$version\"\n")
        );
        assert_eq!(block(&slots, BINS).as_deref(), None);
    }

    #[test]
    fn a_repo_with_no_stack_has_nothing_to_bump() {
        let slots = filled(&[], &["just"]);
        assert!(slots.is_empty(), "{slots:?}");
    }

    #[test]
    fn several_apps_bump_together_and_ship_each_binary() {
        let slots = filled(
            &["api:rust.server", "worker:rust.notif", "site:web.react"],
            &[],
        );
        assert_eq!(
            block(&slots, BUMP).as_deref(),
            Some(
                "echo \"▶ bump the version to $version\"\n\
                 cargo set-version --workspace \"$version\"\n\
                 (cd apps/site && npm pkg set version=\"$version\")\n"
            )
        );
        assert_eq!(
            block(&slots, BINS).as_deref(),
            Some("\"api\"\n\"worker\"\n")
        );
    }

    #[test]
    fn a_firmware_has_nothing_to_bump_or_ship() {
        let slots = filled(&["host:rust.server", "fw:rust.firmware"], &[]);
        assert_eq!(block(&slots, BINS).as_deref(), Some("\"host\"\n"));
    }
}
