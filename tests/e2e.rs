//! End-to-end: generate a repo headlessly, then build, lint, test and package
//! it, and run its own `just ci` gate.
//!
//! Slow and needs crates.io, so every case is `#[ignore]`d; run them with
//! `just e2e`, inside the dev shell: the gate calls typos, lychee, taplo,
//! cargo-deny and the rest, which come from praline's flake.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use rstest::rstest;

/// Options every Rust setup starts from.
const BASE: &[&str] = &["just", "flake", "rust", "rust.common"];

/// Repo-wide checks, docs, git and CI: on in every case but `no_tooling`.
const TOOLING: &[&str] = &[
    "editorconfig",
    "typos",
    "lychee",
    "readme",
    "git",
    "github",
    "github.ci",
    "claude",
];

/// Target dir shared by every generated repo, so dependencies build once.
fn shared_target() -> PathBuf {
    env::temp_dir().join("praline-e2e-target")
}

/// What a failed step reports: the command and its output.
type Outcome = Result<(), String>;

/// Runs `cmd` in `dir`; on failure, returns the command line and its stderr.
fn run(dir: &Path, cmd: &str, args: &[&str]) -> Outcome {
    let line = format!("`{cmd} {}` in {}", args.join(" "), dir.display());
    let mut command = Command::new(cmd);
    // `cargo test` describes praline to its tests through these; a user's
    // shell has none, and some tools (cargo-machete) read them.
    for (key, _) in env::vars() {
        let about_praline = [
            "CARGO_PKG_",
            "CARGO_MANIFEST_",
            "CARGO_CRATE_",
            "CARGO_BIN_",
        ]
        .iter()
        .any(|prefix| key.starts_with(prefix));
        if about_praline || key == "CARGO_PRIMARY_PACKAGE" {
            command.env_remove(key);
        }
    }
    let out = command
        .args(args)
        .current_dir(dir)
        .env("CARGO_TARGET_DIR", shared_target())
        .env("NO_COLOR", "1")
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

/// Generates `name` with exactly `opts`, and returns the repo's path.
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
    for opt in opts {
        args.extend(["--opt", opt]);
    }
    run(Path::new("."), env!("CARGO_BIN_EXE_praline"), &args)?;
    Ok(repo)
}

/// Makes `repo` a git repo with one Conventional commit, as `git init` and a
/// first commit would: typos, committed and cargo package read git.
fn commit_all(repo: &Path) -> Outcome {
    run(
        repo,
        "git",
        &["init", "--quiet", "--initial-branch", "main"],
    )?;
    run(repo, "git", &["add", "--all"])?;
    run(
        repo,
        "git",
        &[
            "-c",
            "user.name=e2e",
            "-c",
            "user.email=e2e@example.com",
            "commit",
            "--quiet",
            "--message",
            "chore: scaffold with praline",
        ],
    )
}

/// Builds, lints, tests and packages `repo`, then runs its own gate.
fn check_rust_repo(repo: &Path) -> Outcome {
    commit_all(repo)?;
    run(repo, "cargo", &["build", "--all-targets"])?;
    run(
        repo,
        "cargo",
        &["clippy", "--all-targets", "--", "-D", "warnings"],
    )?;
    run(repo, "cargo", &["test"])?;
    run(repo, "cargo", &["package", "--no-verify", "--allow-dirty"])?;
    run(repo, "just", &["ci"])
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
    &[
        "rust.server",
        "rust.server.ws",
        "rust.can",
        "rust.notif",
        "rust.mcp",
        "rust.mcp.http",
        "codecov",
    ],
)]
#[case::client_side("e2e-client-side", &["rust.client_ws", "rust.can", "rust.notif"])]
#[ignore = "slow, needs crates.io: run with `just e2e`"]
fn generated_repo_builds_lints_passes_and_packages(
    #[case] name: &str,
    #[case] opts: &[&str],
) -> Outcome {
    let all: Vec<&str> = BASE.iter().chain(TOOLING).chain(opts).copied().collect();
    let repo = generate(name, &all)?;
    check_rust_repo(&repo)?;
    // The commit format the CI job checks, and the workflows the gate lints.
    run(&repo, "just", &["commits"])
}

/// The stripped-down render: a Rust crate with no repo-wide tooling still
/// passes its gate.
#[test]
#[ignore = "slow, needs crates.io: run with `just e2e`"]
fn generated_repo_without_tooling_passes_its_gate() -> Outcome {
    let repo = generate("e2e-no-tooling", BASE)?;
    check_rust_repo(&repo)
}

/// A repo with no Rust: just the flake, `just` and the repo-wide tooling.
#[test]
#[ignore = "slow, needs crates.io: run with `just e2e`"]
fn generated_repo_without_rust_passes_its_gate() -> Outcome {
    let all: Vec<&str> = ["just", "flake"].iter().chain(TOOLING).copied().collect();
    let repo = generate("e2e-no-rust", &all)?;
    commit_all(&repo)?;
    run(&repo, "just", &["ci"])?;
    run(&repo, "just", &["commits"])
}
