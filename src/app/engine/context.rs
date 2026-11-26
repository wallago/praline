use std::collections::HashSet;

use strum::IntoEnumIterator;

use crate::app::engine::release;
use crate::app::engine::slot::{Slots, collect_slots};
use crate::app::opt::{Emit, Scope};
use crate::app::{App, opt::OptId};
use crate::error::EngineError::UnknownOption;
use crate::prelude::*;

/// Slots each app keeps to itself once the repo holds several: every app has
/// its own `justfile`, and the root one calls into it as a module.
pub(super) const APP_LOCAL_SLOTS: &[&str] = &["just.imports", "just.ci", "github.ci.recipes"];

/// Name `{if:}` takes for "this render is part of a Cargo workspace".
const WORKSPACE: &str = "workspace";
/// Name `{if:}` takes for "these are the root files of a repo with several
/// apps".
const APPS: &str = "apps";
/// Name `{if:}` takes for "the flake exports a home-manager or NixOS module".
const FLAKE_MODULE: &str = "flake.module";

/// What every template can read. Built once per render, read-only after.
#[derive(Debug)]
pub(crate) struct Ctx {
    /// `{key}` placeholders and their values.
    pub vars: Vec<(&'static str, String)>,
    /// Options taking part: checked, with their parents and `requires` on.
    pub active: HashSet<OptId>,
    /// Contributions of the active options, in `OptId` order.
    pub slots: Slots,
    /// Whether `{if:workspace}` is on: an app that is a workspace member, or
    /// the root of a repo that has one.
    pub workspace: bool,
    /// Whether `{if:apps}` is on: the root files of a repo with several apps.
    pub apps: bool,
}

/// The options taking part in app `at`: checked there (repo-wide ones count
/// for every app) and allowed with this many apps.
pub(super) fn active_in(app: &App, at: usize) -> HashSet<OptId> {
    let checked: HashSet<OptId> = app.apps[at]
        .options
        .iter()
        .filter(|opt| opt.checked)
        .map(|opt| opt.id)
        .collect();
    let multi = app.is_multi();
    OptId::iter()
        .filter(|&id| id.is_active(&checked))
        .filter(|&id| !(multi && id.def().single_app_only))
        .collect()
}

/// Whether app `at` is a member of the repo's Cargo workspace: several apps,
/// and this one is a hosted Rust crate. A firmware builds for its own target,
/// so it stays out.
pub(super) fn is_member(app: &App, at: usize) -> bool {
    app.is_multi() && active_in(app, at).contains(&OptId::RustCommon)
}

impl Ctx {
    /// Context for app `at`'s own files. With one app this is the whole repo,
    /// exactly as before apps existed.
    pub(crate) fn for_app(app: &App, at: usize) -> Self {
        let active = active_in(app, at);
        let multi = app.is_multi();
        let mut slots = Slots::new();
        for id in OptId::iter().filter(|id| active.contains(id)) {
            if let Emit::Dir(dir) = id.def().emit {
                // The repo-wide recipes live in the root `justfile`, not here.
                let repo_wide = multi && id.def().scope == Scope::Repo;
                collect_slots(dir, &mut slots, |name| {
                    !(repo_wide && APP_LOCAL_SLOTS.contains(&name))
                });
            }
        }
        // With several apps, the release tool lives at the root.
        if !multi {
            release::fill(app, &mut slots);
        }
        let name = if multi {
            app.apps[at].name.clone()
        } else {
            app.name.clone()
        };
        Self {
            vars: vars(app, &name),
            active,
            slots,
            workspace: is_member(app, at),
            apps: false,
        }
    }

    /// Context for the files at the repo root, shared by every app. Only
    /// built when there are several apps: with one, [`Self::for_app`] covers
    /// the whole repo.
    pub(crate) fn for_root(app: &App) -> Self {
        let per_app: Vec<HashSet<OptId>> =
            (0..app.apps.len()).map(|at| active_in(app, at)).collect();
        let active: HashSet<OptId> = per_app.iter().flatten().copied().collect();
        let mut slots = Slots::new();
        for id in OptId::iter().filter(|id| active.contains(id)) {
            let Emit::Dir(dir) = id.def().emit else {
                continue;
            };
            if id.def().scope == Scope::Repo {
                collect_slots(dir, &mut slots, |_| true);
                continue;
            }
            // Each app adds its tools to the shared flake, CLAUDE.md and the
            // rest; two Rust apps add the same lines, which go in once.
            for app_active in &per_app {
                if !app_active.contains(&id) {
                    continue;
                }
                let mut own = Slots::new();
                collect_slots(dir, &mut own, |name| !APP_LOCAL_SLOTS.contains(&name));
                for (name, blocks) in own {
                    let entry = slots.entry(name).or_default();
                    for block in blocks {
                        if !entry.contains(&block) {
                            entry.push(block);
                        }
                    }
                }
            }
        }
        // The root `justfile` and CI run each app's own `ci` recipe.
        for (at, unit) in app.apps.iter().enumerate() {
            let prefix = app.prefix(at);
            let push = |slots: &mut Slots, name: &str, block: String| {
                slots.entry(name.to_string()).or_default().push(block);
            };
            push(
                &mut slots,
                "just.imports",
                format!("mod {} '{}'", unit.name, prefix.display()),
            );
            push(&mut slots, "just.ci", format!("@just {}::ci", unit.name));
            push(
                &mut slots,
                "github.ci.recipes",
                format!("- {}::ci", unit.name),
            );
            push(
                &mut slots,
                "workspace.apps",
                format!(
                    "- `{}/`: its recipes run as `{}::<recipe>`.",
                    prefix.display(),
                    unit.name
                ),
            );
        }
        release::fill(app, &mut slots);
        Self {
            vars: vars(app, &app.name),
            active,
            slots,
            workspace: (0..app.apps.len()).any(|at| is_member(app, at)),
            apps: true,
        }
    }

    /// Answers `{if:<name>}`. An unknown name is an error, not a silent "off".
    pub(crate) fn is_on(&self, name: &str) -> Result<bool> {
        if name == WORKSPACE {
            return Ok(self.workspace);
        }
        if name == APPS {
            return Ok(self.apps);
        }
        if name == FLAKE_MODULE {
            return Ok(self.active.contains(&OptId::FlakeModuleHome)
                || self.active.contains(&OptId::FlakeModuleNixos));
        }
        let id: OptId = name
            .parse()
            .map_err(|_| <EngineError as Into<Error>>::into(UnknownOption(name.to_string())))?;
        Ok(self.active.contains(&id))
    }
}

/// The `{key}` placeholders: `name` is the app's (its crate or package), the
/// rest are the repo's.
fn vars(app: &App, name: &str) -> Vec<(&'static str, String)> {
    vec![
        ("ident", ident(name)),
        ("name", name.to_string()),
        ("repo", app.name.clone()),
        ("desc", app.desc.clone()),
        ("owner", app.owner.clone()),
    ]
}

/// Turns `name` into a Rust identifier: lowercase ASCII alphanumerics, `_` for
/// anything else, and a leading `_` if it would start with a digit.
#[must_use]
pub(super) fn ident(name: &str) -> String {
    let mut out: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    if out.starts_with(|c: char| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_is_inactive_while_its_parent_is_off() {
        assert!(!OptId::RustFirmware.is_active(&HashSet::from([OptId::RustFirmware])));
        assert!(OptId::RustFirmware.is_active(&HashSet::from([OptId::Rust, OptId::RustFirmware])));
    }
}
