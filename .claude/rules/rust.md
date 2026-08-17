# Rust

Edition and MSRV are pinned in `Cargo.toml` and enforced in CI. Don't reach for
a feature newer than the MSRV to save three lines.

<!-- TODO — the lints this crate denies beyond the defaults (`unwrap_used`,
     `expect_used`, `panic`, `missing_docs_in_private_items`, …) and what each
     one means for everyday code. Copy them from `Cargo.toml` once, here. -->

<!-- {if:clippy} -->

## Lints

CI runs clippy with `-D warnings`. A warning is a failed build, not a hint.
Fix the cause; `#[allow]` needs a comment saying why.

<!-- {endif:clippy} -->

<!-- {if:codecov} -->

## Tests

Tests run under `cargo nextest`, coverage under `cargo llvm-cov`. Write the
failing test first — "add validation" means "test the invalid input, then make
it pass".

<!-- {endif:codecov} -->

## Dependencies

<!-- {if:deny} -->

- A new crate has to clear `cargo deny check` — licence, bans, sources,
  advisories — before it lands.
  <!-- {endif:deny} -->
  <!-- {if:audit} -->
- `cargo audit` scans the RustSec DB in CI.
  <!-- {endif:audit} -->
  <!-- {if:machete} -->
- `cargo machete` fails on unused dependencies; drop them in the same change
that orphans them.
<!-- {endif:machete} -->

Adding a dependency is a decision, not a detail. Say what it buys and what it
costs before pulling it in.

## APIs

Don't write a call from memory. Check it — `cargo doc --open`, the source in
`~/.cargo/registry`, or context7 — before using anything you haven't used in
this repo already. Version drift is the normal case, not the exception.

If you're unsure and can't check, say the call is unverified rather than
letting the compiler find out.

## Lockfiles Are Their Own Change

`Cargo.lock` is not a file you edit. It changes as a side effect of a
dependency change, and that makes it a separate commit from the code — never
bundled into a feature diff.
