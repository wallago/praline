# Propose before writing

**Do not call Edit or Write on any file in this repo until the change has been
shown and approved.** This covers everything — `src/`, `templates/`, configs,
workflows, `CLAUDE.md`, and this file.

## The flow

1. Print the change as a unified diff in a ```diff code block, one block per
   file, headed by the file path. Show enough surrounding context to read it —
   a bare `-`/`+` pair with no context is not reviewable.
2. Stop and wait. Do not chain the edit onto the same turn.
3. On "ok" / "go" / "apply", write the files exactly as shown. If the applied
   version has to differ from the diff, say so — do not silently drift.

New files are shown as a full listing in a syntax-highlighted block rather than
an all-`+` diff.

## What this does not block

Read-only work needs no approval; go straight to it. That means `Read`, `Grep`,
`Glob`, `git` queries, and the whole `just` gate (`just check`, `just lint`,
`just ci`) — including after a change lands, to verify it.

## Notes

The user may apply the patch by hand instead, or come back with edits to it.
Both are normal: treat "I did it" as the go-ahead to move on to verification
rather than re-editing the file.

Batch the diffs for one logical change into a single message so the whole thing
can be reviewed at once, rather than one file per turn.
