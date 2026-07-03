# Lesson 37 — More About Cargo & crates.io

*(Phase 9 — tooling, and the last lesson. You've used `cargo new`, `cargo build`, `cargo run`,
and `cargo test` throughout. This lesson rounds out the everyday Cargo you'll actually reach
for: tuning the optimized build, documenting your code so others (and future-you) can read it,
sharing a crate on crates.io, and grouping related crates into a workspace. Most of what's
here is **configuration and commands**, not new Rust syntax.)*

## 1. Why it exists

A language is only as pleasant as its tooling, and Cargo is the part of Rust you touch every
day. Once your code works, the next questions are practical: how do I ship a *fast* build?
how do I write docs that stay correct? how do I publish something for others to depend on?
how do I manage several crates that grow up together? Cargo answers all four with a little
configuration — no extra tools to install.

> **How the sources frame it:** **BOOK** Ch.14 is the backbone (release profiles, doc comments
> + crates.io publishing, workspaces, `cargo install`). **CR** touches docs/tests. This is a
> tour — skim it now, and come back when you actually need to publish or split a project.

## 2. The idea

- **Release profiles** — Cargo builds with *profiles*. `cargo build` uses the **dev** profile
  (fast to compile, unoptimized, easy to debug); `cargo build --release` uses the **release**
  profile (slow to compile, optimized, fast to run). You tune them in `Cargo.toml` under
  `[profile.dev]` / `[profile.release]` (e.g. `opt-level` from `0` to `3`).
- **Documentation comments** — `///` documents the item *below* it; `//!` documents the
  *enclosing* item (a module or the whole crate). They take **Markdown**, and `cargo doc --open`
  builds and opens an HTML manual from them. The neat trick: a code block in a doc comment is a
  **doc-test** — `cargo test` compiles and runs it, so your examples can't silently rot.
- **Publishing to crates.io** — a crate becomes shareable with `cargo publish`. crates.io
  requires some metadata in `[package]` first (a `description` and a `license`), and a publish
  is **permanent** (you can `yank` a version but not delete it). Others then depend on it by
  name + version.
- **Workspaces** — when one project grows into several related crates, a **workspace** ties
  them together: a top-level `Cargo.toml` with `[workspace]` listing the `members`. They share
  one `Cargo.lock` and one `target/` directory, and `cargo build` at the top builds them all.
  (Tempered Studio itself is a workspace of a dozen crates.)

## 3. Tiny examples to read

**A documented function.** The `///` block is Markdown, and its example is a doc-test:

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

This compiles as a library, `cargo doc --open` renders it into a browsable page, and
`cargo test` runs the `assert_eq!` in the doc comment as a doc-test.

**A release profile** (this is `Cargo.toml`, not Rust):

```toml
[profile.release]
opt-level = 3      # full optimization (the default for --release)
```

**A workspace** (top-level `Cargo.toml`):

```toml
[workspace]
members = ["adder", "calculator"]
```

**Metadata you need before publishing** (`Cargo.toml`):

```toml
[package]
name = "adder"
version = "0.1.0"
edition = "2024"
description = "A tiny crate that adds numbers."
license = "MIT OR Apache-2.0"
```

## 4. Common pitfalls (these surface at *build/publish* time, not from `rustc`)

Unlike every earlier lesson, the snags here don't come from the compiler rejecting your
syntax — your code can be perfect and these still bite:

- **A wrong doc example breaks `cargo test`.** Because the ` ``` ` block in a `///` comment is
  a real doc-test, if you write `assert_eq!(add(2, 3), 6)` in the docs, `cargo test` runs it and
  reports a **doctest failure** — your *documentation* is now part of what must stay correct.
  That's the feature working: examples that lie get caught.
- **`cargo publish` is rejected without metadata.** Try to publish a crate whose `[package]`
  has no `description` or `license` and crates.io refuses it — those fields are required, and a
  published version can't be unpublished (only `yank`ed). Fill the metadata in *before* you
  publish.
- **`opt-level = 0` shipped by accident.** If you run a benchmark with `cargo run` (dev profile)
  instead of `cargo run --release`, your "slow" code may just be the unoptimized build. Reach
  for `--release` whenever speed is what you're measuring.

(There's no `rustc` error code to show here — that's the point of the lesson. These are Cargo
and crates.io concerns, and Cargo's messages name the missing field or the failing doc-test.)

## 5. Predict-then-run practice (your turn — write this yourself)

Open a crate you've made (or `cargo new docs_demo --lib`). **Predict before each command.**

1. **Document and view.** Put a `///` doc comment with a `# Examples` code block on a public
   function. Run `cargo doc --open`. **Predict** what the page shows, then look.

2. **Make the doc-test catch you.** Run `cargo test` and confirm your doc example passes. Now
   change the expected value in the doc example to something wrong. **Predict** whether
   `cargo test` passes, then run it and read the failure.

3. **A release build.** Run `cargo build` then `cargo build --release`. **Predict** where each
   binary lands under `target/` (look for `target/debug/` vs `target/release/`), then check.

*(You write every line here — I won't. That's the whole toolkit: you can now write Rust,
organize it, test it, document it, and ship it. This is the end of the lesson series — go
build something.)*

## 6. What surprised you?

A sentence or two: did "your documentation examples are *tested*" change how you think about
writing docs? Did it surprise you that the optimized build is a different *profile* rather than
a flag deep in the compiler? Tell me, and I'll shape the wrap-up review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, Ch.14 "More About Cargo and Crates.io": release
  profiles (§14.1), publishing a crate with doc comments + metadata (§14.2), workspaces (§14.3),
  installing binaries with `cargo install` (§14.4).
- **CR** — *Comprehensive Rust* (Google): documentation and doc-tests.
- The documented-library snippet compiles on **rustc 1.95.0**, edition 2024; the `Cargo.toml`
  excerpts are configuration (not compiled), and the publish/profile behaviours are Cargo/
  crates.io tooling described as commands.

**The path ends here — the habit doesn't.** You can now read what the
compiler says and act on it, which is the only skill the rest of Rust asks
for. The Study Guide's *When you finish the path* section has your next
moves — all offline, all already on this device.

---

<!-- lesson-nav -->
[← Lesson 36 — Automated Tests](36-automated-tests.md) · [↑ Study Guide](../STUDY-GUIDE.md)
