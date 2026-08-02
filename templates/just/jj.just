# ── Release ───────────────────────────────────────────────────
# Cut & publish a release: bump version, changelog, commit, tag, push. Usage: just publish 0.2.0
[confirm("This will tag and push a release to origin. Continue?")]
[group('release')]
jj-publish VERSION:
    @test -z "$(jj diff --name-only)" || (echo "✗ working copy has uncommitted changes — commit or abandon them first" && exit 1)
    @echo "▶ bump version → {{ VERSION }}"
    cargo set-version {{ VERSION }}
    @echo "▶ regenerate changelog"
    @if command -v git-cliff >/dev/null 2>&1; then \
        git-cliff --tag v{{ VERSION }} -o CHANGELOG.md; \
    else \
        echo "⊘ git-cliff not installed — skipping changelog"; \
    fi
    @echo "▶ record release commit"
    jj commit -m "chore(release): v{{ VERSION }}"
    @echo "▶ advance main bookmark"
    jj bookmark set main -r @-
    @echo "▶ tag release commit"
    git tag -a v{{ VERSION }} -m "v{{ VERSION }}" "$(jj log -r @- --no-graph -T commit_id)"
    @echo "▶ push bookmark + tag"
    jj git push --bookmark main
    git push origin v{{ VERSION }}
    @echo "✅ published v{{ VERSION }}"
