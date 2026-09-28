/// Tool categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::Display)]
pub(crate) enum Category {
    /// Format code.
    _Format,
    /// Lint code.
    _Lint,
    /// Run tests.
    _Test,
    /// Scan for security issues.
    _Security,
    /// Build the project.
    Build,
    /// Publish or release.
    _Release,
    /// Version control.
    _Git,
    /// Environment and tooling setup.
    _Env,
}
