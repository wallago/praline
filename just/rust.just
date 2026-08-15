# ── Dev ───────────────────────────────────────────────────────

# Run the app. Usage: just run -- statement.csv bagels.db
[group('dev')]
run *ARGS:
    cargo run -- --config config.toml {{ ARGS }}

# Build the optimized release binary
[group('dev')]
build:
    cargo build --release

# Type-check the whole workspace (fast feedback, no binary)
[group('dev')]
check:
    cargo check --all-targets

# Run the test suite. Usage: just test [pattern]
[group('dev')]
test *ARGS:
    cargo nextest run {{ ARGS }}

# Run the test suite. Usage: just test [pattern]
[group('dev')]
coverage-report:
    cargo llvm-cov nextest --html --open

# Format Rust + Nix sources in place
[group('dev')]
fmt:
    cargo fmt
    nixfmt flake.nix package.nix module.nix

# ── Quality ───────────────────────────────────────────────────

# Lint with clippy, treating warnings as errors
[group('quality')]
lint:
    cargo clippy --all-targets -- -D warnings

# Report unused dependencies
[group('quality')]
udeps:
    cargo machete

# ── Security ──────────────────────────────────────────────────

# Audit dependencies against the RustSec advisory DB
[group('security')]
audit:
    cargo audit

# Check licenses, banned crates, sources and advisories
[group('security')]
deny:
    cargo deny check

# ── CI ────────────────────────────────────────────────────────

# act's default image ships no Rust and no sqlite3; full-latest mirrors the
# real GitHub runner, so a local run predicts the hosted one. act also leaves
# USER unset, which trips scripts running under `set -u`; the image runs as
# uid 1001 (runner), which is what the hosted runner reports too.
act_flags := '-W .github/workflows/ci.yml -P ubuntu-latest=catthehacker/ubuntu:full-latest --env USER=runner --env GIT_PAGER=cat --env PAGER=cat'

# Mirror the full CI pipeline locally. Run before pushing.
[group('ci')]
ci-light:
    @echo "▶ format"
    cargo fmt --check
    @echo "▶ clippy"
    cargo clippy --all-targets -- -D warnings
    @echo "▶ test"
    cargo nextest run
    @echo "▶ doc"
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
    @echo "▶ typos"
    typos
    @echo "▶ unused deps"
    cargo machete
    @echo "▶ deny"
    cargo deny check
    @echo "▶ audit"
    cargo audit
    @echo "▶ msrv"
    cargo msrv verify
    @echo "▶ toml format"
    taplo fmt --check --diff
    @echo "▶ file format"
    editorconfig-checker
    @echo "▶ commits"
    committed -vv HEAD
    @echo "▶ lychee"
    lychee --verbose --no-progress '*.md'
    @echo "▶ nix lint"
    statix check .
    deadnix --fail .
    @echo "▶ flake check"
    nix flake check --print-build-logs --all-systems
    @echo "✅ CI mirror passed"

# Run the full CI pipeline locally. Run before pushing.
[group('ci')]
ci-deep:
    act {{ act_flags }} -s GITHUB_TOKEN="$(gh auth token)"

# Run a single CI job locally. Usage: just ci-job msrv
[group('ci')]
ci-job JOB:
    act {{ act_flags }} -j {{ JOB }} -s GITHUB_TOKEN="$(gh auth token)"

# List the CI job IDs
[group('ci')]
ci-jobs:
    act -l -W .github/workflows/ci.yml
