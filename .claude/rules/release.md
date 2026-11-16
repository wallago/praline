# Release

`CHANGELOG.md` is generated from git history by `git-cliff`. Never hand-edit it
— fix the commit messages and regenerate.

A release is two commands, with a PR between them:

1. `just release 0.3.0`, from an up-to-date `main`: creates `release/v0.3.0`,
   bumps `Cargo.toml` and `Cargo.lock`, regenerates `CHANGELOG.md`, pushes, and
   opens the PR with `gh`.
2. Once that PR is merged, `just release-publish 0.3.0`: creates the GitHub
   release and its `v0.3.0` tag on `main`. The tag starts `cd.yml`, which
   builds the binaries, attaches them, and publishes to crates.io (not for a
   pre-release version).

Both push to the remote, so they are the user's to run: print the command.
<!-- {if:committed} -->

Commit subjects feed the changelog, so the type prefix decides which section a
change lands in. `chore:` is not a free pass for a user-visible change.

<!-- {endif:committed} -->
