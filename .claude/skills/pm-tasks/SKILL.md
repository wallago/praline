---
name: pm-tasks
description: Read and write task files in this repo's format. Use when
  touching any file under the pm folder, todo.md, or *.todo.md.
---

## States

`- [ ]` open · `- [.]` in progress · `- [x]` done · `- [c]` cancelled ·
`- [/]` on hold

## Metadata

`@due`, `@target`, `@id` — @id values are generated, never invented or
reused. Preserve existing ones verbatim on any edit.

## Structure

Sections in order: Active, Backlog, Notes, Archive. Completed items move
to Archive; they are not deleted. Callouts use `> [!TIP]` style.

Never reformat a file you were only asked to add an item to.
