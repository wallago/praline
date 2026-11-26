# {if:just}
# ── Release ───────────────────────────────────────────────────

# Step 1: open the release PR, with the version bumped and the changelog written. Usage: just release 1.2.0
[group('release')]
release VERSION:
    #!/usr/bin/env bash
    set -euo pipefail
    version="{{ VERSION }}"
    tag="v$version"
    branch="release/$tag"
    fail() { echo "✗ $1" >&2; exit 1; }

    [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]] \
        || fail "$version is not a version like 1.2.0 or 1.2.0-rc.1"
    gh auth status >/dev/null || fail "gh is not logged in: run \`gh auth login\`"
    test -z "$(git status --porcelain)" || fail "uncommitted changes: commit or stash them first"
    git fetch --quiet --tags origin main
    test "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)" \
        || fail "HEAD is not origin/main: switch to main and pull first"
    ! git rev-parse --verify --quiet "refs/tags/$tag" >/dev/null || fail "tag $tag already exists"

    echo "▶ branch $branch"
    git switch --quiet --create "$branch"
    # {slot:release.bump}
    echo "▶ write the changelog"
    git-cliff --tag "$tag" --output CHANGELOG.md
    echo "▶ commit and push"
    git add --all
    git commit --quiet --message "chore(release): prepare for $tag"
    git push --quiet --set-upstream origin "$branch"
    echo "▶ open the pull request"
    notes="$(mktemp)"
    trap 'rm -f "$notes"' EXIT
    git-cliff --unreleased --tag "$tag" --strip all > "$notes"
    gh pr create --base main --head "$branch" \
        --title "chore(release): prepare for $tag" --body-file "$notes"
    echo "✅ merge that PR, then run: just release-publish $version"

# Step 2, once the release PR is merged: tag main and create the GitHub release. Usage: just release-publish 1.2.0
[confirm("This creates a public tag and GitHub release. Continue?")]
[group('release')]
release-publish VERSION:
    #!/usr/bin/env bash
    set -euo pipefail
    version="{{ VERSION }}"
    tag="v$version"
    fail() { echo "✗ $1" >&2; exit 1; }

    gh auth status >/dev/null || fail "gh is not logged in: run \`gh auth login\`"
    git fetch --quiet --tags origin main
    ! git rev-parse --verify --quiet "refs/tags/$tag" >/dev/null || fail "tag $tag already exists"
    # The release commit must be on main. A squash merge adds ` (#N)` to it.
    head="$(git rev-parse origin/main)"
    subjects="$(git log --format=%s "$head" | sed 's/ (#[0-9]*)$//')"
    grep -qxF "chore(release): prepare for $tag" <<<"$subjects" \
        || fail "main has no release commit for $tag: merge the release PR first"

    # A version with a pre-release part (1.2.0-rc.1) is marked as such.
    flags=()
    case "$version" in *-*) flags+=(--prerelease) ;; esac
    last="$(git describe --tags --abbrev=0 "$head" 2>/dev/null || true)"
    notes="$(mktemp)"
    trap 'rm -f "$notes"' EXIT
    echo "▶ release $tag from main ($head)"
    # Notes first, so a git-cliff failure stops before anything is published.
    # The range ends at the SHA: git-cliff asks the GitHub API about it, and
    # the API knows no `origin/main`.
    git-cliff ${last:+"$last..$head"} --tag "$tag" --strip all > "$notes"
    gh release create "$tag" --target "$head" \
        --title "$tag" --notes-file "$notes" "${flags[@]}"
    # {if:rust.common}
    echo "✅ $tag is out; cd.yml attaches the binaries: gh run watch"
    # {endif:rust.common}
    # {ifnot:rust.common}
    echo "✅ $tag is out"
    # {endif:rust.common}
# {endif:just}
