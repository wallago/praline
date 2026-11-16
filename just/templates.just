# ── Templates ─────────────────────────────────────────────────

# Generate each setup, then run its own `just ci` (slow, needs crates.io, npm and the dev shell's tools)
[group('templates')]
e2e *ARGS:
    cargo nextest run --test e2e --run-ignored only {{ ARGS }}

# Re-lock the web.react template after editing its package.json
[group('templates')]
web-lock:
    cd templates/web.react && pnpm install --lockfile-only
