//! Runs the built binary the way a user would.

use std::process::{Command, Output};

/// Runs the crate's binary with `args` and returns what it printed.
fn run(args: &[&str]) -> std::io::Result<Output> {
    Command::new(env!("CARGO_BIN_EXE_{name}")).args(args).output()
}

#[test]
fn version_names_the_binary() -> std::io::Result<()> {
    let out = run(&["--version"])?;
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.starts_with(concat!("{name} ", env!("CARGO_PKG_VERSION"))),
        "{stdout}"
    );
    Ok(())
}

#[test]
fn help_lists_the_flags() -> std::io::Result<()> {
    let out = run(&["--help"])?;
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    for flag in ["--config", "--verbose", "--version"] {
        assert!(stdout.contains(flag), "{flag} missing from:\n{stdout}");
    }
    Ok(())
}

#[test]
fn bad_config_path_exits_with_error() -> std::io::Result<()> {
    let out = run(&["--config", "/nonexistent/config.toml"])?;
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("/nonexistent/config.toml"), "{stderr}");
    Ok(())
}
