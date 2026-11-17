use std::collections::HashSet;
use std::env::current_dir;
use std::path::PathBuf;

use strum::IntoEnumIterator;

use crate::app::engine::{Tree, write_tree};
use crate::app::opt::OptId;
use crate::error::EngineError::{NotGenerated, TargetExists};
use crate::prelude::*;

/// Optional tools.
mod opt;

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
    /// Options available.
    pub(crate) options: Vec<Opt>,
    /// Last render, shown in the preview and written on export.
    pub(crate) staged: Option<Tree>,
}

/// Selectable repo option.
#[derive(Debug, Clone)]
pub(crate) struct Opt {
    /// Which option: the key into the table.
    pub(crate) id: OptId,
    /// Ticked by the user (it can still be inactive, see `Ctx::active`).
    pub(crate) checked: bool,
    /// Conflicted by other options.
    pub(crate) conflicted: bool,
}

impl App {
    /// Set a new app.
    pub(crate) fn new(args: Args) -> Self {
        let mut list = Vec::new();
        OptId::sorted_list(&mut list, None);
        let mut app = Self {
            name: args.name.unwrap_or("demo-name".to_string()),
            desc: args.desc.unwrap_or("demo-desc".to_string()),
            owner: args.owner.unwrap_or("demo-owner".to_string()),
            dest: args.dest.unwrap_or(current_dir().unwrap_or_default()),
            options: list
                .into_iter()
                .map(|id| Opt {
                    id,
                    checked: args.opt.is_empty() && id.starts_checked(),
                    conflicted: false,
                })
                .collect(),
            staged: None,
        };
        for opt in args.opt {
            if let Ok(id) = opt.parse::<OptId>() {
                app.nested_check(id, Some(true));
            }
        }
        app
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

    /// Checks every field against the rule the form enforces while typing.
    /// and that no two checked options conflict.
    ///
    /// # Errors
    ///
    /// Returns an error naming the first two checked options that conflict, or
    /// the first field that breaks its rule.
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
        Ok(())
    }

    /// Flags every unchecked option that clashes with a checked one.
    pub(crate) fn mark_conflicts(&mut self) {
        let checked: Vec<OptId> = self
            .options
            .iter()
            .filter(|o| o.checked)
            .map(|o| o.id)
            .collect();
        for opt in &mut self.options {
            opt.conflicted = !opt.checked && checked.iter().any(|&c| opt.id.conflicts_with(c));
        }
    }

    /// The first two checked options that can't take part together, if any.
    pub(crate) fn conflict(&self) -> Option<(OptId, OptId)> {
        let checked: Vec<OptId> = self
            .options
            .iter()
            .filter(|o| o.checked)
            .map(|o| o.id)
            .collect();
        checked.iter().enumerate().find_map(|(at, &a)| {
            checked[at + 1..]
                .iter()
                .find(|&&b| a.conflicts_with(b))
                .map(|&b| (a, b))
        })
    }

    /// The options currently checked.
    fn checked(&self) -> HashSet<OptId> {
        self.options
            .iter()
            .filter(|o| o.checked)
            .map(|o| o.id)
            .collect()
    }

    /// Toggles `id` the way the options pane does: like [`Self::nested_check`],
    /// and every option that gets checked on the way also checks its default
    /// children, which can then be unchecked one by one.
    pub(crate) fn toggle(&mut self, id: OptId) {
        let before = self.checked();
        self.nested_check(id, None);
        let now = self.checked();
        for parent in now.difference(&before) {
            self.check_defaults(*parent);
        }
    }

    /// Checks every default child of `parent`, and theirs.
    fn check_defaults(&mut self, parent: OptId) {
        for child in OptId::iter().filter(|c| c.def().parent == Some(parent) && c.def().default) {
            self.nested_check(child, Some(true));
            self.check_defaults(child);
        }
    }

    /// Checks or toggles `id`; once checked, also checks its parent and every
    /// option it requires, so it can take part.
    pub(crate) fn nested_check(&mut self, id: OptId, check: Option<bool>) {
        let Some(opt) = self.options.iter_mut().find(|o| o.id == id) else {
            return;
        };
        opt.checked = check.unwrap_or(!opt.checked);

        if !opt.checked {
            return;
        }
        let def = id.def();
        for dep in def.parent.into_iter().chain(def.requires.iter().copied()) {
            self.nested_check(dep, Some(true));
        }
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
        app.nested_check(OptId::RustMcpHttp, Some(true));
        let checked: HashSet<OptId> = app
            .options
            .iter()
            .filter(|o| o.checked)
            .map(|o| o.id)
            .collect();
        assert!(OptId::RustMcpHttp.is_active(&checked));
    }

    #[rstest]
    #[case::top_level_default(OptId::Just, true)]
    #[case::default_under_default(OptId::GithubCi, true)]
    #[case::default_under_unchecked_parent(OptId::WebReact, false)]
    #[case::default_under_unchecked_rust(OptId::RustCommonDeny, false)]
    fn the_tui_starts_with_defaults_whose_parents_are_defaults(
        #[case] id: OptId,
        #[case] checked: bool,
    ) {
        assert_eq!(with_opts(&[]).checked().contains(&id), checked);
    }

    #[test]
    fn headless_checks_only_what_it_is_given() {
        let checked = with_opts(&["rust.common"]).checked();
        assert!(checked.contains(&OptId::RustCommon));
        assert!(!checked.contains(&OptId::RustCommonDeny));
    }

    #[rstest]
    #[case::the_parent(OptId::RustCommon)]
    #[case::a_child_that_checks_the_parent(OptId::RustCommonCodecov)]
    fn toggling_on_checks_the_default_children(#[case] id: OptId) {
        let mut app = with_opts(&["just"]);
        app.toggle(id);
        let checked = app.checked();
        for tool in [
            OptId::RustCommonDeny,
            OptId::RustCommonAudit,
            OptId::RustCommonUdeps,
        ] {
            assert!(checked.contains(&tool), "{tool:?}");
        }
        assert_eq!(
            checked.contains(&OptId::RustCommonCodecov),
            id == OptId::RustCommonCodecov
        );
    }

    #[test]
    fn a_default_child_can_be_unchecked() {
        let mut app = with_opts(&["just"]);
        app.toggle(OptId::RustCommon);
        app.toggle(OptId::RustCommonDeny);
        let checked = app.checked();
        assert!(checked.contains(&OptId::RustCommon));
        assert!(!checked.contains(&OptId::RustCommonDeny));
    }

    #[test]
    fn toggling_web_on_checks_react() {
        let mut app = with_opts(&["just"]);
        app.toggle(OptId::Web);
        assert!(app.checked().contains(&OptId::WebReact));
    }
}
