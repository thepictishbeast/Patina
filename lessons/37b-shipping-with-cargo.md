# Lesson 37b — Shipping with Cargo: profiles, publishing & workspaces

*(Phase 9 — tooling, part 3 — and the last lesson of the course. Your code
works, tests pass, docs are honest. What's left is everything between "it
works" and "it's out there": a fast build, a shareable crate, a project that
grew into several. This is a ten-minute tour — skim it now, return when you
actually ship.)*

## 1. Why it exists

A language is only as pleasant as its tooling, and Cargo is the part of Rust
you touch every day. Three practical questions remain: how do I ship a *fast*
build? how do I publish something for others to depend on? how do I manage
several crates that grow up together? Cargo answers all three with a little
configuration — no extra tools to install.

## 2. The idea

- **Release profiles.** `cargo build` uses the **dev** profile (fast to
  compile, unoptimized, easy to debug); `cargo build --release` uses the
  **release** profile (slow to compile, optimized, fast to run). You tune
  them in `Cargo.toml` under `[profile.dev]` / `[profile.release]` (e.g.
  `opt-level` from `0` to `3`).
- **Publishing to crates.io.** A crate becomes shareable with
  `cargo publish`. crates.io requires metadata in `[package]` first (a
  `description` and a `license`), and a publish is **permanent** — you can
  `yank` a version (stop new projects from using it) but never delete it.
  The full checklist and yank semantics are **Book §14.2**; read it the day
  you actually publish.
- **Workspaces.** When one project grows into several related crates, a
  **workspace** ties them together: a top-level `Cargo.toml` with
  `[workspace]` listing the `members`, sharing one `Cargo.lock` and one
  `target/`. (Tempered Studio itself is a workspace of a dozen crates.) The
  member-layout details are **Book §14.3**.
- **`cargo install`** rounds out the kit: it builds and installs a published
  *binary* crate onto your machine — how Rust tools distribute themselves
  (Book §14.4).

## 3. Tiny examples to read — three pieces of `Cargo.toml`

**A release profile** (configuration, not Rust):

```toml
[profile.release]
opt-level = 3      # full optimization (the default for --release)
```

**Metadata you need before publishing:**

```toml
[package]
name = "adder"
version = "0.1.0"
edition = "2024"
description = "A tiny crate that adds numbers."
license = "MIT OR Apache-2.0"
```

**A workspace** (top-level `Cargo.toml`):

```toml
[workspace]
members = ["adder", "calculator"]
```

## 4. Common pitfalls (these surface at *build/publish* time, not from `rustc`)

Unlike every earlier lesson, the snags here don't come from the compiler
rejecting your syntax — your code can be perfect and these still bite:

- **`opt-level = 0` benchmarked by accident.** If you time your program with
  `cargo run` (dev profile) instead of `cargo run --release`, your "slow"
  code may just be the unoptimized build. Reach for `--release` whenever
  speed is what you're measuring.
- **`cargo publish` rejected without metadata.** No `description` or
  `license` in `[package]` and crates.io refuses the publish — and since a
  published version can't be unpublished (only yanked), fill the metadata in
  *before* you push the button.

(There's no `rustc` error code to show here — that's the point of the
lesson. These are Cargo and crates.io concerns, and Cargo's messages name the
missing field.)

## 5. Predict-then-run practice (your turn — write this yourself)

On your own machine or in Termux, in any cargo project. **Predict before each
command.**

1. **Two builds.** Run `cargo build`, then `cargo build --release`.
   **Predict** where each binary lands under `target/` before you look
   (`target/debug/` vs `target/release/`) — and which took longer to
   compile.
2. **The publish gate, on paper.** Open your project's `Cargo.toml` and
   compare its `[package]` against part 3's metadata block. **Predict**:
   which fields would crates.io still demand before `cargo publish` would
   accept it? (Don't publish — just read.)
3. **Spot a workspace in the wild.** This app's own source is a workspace.
   In part 2's terms, say what its top-level `Cargo.toml` must contain, and
   why all its crates share one `target/` — then, if you have the repo,
   check.

*(You write every line here — I won't. That's the whole toolkit: you can now
write Rust, organize it, test it, document it, and ship it.)*

**The path ends here — the habit doesn't.** You can now read what the
compiler says and act on it, which is the only skill the rest of Rust asks
for. The Study Guide's *When you finish the path* section has your next
moves — all offline, all already on this device.

## 6. What surprised you?

A sentence or two: did it surprise you that the optimized build is a
different *profile* rather than a flag deep in the compiler? Did "publish is
permanent, yank is the only undo" change how you'd version things? Tell me —
it shapes the wrap-up review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, Ch.14 "More About Cargo and
  Crates.io": release profiles (§14.1), publishing metadata + yank (§14.2),
  workspaces (§14.3), `cargo install` (§14.4) — the last three pointed at
  rather than recited, per the tour framing.
- The `Cargo.toml` excerpts are configuration (not compiled); the
  publish/profile behaviours are Cargo/crates.io tooling described as
  commands.

---

<!-- lesson-nav -->
[← Lesson 37 — Documenting Rust: doc comments & doc-tests](37-doc-comments.md) · [↑ Study Guide](../STUDY-GUIDE.md)
