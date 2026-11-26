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
    "github.release",
    "claude",
];

/// Rust dependency checks, which the TUI ticks with `rust.common`: on in
/// every Rust case but `no_tooling`.
const RUST_TOOLS: &[&str] = &["cargo.deny", "cargo.audit", "cargo.udeps"];

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
    generate_apps(name, &[], opts)
}

/// Generates `name` with these `--app` specs and repo-wide `opts`, and
/// returns the repo's path.
fn generate_apps(name: &str, apps: &[&str], opts: &[&str]) -> Result<PathBuf, String> {
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
    for app in apps {
        args.extend(["--app", app]);
    }
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
        "cargo.codecov",
        "github.release.crates",
    ],
)]
#[case::client_side("e2e-client-side", &["rust.client_ws", "rust.can", "rust.notif"])]
#[ignore = "slow, needs crates.io: run with `just e2e`"]
fn generated_repo_builds_lints_passes_and_packages(
    #[case] name: &str,
    #[case] opts: &[&str],
) -> Outcome {
    let all: Vec<&str> = BASE
        .iter()
        .chain(RUST_TOOLS)
        .chain(TOOLING)
        .chain(opts)
        .copied()
        .collect();
    let repo = generate(name, &all)?;
    check_rust_repo(&repo)?;
    // The commit format the CI job checks, and the workflows the gate lints.
    run(&repo, "just", &["commits"])
}

/// The stripped-down render: a Rust crate with no dependency checks and no
/// repo-wide tooling still passes its gate.
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

/// Options every web setup starts from.
const WEB_BASE: &[&str] = &["just", "flake", "web", "web.react"];

#[rstest]
#[case::web("e2e-web", &[])]
#[case::web_pages("e2e-web-pages", &["web.pages"])]
#[case::web_pwa("e2e-web-pwa", &["web.pwa"])]
#[case::web_pwa_pages("e2e-web-pwa-pages", &["web.pwa", "web.pages"])]
#[ignore = "slow, needs the npm registry: run with `just e2e`"]
fn generated_web_repo_passes_its_gate(#[case] name: &str, #[case] opts: &[&str]) -> Outcome {
    let all: Vec<&str> = WEB_BASE
        .iter()
        .chain(TOOLING)
        .chain(opts)
        .copied()
        .collect();
    let repo = generate(name, &all)?;
    commit_all(&repo)?;
    // Typecheck, tests, the build and every repo-wide check, from the
    // shipped lockfile: `install` fails if it no longer matches `package.json`.
    run(&repo, "just", &["ci"])?;
    if opts.contains(&"web.pwa") {
        check_pwa_build(&repo)?;
    }
    run(&repo, "just", &["commits"])
}

/// Checks what `just build` left in `site`'s `dist/` makes an installable,
/// offline PWA: the manifest and its icons, and a worker with the build's
/// files written in.
fn check_pwa_build(site: &Path) -> Outcome {
    let dist = site.join("dist");
    let at = dist.display();
    let read = |file: &str| {
        fs::read_to_string(dist.join(file)).map_err(|err| format!("{at}: can't read {file}: {err}"))
    };
    let index = read("index.html")?;
    if !index.contains("manifest.webmanifest") {
        return Err(format!("{at}: index.html links no manifest"));
    }
    let manifest = read("manifest.webmanifest")?;
    for icon in ["icon-192.png", "icon-512.png", "icon-maskable-512.png"] {
        if !manifest.contains(icon) || !dist.join("icons").join(icon).is_file() {
            return Err(format!("{at}: icon {icon} is missing"));
        }
    }
    let worker = read("sw.js")?;
    if worker.contains("__PRECACHE__") || worker.contains("__VERSION__") {
        return Err(format!("{at}: sw.js still has its placeholders"));
    }
    if !worker.contains("assets/") {
        return Err(format!("{at}: sw.js precaches no built asset"));
    }
    Ok(())
}

/// The stripped-down web render: no repo-wide tooling, still green.
#[test]
#[ignore = "slow, needs the npm registry: run with `just e2e`"]
fn generated_web_repo_without_tooling_passes_its_gate() -> Outcome {
    let repo = generate("e2e-web-no-tooling", WEB_BASE)?;
    run(&repo, "just", &["ci"])
}

/// Repo-wide options of a repo with several apps: the shared tooling, plus
/// the Rust dependency checks, which run once over the whole workspace.
const SHARED: &[&str] = &["just", "flake", "cargo.deny", "cargo.audit", "cargo.udeps"];

#[rstest]
// Two Rust apps in one workspace, which can't share one crate (they conflict),
// and a site next to them.
#[case::server_client_site(
    "e2e-apps",
    &[
        "server:rust.server,rust.server.ws",
        "client:rust.client_ws,rust.notif",
        "site:web.react,web.pwa",
    ],
)]
// The smallest workspace: two crates, nothing else.
#[case::two_crates("e2e-apps-two-crates", &["api:rust.server", "worker:rust.notif"])]
#[ignore = "slow, needs crates.io and the npm registry: run with `just e2e`"]
fn generated_multi_app_repo_passes_its_gate(#[case] name: &str, #[case] apps: &[&str]) -> Outcome {
    let opts: Vec<&str> = SHARED.iter().chain(TOOLING).copied().collect();
    let repo = generate_apps(name, apps, &opts)?;
    commit_all(&repo)?;
    // The workspace builds, lints and tests as one, from the root.
    run(&repo, "cargo", &["build", "--workspace", "--all-targets"])?;
    run(
        &repo,
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run(&repo, "cargo", &["test", "--workspace"])?;
    // The root gate: the repo-wide checks, then each app's own `ci`.
    run(&repo, "just", &["ci"])?;
    if apps.iter().any(|app| app.contains("web.pwa")) {
        check_pwa_build(&repo.join("apps/site"))?;
    }
    run(&repo, "just", &["commits"])
}

//------------------------------------------------------------------//
//                             Release                              //
//------------------------------------------------------------------//

/// A `gh` that logs each call to `$GH_LOG`, with the notes or PR body it is
/// handed, and succeeds: what the release recipes send GitHub, without
/// GitHub.
const FAKE_GH: &str = r#"#!/usr/bin/env bash
set -eu
{
    echo "gh $*"
    while [ $# -gt 0 ]; do
        case "$1" in
            --notes-file | --body-file) echo "--- $1"; cat "$2"; echo "---" ;;
        esac
        shift
    done
} >> "$GH_LOG"
"#;

/// A generated repo wired for the release recipes: a bare `origin` holding
/// `main`, and a fake `gh` first on `PATH`.
struct Released {
    /// The generated repo, a clone of `origin`.
    repo: PathBuf,
    /// The bare repo standing in for GitHub.
    origin: PathBuf,
    /// Where the fake `gh` logs.
    gh_log: PathBuf,
    /// `PATH`, with the fake `gh` first.
    path: String,
}

impl Released {
    /// Generates `name` with `apps` and `opts`, commits it and pushes `main`.
    fn new(name: &str, apps: &[&str], opts: &[&str]) -> Result<Self, String> {
        let repo = generate_apps(name, apps, opts)?;
        let fail = |what: &str, err: std::io::Error| format!("{name}: can't {what}: {err}");
        let origin = repo.with_extension("git");
        let bin = repo.with_extension("bin");
        let _ = fs::remove_dir_all(&origin);
        let _ = fs::remove_dir_all(&bin);
        fs::create_dir_all(&bin).map_err(|err| fail("create bin/", err))?;
        let gh = bin.join("gh");
        fs::write(&gh, FAKE_GH).map_err(|err| fail("write gh", err))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&gh, fs::Permissions::from_mode(0o755))
                .map_err(|err| fail("make gh executable", err))?;
        }
        let path = format!("{}:{}", bin.display(), env::var("PATH").unwrap_or_default());
        let gh_log = bin.join("gh.log");
        let released = Self {
            repo,
            origin,
            gh_log,
            path,
        };
        commit_all(&released.repo)?;
        let origin_arg = released.origin.to_string_lossy();
        run(
            &released.repo,
            "git",
            &[
                "init",
                "--quiet",
                "--bare",
                "--initial-branch",
                "main",
                &origin_arg,
            ],
        )?;
        run(
            &released.repo,
            "git",
            &["remote", "add", "origin", &origin_arg],
        )?;
        run(
            &released.repo,
            "git",
            &["push", "--quiet", "origin", "main"],
        )?;
        Ok(released)
    }

    /// Runs `just <args>` in the repo, as a user with `gh` logged in would.
    fn just(&self, args: &[&str]) -> Outcome {
        let line = format!("`just {}` in {}", args.join(" "), self.repo.display());
        let out = Command::new("just")
            .args(args)
            .current_dir(&self.repo)
            .env("PATH", &self.path)
            .env("GH_LOG", &self.gh_log)
            // The changelog template asks GitHub about each commit; there is
            // no such repo.
            .env("GIT_CLIFF_OFFLINE", "true")
            .env("GIT_AUTHOR_NAME", "e2e")
            .env("GIT_AUTHOR_EMAIL", "e2e@example.com")
            .env("GIT_COMMITTER_NAME", "e2e")
            .env("GIT_COMMITTER_EMAIL", "e2e@example.com")
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

    /// What the fake `gh` was asked so far.
    fn gh_calls(&self) -> String {
        fs::read_to_string(&self.gh_log).unwrap_or_default()
    }

    /// `path` as `origin` has it on `rev`.
    fn on_origin(&self, rev: &str, path: &str) -> Result<String, String> {
        let out = Command::new("git")
            .args(["show", &format!("{rev}:{path}")])
            .current_dir(&self.origin)
            .output()
            .map_err(|err| format!("can't spawn git show: {err}"))?;
        if !out.status.success() {
            return Err(format!(
                "origin has no {path} on {rev}: {}",
                String::from_utf8_lossy(&out.stderr)
            ));
        }
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }

    /// Merges the release PR, as GitHub's squash merge would: one commit on
    /// `main` titled like the PR, with ` (#1)` after it.
    fn merge(&self, branch: &str, title: &str) -> Outcome {
        let repo = &self.repo;
        run(repo, "git", &["switch", "--quiet", "main"])?;
        run(repo, "git", &["merge", "--quiet", "--squash", branch])?;
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
                &format!("{title} (#1)"),
            ],
        )?;
        run(repo, "git", &["push", "--quiet", "origin", "main"])
    }
}

/// Fails with `what` unless `cond` holds.
fn ensure(cond: bool, what: impl FnOnce() -> String) -> Outcome {
    if cond { Ok(()) } else { Err(what()) }
}

/// Both release steps, run for real against a local `origin` and a fake
/// `gh`: the PR branch holds the bumped versions and the changelog, publishing
/// refuses until it is merged, then tags main with notes.
#[rstest]
#[case::rust("e2e-release-rust", &[], &["rust", "rust.common"], &["Cargo.toml"])]
#[case::web("e2e-release-web", &[], &["web", "web.react"], &["package.json"])]
#[case::apps(
    "e2e-release-apps",
    &["api:rust.server", "worker:rust.notif", "site:web.react"],
    &[],
    &["apps/api/Cargo.toml", "apps/worker/Cargo.toml", "apps/site/package.json"],
)]
#[case::no_stack("e2e-release-no-stack", &[], &[], &[])]
#[ignore = "needs git, just, git-cliff and cargo-edit; Rust cases need crates.io: run with `just e2e`"]
fn generated_repo_releases_in_two_steps(
    #[case] name: &str,
    #[case] apps: &[&str],
    #[case] stack: &[&str],
    #[case] versioned: &[&str],
) -> Outcome {
    let opts: Vec<&str> = ["just", "git", "github", "github.release"]
        .iter()
        .chain(stack)
        .copied()
        .collect();
    let rel = Released::new(name, apps, &opts)?;

    rel.just(&["release", "0.2.0"])?;
    let branch = "release/v0.2.0";
    for file in versioned {
        let text = rel.on_origin(branch, file)?;
        ensure(text.contains("\"0.2.0\""), || {
            format!("{name}: {file} on {branch} is not at 0.2.0:\n{text}")
        })?;
    }
    let changelog = rel.on_origin(branch, "CHANGELOG.md")?;
    ensure(changelog.contains("## 0.2.0"), || {
        format!("{name}: no 0.2.0 section in the changelog:\n{changelog}")
    })?;
    ensure(rel.gh_calls().contains("gh pr create --base main"), || {
        format!("{name}: no PR opened:\n{}", rel.gh_calls())
    })?;

    // Not merged yet: nothing gets published.
    let early = rel.just(&["--yes", "release-publish", "0.2.0"]);
    ensure(
        early
            .as_ref()
            .is_err_and(|err| err.contains("merge the release PR first")),
        || format!("{name}: publishing before the merge didn't stop on it: {early:?}"),
    )?;
    ensure(!rel.gh_calls().contains("gh release create"), || {
        format!("{name}: gh release create ran anyway:\n{}", rel.gh_calls())
    })?;

    rel.merge(branch, "chore(release): prepare for v0.2.0")?;
    rel.just(&["--yes", "release-publish", "0.2.0"])?;
    let calls = rel.gh_calls();
    let release = calls
        .lines()
        .find(|line| line.starts_with("gh release create v0.2.0 --target "))
        .ok_or_else(|| format!("{name}: no release created:\n{calls}"))?;
    ensure(!release.contains("--prerelease"), || {
        format!("{name}: 0.2.0 marked as a pre-release: {release}")
    })?;
    // The target is main's SHA, never the `origin/main` name the GitHub API
    // can't resolve.
    let target = release.split_whitespace().nth(5).unwrap_or_default();
    ensure(
        target.len() == 40 && target.chars().all(|c| c.is_ascii_hexdigit()),
        || format!("{name}: the release targets {target:?}, not a SHA"),
    )?;
    // The notes git-cliff wrote for this release, not an empty body.
    let notes = calls
        .split("--- --notes-file\n")
        .nth(1)
        .and_then(|rest| rest.split("\n---").next())
        .unwrap_or_default();
    ensure(
        notes.to_lowercase().contains("scaffold with praline"),
        || format!("{name}: the release notes miss the commits:\n{calls}"),
    )?;
    Ok(())
}

/// A version with a pre-release part is released as a GitHub pre-release,
/// and a malformed version is refused before anything changes.
#[test]
#[ignore = "needs git, just and git-cliff: run with `just e2e`"]
fn a_pre_release_is_marked_and_a_bad_version_refused() -> Outcome {
    let rel = Released::new(
        "e2e-release-rc",
        &[],
        &["just", "git", "github", "github.release"],
    )?;
    ensure(rel.just(&["release", "1.0"]).is_err(), || {
        "`just release 1.0` went through".to_string()
    })?;
    ensure(rel.gh_calls().is_empty(), || {
        format!("gh called for a bad version:\n{}", rel.gh_calls())
    })?;
    rel.just(&["release", "1.0.0-rc.1"])?;
    rel.merge(
        "release/v1.0.0-rc.1",
        "chore(release): prepare for v1.0.0-rc.1",
    )?;
    rel.just(&["--yes", "release-publish", "1.0.0-rc.1"])?;
    ensure(
        rel.gh_calls().lines().any(|line| {
            line.starts_with("gh release create v1.0.0-rc.1") && line.ends_with("--prerelease")
        }),
        || format!("not a pre-release:\n{}", rel.gh_calls()),
    )
}
