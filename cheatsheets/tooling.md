# Tooling Cheatsheet — Tests & Cargo

Quick reference (pairs with the Tooling lessons — L36 automated tests · L37 more about Cargo &
crates.io). The shape of it: a **test** is code that runs your code and checks the answer — it
**passes if it doesn't panic**, so you check with macros that *panic on failure*; the
`#[cfg(test)] mod tests` block holds them and ships in **no** release build. Then **Cargo** is
the everyday tooling: **release profiles** tune the optimized build, **doc comments** (`///`,
`//!`) build an HTML manual *and* get tested, **publishing** to crates.io needs metadata and is
permanent, and a **workspace** groups related crates. Verified on rustc 1.95.0, edition 2024.
*(The compiler proves your code is type-correct; tests pin down whether it's **right**.)*

## Tests — `#[test]` + the harness
- **`#[test]`** above a `fn` marks it a test. `cargo test` (or `rustc --test file.rs` then run the binary) builds a runner that calls every `#[test]` function and reports which passed.
- **A test passes if it doesn't panic.** Return normally → `... ok`; panic → `... FAILED`. So you check things with macros that panic on failure.
- The summary line is the scoreboard: `test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`. Per-test lines (`test tests::name ... ok`) may print in **any order**; the counts don't change.

## Assertion macros — how a failure reads
- **`assert!(cond)`** — panics if `cond` is `false`. With a custom message `assert!(cond, "expected 5 but got {x}")`, a failure prints **just that message** — it has no two values to show.
- **`assert_eq!(a, b)`** — panics if `a != b`, header `assertion `left == right` failed`, then prints both: **`left:`** = what it got (first arg), **`right:`** = what you expected (second arg).
- **`assert_ne!(a, b)`** — panics if `a == b`, header `assertion `left != right` failed`, and *still prints both values* (they collided — that's why it failed).
- **Why `assert_eq!`/`assert_ne!` beat `assert!(a == b)`:** they print the actual `left:`/`right:` values, so a failing test names the gap (`left: 7` / `right: 8`) instead of just "false."
- Reading a failure: the runner names the failing test (`---- tests::name stdout ----`), the `panicked at <file>:<line>:<col>`, the `left:`/`right:` lines, then the `failures:` list and the `FAILED` count. Read it top to bottom.

## `#[should_panic]` — testing that code rejects bad input
- **`#[test] #[should_panic]`** — the test **passes when the body panics** (prints `... - should panic ... ok`); if the code *doesn't* panic it **FAILs** with `note: test did not panic as expected`.
- **`#[should_panic(expected = "substring")]`** — also checks the panic message **contains** that substring. Wrong substring → **FAIL** with `note: panic did not contain expected string`, printing the real `panic message:` and the `expected substring:`.

## The test module — `#[cfg(test)] mod tests`
- **`#[cfg(test)] mod tests { use super::*; … }`** is the conventional home for unit tests, sitting next to the code it checks. `use super::*;` pulls the code-under-test into scope.
- **`#[cfg(test)]` = conditional compilation:** the module is compiled **only** under `cargo test`. A normal `cargo build` / `cargo build --release` **skips it** — tests add nothing to the shipped binary.
- A **`#[test]` function takes no arguments** — the harness calls it with none. Give it a parameter and the build fails: `functions used as tests can not have any arguments` (a harness diagnostic, no `E0###` code).

## Cargo — release profiles
- Cargo builds with **profiles**. `cargo build` → **dev** profile (fast compile, unoptimized, easy to debug). `cargo build --release` → **release** profile (slow compile, optimized, fast to run).
- Tune them in `Cargo.toml` under `[profile.dev]` / `[profile.release]`. **`opt-level`** uses the numeric levels `0`, `1`, `2`, `3` (`0` = dev default, no optimization; `3` = release default, full):
  ```toml
  [profile.release]
  opt-level = 3      # full optimization (the --release default)
  ```
- **Pitfall:** benchmarking with `cargo run` (dev, `opt-level = 0`) measures the *unoptimized* build. Use `--release` whenever speed is what you're measuring.

## Cargo — documentation comments & doc-tests
- **`///`** documents the item **below** it; **`//!`** documents the **enclosing** item (a module, or the whole crate from its top). Both take **Markdown**.
- **`cargo doc --open`** builds and opens a browsable HTML manual from those comments.
- **A code block in a `///` comment is a doc-test:** `cargo test` *compiles and runs it*. So an example that lies (`assert_eq!(add(2, 3), 6)`) is caught as a doctest failure — your docs can't silently rot. The documented library itself compiles like any other Rust:
  ```rust
  //! A tiny crate that adds numbers.

  /// Adds two numbers and returns the sum.
  ///
  /// # Examples
  ///
  /// ```
  /// assert_eq!(adder::add(2, 3), 5);
  /// ```
  pub fn add(a: i32, b: i32) -> i32 {
      a + b
  }
  ```

## Cargo — publishing to crates.io
- `cargo publish` shares a crate. crates.io **requires metadata first** — at minimum a **`description`** and a **`license`** in `[package]`:
  ```toml
  [package]
  name = "adder"
  version = "0.1.0"
  edition = "2024"
  description = "A tiny crate that adds numbers."
  license = "MIT OR Apache-2.0"
  ```
- **A publish is permanent.** You can **`yank`** a version (new projects won't select it) but you **cannot delete** it. Fill the metadata in *before* you publish.

## Cargo — workspaces
- A **workspace** groups related crates: a top-level `Cargo.toml` with `[workspace]` listing the `members`:
  ```toml
  [workspace]
  members = ["adder", "calculator"]
  ```
- Members **share one `Cargo.lock` and one `target/` directory**; a top-level `cargo build` builds them all. (Tempered Studio itself is a workspace of a dozen crates.)

— *Sources:* BOOK Ch.11 "Writing Automated Tests" (§11.1) & Ch.14 "More About Cargo and
Crates.io" (§14.1 profiles, §14.2 publishing + doc comments, §14.3 workspaces) · CR §28 unit
tests + docs/doc-tests. Test snippets verified on rustc 1.95.0, edition 2024 — compiled **and
run** with the harness, failing-test text captured live; the documented library compiles as a
library, while profiles, publishing, and workspace facts are `Cargo.toml` configuration and
tooling — shown and described, not compiled.
