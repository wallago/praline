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

# Mirror the full CI pipeline locally. Run before pushing.
[group('ci')]
ci:
    @echo "▶ format"
    cargo fmt --check
    @echo "▶ clippy"
    cargo clippy --all-targets -- -D warnings
    @echo "▶ test"
    cargo nextest run
    @echo "▶ typos"
    typos
    @echo "▶ unused deps"
    cargo machete
    @echo "▶ deny"
    cargo deny check
    @echo "▶ audit"
    cargo audit
    @echo "▶ commits"
    committed -vv HEAD
    @echo "▶ nix lint"
    statix check .
    deadnix --fail .
    @echo "▶ flake check"
    nix flake check --print-build-logs --all-systems
    @echo "✅ CI mirror passed"
