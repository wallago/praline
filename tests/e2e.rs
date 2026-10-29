//! End-to-end: generate a repo headlessly, then build, lint, test and package\n//! it.
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
// One option at a time: every template file compiles alone.
#[case::common("e2e-common", &[])]
#[case::server("e2e-server", &["rust.server"])]
#[case::server_ws("e2e-server-ws", &["rust.server", "rust.server.ws"])]
#[case::can("e2e-can", &["rust.can"])]
#[case::client_ws("e2e-client-ws", &["rust.client_ws"])]
#[case::notif("e2e-notif", &["rust.notif"])]
#[case::mcp("e2e-mcp", &["rust.mcp"])]
#[case::mcp_http("e2e-mcp-http", &["rust.server", "rust.mcp", "rust.mcp.http"])]
// Together: options sharing slots (select!, deps, errors, config, routes).
#[case::mcp_http_ws(
    "e2e-mcp-http-ws",
    &["rust.server", "rust.server.ws", "rust.mcp", "rust.mcp.http"],
)]
#[case::kitchen_sink(
    "e2e-kitchen-sink",
    &["rust.server", "rust.server.ws", "rust.can", "rust.notif", "rust.mcp", "rust.mcp.http"],
)]
#[case::client_side("e2e-client-side", &["rust.client_ws", "rust.can", "rust.notif"])]
#[ignore = "slow, needs crates.io: run with `just e2e`"]
fn generated_repo_builds_lints_passes_and_packages(
    #[case] name: &str,
    #[case] opts: &[&str],
) -> Outcome {
    let repo = generate(name, opts)?;
    run(&repo, "cargo", &["build", "--all-targets"])?;
    run(
        &repo,
        "cargo",
        &["clippy", "--all-targets", "--", "-D", "warnings"],
    )?;
    run(&repo, "cargo", &["test"])?;
    run(&repo, "cargo", &["package", "--no-verify", "--allow-dirty"])
}
