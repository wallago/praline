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
                    checked: args.opt.is_empty() && id.def().default,
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

    /// Check current option and parent if not already check.
    pub(crate) fn nested_check(&mut self, id: OptId, check: Option<bool>) {
        let Some(opt) = self.options.iter_mut().find(|o| o.id == id) else {
            return;
        };
        opt.checked = check.unwrap_or(!opt.checked);
        let Some(parent) = id.def().parent else {
            return;
        };

        if opt.checked {
            self.nested_check(parent, Some(true));
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
}
