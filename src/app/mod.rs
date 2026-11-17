use std::collections::HashSet;
use std::env::current_dir;
use std::path::PathBuf;

use strum::IntoEnumIterator;

use crate::app::engine::{Tree, write_tree};
use crate::app::opt::{OptId, Scope};
use crate::error::EngineError::{NotGenerated, TargetExists};
use crate::prelude::*;

/// Optional tools.
pub(crate) mod opt;

/// Optional category.
mod category;

/// Free-text fields and the rules their values follow.
mod field;

/// Renders the active options' templates into a file tree.
mod engine;

/// App types the rest of the crate uses often.
pub(crate) mod prelude {
    pub(crate) use super::field::Field;
    pub(crate) use super::opt::OptId;
}

/// Folder the apps sit in once the repo holds more than one.
pub(crate) const APPS_DIR: &str = "apps";

/// Repo builder.
#[derive(Debug)]
pub(crate) struct App {
    /// Repo name.
    pub(crate) name: String,
    /// Repo description.
    pub(crate) desc: String,
    /// Repo owner.
    pub(crate) owner: String,
    /// Repo destination.
    pub(crate) dest: PathBuf,
    /// The apps of the repo; never empty. One app renders at the root, as a
    /// plain single-crate repo; two or more each render in `apps/<name>/`.
    pub(crate) apps: Vec<AppUnit>,
    /// Last render, shown in the preview and written on export.
    pub(crate) staged: Option<Tree>,
}

/// One app of the repo: a crate, a firmware or a site.
#[derive(Debug, Clone)]
pub(crate) struct AppUnit {
    /// Its folder under `apps/` and its package name, once there are several.
    pub(crate) name: String,
    /// Every option. The repo-wide ones ([`Scope::Repo`]) are the same in
    /// every app: changing one in an app changes it in all of them.
    pub(crate) options: Vec<Opt>,
}

/// Selectable repo option.
#[derive(Debug, Clone)]
pub(crate) struct Opt {
    /// Which option: the key into the table.
    pub(crate) id: OptId,
    /// Ticked by the user (it can still be inactive, see `Ctx::active`).
    pub(crate) checked: bool,
}

/// What an option list is about: the repo-wide options, or one app's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum View {
    /// Options shared by the whole repo.
    #[default]
    Repo,
    /// Options of the app at this index.
    App(usize),
}

/// One row of an option list: the option, how deep it sits, and whether
/// a checked option of the same app rules it out.
#[derive(Debug, Clone)]
pub(crate) struct Row {
    /// The option.
    pub(crate) id: OptId,
    /// Ticked.
    pub(crate) checked: bool,
    /// Ruled out by a checked option it conflicts with.
    pub(crate) conflicted: bool,
    /// Ticked but not taking part: its parent or a requirement is off.
    pub(crate) idle: bool,
    /// Indent level in the tree.
    pub(crate) depth: usize,
}

/// Every option, in tree order, checked as the TUI starts or all unchecked.
fn fresh_options(defaults: bool) -> Vec<Opt> {
    let mut list = Vec::new();
    OptId::sorted_list(&mut list, None);
    list.into_iter()
        .map(|id| Opt {
            id,
            checked: defaults && id.starts_checked(),
        })
        .collect()
}

/// The ids checked in `options`.
fn checked_in(options: &[Opt]) -> HashSet<OptId> {
    options.iter().filter(|o| o.checked).map(|o| o.id).collect()
}

/// Checks or toggles `id` in `options`; once checked, also checks its parent
/// and what it requires. With `repo_only`, app options are left alone.
fn nested(options: &mut [Opt], id: OptId, check: Option<bool>, repo_only: bool) {
    if repo_only && id.def().scope != Scope::Repo {
        return;
    }
    let Some(opt) = options.iter_mut().find(|o| o.id == id) else {
        return;
    };
    opt.checked = check.unwrap_or(!opt.checked);
    if !opt.checked {
        return;
    }
    let def = id.def();
    for dep in def.parent.into_iter().chain(def.requires.iter().copied()) {
        nested(options, dep, Some(true), repo_only);
    }
}

/// Checks every default child of `parent` in `options`, and theirs.
fn check_defaults(options: &mut [Opt], parent: OptId, repo_only: bool) {
    for child in OptId::iter().filter(|c| c.def().parent == Some(parent) && c.def().default) {
        nested(options, child, Some(true), repo_only);
        check_defaults(options, child, repo_only);
    }
}

/// The first two checked options of `options` that can't take part together.
fn conflict_in(options: &[Opt]) -> Option<(OptId, OptId)> {
    let checked: Vec<OptId> = options.iter().filter(|o| o.checked).map(|o| o.id).collect();
    checked.iter().enumerate().find_map(|(at, &a)| {
        checked[at + 1..]
            .iter()
            .find(|&&b| a.conflicts_with(b))
            .map(|&b| (a, b))
    })
}

impl App {
    /// Set a new app.
    ///
    /// # Errors
    ///
    /// Returns an error if an `--app` is malformed, names an option that
    /// doesn't exist, or puts an option of the wrong scope in the wrong place.
    pub(crate) fn new(args: Args) -> Result<Self> {
        let name = args.name.unwrap_or("demo-name".to_string());
        let defaults = args.opt.is_empty() && args.app.is_empty();
        let mut app = Self {
            apps: vec![AppUnit {
                name: name.clone(),
                options: fresh_options(defaults),
            }],
            name,
            desc: args.desc.unwrap_or("demo-desc".to_string()),
            owner: args.owner.unwrap_or("demo-owner".to_string()),
            dest: args.dest.unwrap_or(current_dir().unwrap_or_default()),
            staged: None,
        };
        if args.app.is_empty() {
            for opt in args.opt {
                if let Ok(id) = opt.parse::<OptId>() {
                    app.nested_check(View::App(0), id, Some(true));
                }
            }
            return Ok(app);
        }
        app.apps.clear();
        for spec in &args.app {
            let (name, opts) = spec.split_once(':').unwrap_or((spec, ""));
            app.apps.push(AppUnit {
                name: name.to_string(),
                options: fresh_options(false),
            });
            let at = View::App(app.apps.len() - 1);
            for opt in opts.split(',').filter(|o| !o.is_empty()) {
                let id = parse_opt(name, opt)?;
                if id.def().scope == Scope::Repo {
                    return Err(InputError::App(
                        name.to_string(),
                        format!("`{opt}` is repo-wide: pass it with --opt"),
                    )
                    .into());
                }
                app.nested_check(at, id, Some(true));
            }
        }
        for opt in &args.opt {
            let id = parse_opt(&app.name, opt)?;
            if id.def().scope != Scope::Repo {
                return Err(InputError::App(
                    app.name.clone(),
                    format!("`{opt}` belongs to an app: pass it in --app"),
                )
                .into());
            }
            app.nested_check(View::Repo, id, Some(true));
        }
        Ok(app)
    }

    /// Whether the repo holds several apps, each in its own folder.
    pub(crate) fn is_multi(&self) -> bool {
        self.apps.len() > 1
    }

    /// Where app `at` renders, relative to the repo root.
    pub(crate) fn prefix(&self, at: usize) -> PathBuf {
        if self.is_multi() {
            PathBuf::from(APPS_DIR).join(&self.apps[at].name)
        } else {
            PathBuf::new()
        }
    }

    /// Renders the repo into memory.
    ///
    /// # Errors
    ///
    /// Returns an error if a template names an option that doesn't exist, or
    /// if two active options render the same file. `staged` is left untouched.
    pub(crate) fn generate(&mut self) -> Result<()> {
        self.staged = Some(engine::generate(self)?);
        Ok(())
    }

    /// Writes the last render to `dest/<name>`.
    ///
    /// # Errors
    ///
    /// Returns an error if a field breaks its rule, if nothing has been
    /// generated yet, if the target already exists, or if creating a directory
    /// or writing a file fails. A failure part-way leaves the files already
    /// written on disk.
    pub(crate) fn create(&self) -> Result<()> {
        self.validate()?;
        let Some(tree) = &self.staged else {
            return Err(NotGenerated.into());
        };
        let target = self.dest.join(&self.name);
        if target.exists() {
            return Err(TargetExists(target).into());
        }
        write_tree(tree, &target)
    }

    /// Checks every field against the rule the form enforces while typing,
    /// every app name, and that no app checks two options that conflict.
    ///
    /// # Errors
    ///
    /// Returns an error naming the first two checked options that conflict, or
    /// the first field or app name that breaks its rule.
    fn validate(&self) -> Result<()> {
        if let Some((opt_1, opt_2)) = self.conflict() {
            return Err(Error::Input(InputError::Conflict(
                <&'static str>::from(opt_1),
                <&'static str>::from(opt_2),
            )));
        }
        for field in Field::iter() {
            field.validate(field.get(self)).map_err(|reason| {
                InputError::Field(format!("--{field:?}").to_lowercase(), reason)
            })?;
        }
        if self.is_multi() {
            let mut seen = HashSet::new();
            for unit in &self.apps {
                Field::Name
                    .validate(&unit.name)
                    .map_err(|reason| InputError::App(unit.name.clone(), reason.to_string()))?;
                if !seen.insert(unit.name.as_str()) {
                    return Err(InputError::App(unit.name.clone(), "named twice".into()).into());
                }
            }
        }
        Ok(())
    }

    /// The first two checked options, in any one app, that can't take part
    /// together.
    pub(crate) fn conflict(&self) -> Option<(OptId, OptId)> {
        self.apps.iter().find_map(|unit| conflict_in(&unit.options))
    }

    /// The app whose list `view` edits: the first one for the repo view,
    /// since every app holds the same repo-wide options.
    fn index(&self, view: View) -> usize {
        match view {
            View::Repo => 0,
            View::App(at) => at.min(self.apps.len() - 1),
        }
    }

    /// The options checked in `view`'s app.
    pub(crate) fn checked(&self, view: View) -> HashSet<OptId> {
        checked_in(&self.apps[self.index(view)].options)
    }

    /// Copies the repo-wide options of app `from` to every other app.
    fn sync_repo(&mut self, from: usize) {
        let source: Vec<(OptId, bool)> = self.apps[from]
            .options
            .iter()
            .filter(|o| o.id.def().scope == Scope::Repo)
            .map(|o| (o.id, o.checked))
            .collect();
        for unit in &mut self.apps {
            for opt in &mut unit.options {
                if let Some(&(_, checked)) = source.iter().find(|(id, _)| *id == opt.id) {
                    opt.checked = checked;
                }
            }
        }
    }

    /// Toggles `id` the way the options pane does: like [`Self::nested_check`],
    /// and every option that gets checked on the way also checks its default
    /// children, which can then be unchecked one by one.
    pub(crate) fn toggle(&mut self, view: View, id: OptId) {
        let at = self.index(view);
        let repo_only = view == View::Repo;
        let before = self.checked(view);
        nested(&mut self.apps[at].options, id, None, repo_only);
        let now = self.checked(view);
        for parent in now.difference(&before) {
            check_defaults(&mut self.apps[at].options, *parent, repo_only);
        }
        self.sync_repo(at);
    }

    /// Checks or toggles `id`; once checked, also checks its parent and every
    /// option it requires, so it can take part. In the repo view only
    /// repo-wide options change.
    pub(crate) fn nested_check(&mut self, view: View, id: OptId, check: Option<bool>) {
        let at = self.index(view);
        nested(&mut self.apps[at].options, id, check, view == View::Repo);
        self.sync_repo(at);
    }

    /// The rows the options pane shows for `view`, in tree order: the
    /// repo-wide options, or an app's own. An option whose parent isn't
    /// listed sits at the top level.
    pub(crate) fn rows(&self, view: View) -> Vec<Row> {
        let multi = self.is_multi();
        let scope = if view == View::Repo {
            Scope::Repo
        } else {
            Scope::App
        };
        let shown = |id: OptId| {
            let def = id.def();
            def.scope == scope && !(multi && def.single_app_only)
        };
        // The repo view gathers what every app checked, so `cargo.deny`
        // under `rust.common` reads as taking part if any app has Rust.
        let checked: HashSet<OptId> = match view {
            View::Repo => self
                .apps
                .iter()
                .flat_map(|u| checked_in(&u.options))
                .collect(),
            View::App(_) => self.checked(view),
        };
        let mut rows = Vec::new();
        push_rows(&checked, &shown, None, 0, &mut rows);
        // Options hoisted from under a hidden parent (`flake.package` under
        // the repo-wide `flake`) go after the app's own trees.
        let (own, hoisted): (Vec<Vec<Row>>, Vec<Vec<Row>>) = trees(rows)
            .into_iter()
            .partition(|tree| tree[0].id.def().parent.is_none_or(shown));
        own.into_iter().chain(hoisted).flatten().collect()
    }

    /// Adds an app named `name`, with no option checked but the repo-wide ones.
    pub(crate) fn add_app(&mut self, name: String) {
        let mut options = self.apps[0].options.clone();
        for opt in &mut options {
            if opt.id.def().scope == Scope::App {
                opt.checked = false;
            }
        }
        self.apps.push(AppUnit { name, options });
    }

    /// Removes app `at`; the last one stays.
    pub(crate) fn remove_app(&mut self, at: usize) {
        if self.apps.len() > 1 && at < self.apps.len() {
            self.apps.remove(at);
        }
    }
}

/// Parses `opt`, as given for app `app`.
fn parse_opt(app: &str, opt: &str) -> Result<OptId> {
    opt.parse::<OptId>()
        .map_err(|_| InputError::App(app.to_string(), format!("no option `{opt}`")).into())
}

/// Splits `rows` into its top-level trees: each top-level row with the rows
/// under it.
fn trees(rows: Vec<Row>) -> Vec<Vec<Row>> {
    let mut out: Vec<Vec<Row>> = Vec::new();
    for row in rows {
        match out.last_mut() {
            Some(tree) if row.depth > 0 => tree.push(row),
            _ => out.push(vec![row]),
        }
    }
    out
}

/// Pushes the rows `shown` lets through under `parent`, depth first. A shown
/// option under a hidden parent is pushed where the parent would be.
fn push_rows(
    checked: &HashSet<OptId>,
    shown: &dyn Fn(OptId) -> bool,
    parent: Option<OptId>,
    depth: usize,
    rows: &mut Vec<Row>,
) {
    for id in OptId::iter().filter(|id| id.def().parent == parent) {
        if !shown(id) {
            push_rows(checked, shown, Some(id), depth, rows);
            continue;
        }
        let is_checked = checked.contains(&id);
        rows.push(Row {
            id,
            checked: is_checked,
            conflicted: !is_checked && checked.iter().any(|&c| id.conflicts_with(c)),
            idle: is_checked && !id.is_active(checked),
            depth,
        });
        push_rows(checked, shown, Some(id), depth + 1, rows);
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    /// An app built from the command line with these fields.
    fn app(name: &str, owner: &str, desc: &str) -> App {
        App::new(Args {
            name: Some(name.into()),
            owner: Some(owner.into()),
            desc: Some(desc.into()),
            ..Args::default()
        })
        .unwrap()
    }

    #[test]
    fn valid_fields_pass() {
        assert!(app("demo-app", "demo", "Demo app.").validate().is_ok());
    }

    #[rstest]
    #[case::name_escapes_dest("../escape", "demo", "Demo app.")]
    #[case::owner_double_dash("demo-app", "my--org", "Demo app.")]
    #[case::quote_breaks_toml("demo-app", "demo", "say \"hi\"")]
    fn a_bad_field_is_an_error(#[case] name: &str, #[case] owner: &str, #[case] desc: &str) {
        let err = app(name, owner, desc).validate().unwrap_err();
        assert!(matches!(err, Error::Input(InputError::Field(..))), "{err}");
    }

    /// A valid app with exactly these options given on the command line.
    fn with_opts(opts: &[&str]) -> App {
        App::new(Args {
            opt: opts.iter().map(|&o| String::from(o)).collect(),
            ..Args::default()
        })
        .unwrap()
    }

    #[test]
    fn checked_options_that_conflict_are_an_error() {
        let err = with_opts(&["rust.firmware", "rust.common"])
            .validate()
            .unwrap_err();
        assert!(
            matches!(err, Error::Input(InputError::Conflict(..))),
            "{err}"
        );
    }

    #[test]
    fn an_unchecked_option_never_conflicts() {
        assert!(with_opts(&["rust.server"]).validate().is_ok());
    }

    #[test]
    fn checking_an_option_checks_what_it_requires() {
        let mut app = with_opts(&[]);
        app.nested_check(View::App(0), OptId::RustMcpHttp, Some(true));
        assert!(OptId::RustMcpHttp.is_active(&app.checked(View::App(0))));
    }

    #[rstest]
    #[case::top_level_default(OptId::Just, true)]
    #[case::default_under_default(OptId::GithubCi, true)]
    #[case::default_under_unchecked_parent(OptId::WebReact, false)]
    #[case::default_under_unchecked_rust(OptId::CargoDeny, false)]
    fn the_tui_starts_with_defaults_whose_parents_are_defaults(
        #[case] id: OptId,
        #[case] checked: bool,
    ) {
        assert_eq!(with_opts(&[]).checked(View::App(0)).contains(&id), checked);
    }

    #[test]
    fn headless_checks_only_what_it_is_given() {
        let checked = with_opts(&["rust.common"]).checked(View::App(0));
        assert!(checked.contains(&OptId::RustCommon));
        assert!(!checked.contains(&OptId::CargoDeny));
    }

    #[rstest]
    #[case::the_parent(OptId::RustCommon)]
    #[case::a_child_that_checks_the_parent(OptId::CargoCodecov)]
    fn toggling_on_checks_the_default_children(#[case] id: OptId) {
        let mut app = with_opts(&["just"]);
        app.toggle(View::App(0), id);
        let checked = app.checked(View::App(0));
        for tool in [OptId::CargoDeny, OptId::CargoAudit, OptId::CargoUdeps] {
            assert!(checked.contains(&tool), "{tool:?}");
        }
        assert_eq!(
            checked.contains(&OptId::CargoCodecov),
            id == OptId::CargoCodecov
        );
    }

    #[test]
    fn a_default_child_can_be_unchecked() {
        let mut app = with_opts(&["just"]);
        app.toggle(View::App(0), OptId::RustCommon);
        app.toggle(View::App(0), OptId::CargoDeny);
        let checked = app.checked(View::App(0));
        assert!(checked.contains(&OptId::RustCommon));
        assert!(!checked.contains(&OptId::CargoDeny));
    }

    #[test]
    fn toggling_web_on_checks_react() {
        let mut app = with_opts(&["just"]);
        app.toggle(View::App(0), OptId::Web);
        assert!(app.checked(View::App(0)).contains(&OptId::WebReact));
    }

    /// The app from `--app` specs and repo-wide `--opt`s, or why it can't be.
    fn with_apps(apps: &[&str], opts: &[&str]) -> Result<App> {
        App::new(Args {
            app: apps.iter().map(|&a| a.into()).collect(),
            opt: opts.iter().map(|&o| o.into()).collect(),
            ..Args::default()
        })
    }

    #[rstest]
    #[case::repo_option_in_an_app(&["api:rust.server,just"], &[])]
    #[case::app_option_in_opt(&["api:rust.server", "web:web.react"], &["rust.can"])]
    #[case::unknown_option(&["api:rust.nope"], &[])]
    fn a_misplaced_option_is_an_error(#[case] apps: &[&str], #[case] opts: &[&str]) {
        let err = with_apps(apps, opts).unwrap_err();
        assert!(matches!(err, Error::Input(InputError::App(..))), "{err}");
    }

    #[test]
    fn two_apps_with_one_name_are_an_error() {
        let app = with_apps(&["api:rust.server", "api:web.react"], &[]).unwrap();
        let err = app.validate().unwrap_err();
        assert!(matches!(err, Error::Input(InputError::App(..))), "{err}");
    }

    #[test]
    fn repo_wide_options_are_shared_by_every_app() {
        let mut app = with_apps(&["api:rust.server", "web:web.react"], &["just"]).unwrap();
        assert!(app.checked(View::App(1)).contains(&OptId::Just));
        app.toggle(View::App(1), OptId::Typos);
        assert!(app.checked(View::App(0)).contains(&OptId::Typos));
    }

    #[test]
    fn the_repo_view_never_touches_an_app() {
        let mut app = with_apps(&["api:rust.server", "web:web.react"], &[]).unwrap();
        // cargo.deny sits under rust.common; from the repo view it must not
        // check Rust in the web app.
        app.toggle(View::Repo, OptId::CargoDeny);
        assert!(!app.checked(View::App(1)).contains(&OptId::RustCommon));
        assert!(app.checked(View::App(1)).contains(&OptId::CargoDeny));
    }

    #[test]
    fn rows_split_repo_and_app_options() {
        let app = with_apps(&["api:rust.server", "web:web.react"], &[]).unwrap();
        let repo: Vec<OptId> = app.rows(View::Repo).iter().map(|row| row.id).collect();
        let api: Vec<OptId> = app.rows(View::App(0)).iter().map(|row| row.id).collect();
        assert!(repo.contains(&OptId::CargoDeny) && !repo.contains(&OptId::Rust));
        assert!(api.contains(&OptId::RustServer) && !api.contains(&OptId::Just));
        // One app per folder: the flake package assumes the app at the root.
        assert!(!api.contains(&OptId::FlakePackage));
    }
}
