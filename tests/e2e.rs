//! End-to-end: generate a repo headlessly, then build, lint and test it.
//!
//! Slow and needs crates.io, so every case is `#[ignore]`d; run them with
//! `just e2e`.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use rstest::rstest;

/// Options every Rust setup starts from.
const BASE: &[&str] = &["just", "flake", "rust", "rust.common"];

/// Target dir shared by every generated repo, so dependencies build once.
fn shared_target() -> PathBuf {
    env::temp_dir().join("praline-e2e-target")
}

/// What a failed step reports: the command and its output.
type Outcome = Result<(), String>;

/// Runs `cmd` in `dir`; on failure, returns the command line and its stderr.
fn run(dir: &Path, cmd: &str, args: &[&str]) -> Outcome {
    let line = format!("`{cmd} {}` in {}", args.join(" "), dir.display());
    let out = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .env("CARGO_TARGET_DIR", shared_target())
        .output()
        .map_err(|err| format!("{line}: can't spawn: {err}"))?;
    if out.status.success() {
        return Ok(());
    }
    Err(format!(
        "{line} failed\n{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    ))
}

/// Generates `name` with `BASE` plus `opts`, and returns the repo's path.
fn generate(name: &str, opts: &[&str]) -> Result<PathBuf, String> {
    let dest = env::temp_dir().join("praline-e2e");
    let repo = dest.join(name);
    let _ = fs::remove_dir_all(&repo);
    fs::create_dir_all(&dest).map_err(|err| format!("can't create {}: {err}", dest.display()))?;
    let dest_arg = dest.to_string_lossy();
    let mut args = vec![
        "--headless",
        "--name",
        name,
        "--owner",
        "demo",
        "--desc",
        "Demo.",
        "--dest",
        &dest_arg,
    ];
    for opt in BASE.iter().chain(opts) {
        args.extend(["--opt", opt]);
    }
    run(Path::new("."), env!("CARGO_BIN_EXE_praline"), &args)?;
    Ok(repo)
}

#[rstest]
#[case::common("e2e-common", &[])]
#[case::mcp("e2e-mcp", &["rust.mcp"])]
#[case::mcp_http("e2e-mcp-http", &["rust.server", "rust.mcp", "rust.mcp.http"])]
#[case::mcp_http_ws(
    "e2e-mcp-http-ws",
    &["rust.server", "rust.server.ws", "rust.mcp", "rust.mcp.http"],
)]
#[ignore = "slow, needs crates.io: run with `just e2e`"]
fn generated_repo_builds_lints_and_passes(#[case] name: &str, #[case] opts: &[&str]) -> Outcome {
    let repo = generate(name, opts)?;
    run(&repo, "cargo", &["build", "--all-targets"])?;
    run(
        &repo,
        "cargo",
        &["clippy", "--all-targets", "--", "-D", "warnings"],
    )?;
    run(&repo, "cargo", &["test"])
}
