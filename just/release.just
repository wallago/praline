# ── Release ───────────────────────────────────────────────────

# Step 1: open the release PR (version bump + changelog on `release/vVERSION`). Usage: just release 0.3.0
[group('release')]
release VERSION:
    #!/usr/bin/env bash
    set -euo pipefail
    tag="v{{ VERSION }}"
    branch="release/$tag"
    fail() { echo "✗ $1" >&2; exit 1; }

    gh auth status >/dev/null || fail "gh is not logged in: run \`gh auth login\`"
    test -z "$(git status --porcelain)" || fail "uncommitted changes: commit or stash them first"
    git fetch --quiet --tags origin main
    test "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)" \
        || fail "HEAD is not origin/main: switch to main and pull first"
    ! git rev-parse --verify --quiet "refs/tags/$tag" >/dev/null || fail "tag $tag already exists"

    echo "▶ branch $branch"
    git switch --quiet --create "$branch"
    echo "▶ bump version → {{ VERSION }}"
    cargo set-version "{{ VERSION }}"
    echo "▶ regenerate changelog"
    git-cliff --tag "$tag" --output CHANGELOG.md
    echo "▶ commit and push"
    git add Cargo.toml Cargo.lock CHANGELOG.md
    git commit --quiet --message "chore(release): prepare for $tag"
    git push --quiet --set-upstream origin "$branch"
    echo "▶ open the pull request"
    git-cliff --unreleased --tag "$tag" --strip all \
        | gh pr create --base main --head "$branch" --title "chore(release): prepare for $tag" --body-file -
    echo "✅ merge that PR, then run: just release-publish {{ VERSION }}"

# Step 2, once the release PR is merged: create the GitHub release and its tag; cd.yml builds and publishes. Usage: just release-publish 0.3.0
[confirm("This creates a public tag and GitHub release, and cd.yml publishes from it. Continue?")]
[group('release')]
release-publish VERSION:
    #!/usr/bin/env bash
    set -euo pipefail
    tag="v{{ VERSION }}"
    fail() { echo "✗ $1" >&2; exit 1; }

    gh auth status >/dev/null || fail "gh is not logged in: run \`gh auth login\`"
    git fetch --quiet --tags origin main
    ! git rev-parse --verify --quiet "refs/tags/$tag" >/dev/null || fail "tag $tag already exists"
    on_main="$(git show origin/main:Cargo.toml | sed -n 's/^version *= *"\(.*\)"/\1/p' | head -n 1)"
    test "$on_main" = "{{ VERSION }}" \
        || fail "main is at $on_main, not {{ VERSION }}: merge the release PR first"

    # A version with a pre-release part (0.3.0-alpha.1) is marked as such.
    flags=()
    case "{{ VERSION }}" in *-*) flags+=(--prerelease) ;; esac
    last="$(git describe --tags --abbrev=0 origin/main 2>/dev/null || true)"
    # git-cliff sends the range end to the GitHub API, which 404s on `origin/main`: use the SHA.
    head="$(git rev-parse origin/main)"
    notes="$(mktemp)"
    trap 'rm -f "$notes"' EXIT
    echo "▶ release $tag from origin/main"
    # Notes first, so a git-cliff failure stops before anything is published.
    git-cliff ${last:+"$last..$head"} --tag "$tag" --strip all > "$notes"
    gh release create "$tag" --target "$head" \
        --title "Release $tag" --notes-file "$notes" "${flags[@]}"
    echo "✅ $tag is out; follow the build with: gh run watch"
