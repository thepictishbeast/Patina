# Lesson 37 — Documenting Rust: doc comments & doc-tests

*(Phase 9 — tooling, part 2. Lesson 36 made your CODE prove itself with
tests. This lesson makes your DOCUMENTATION prove itself too — because in
Rust, the examples in your docs are compiled and run. Docs that lie get
caught.)*

## 1. Why it exists

Documentation rots. The code changes, the comment doesn't, and six months
later the example in the docs quietly returns the wrong number — and every
reader who trusts it inherits the bug. Rust's answer is structural: **doc
comments are Markdown that renders into a real manual, and the code examples
inside them are tests.** `cargo test` runs them. Your docs can't drift from
your code without the build telling you.

## 2. The idea

- **`///` documents the item *below* it** — a function, struct, module.
- **`//!` documents the *enclosing* item** — put it at the top of a file to
  document the module or the whole crate.
- Both take **Markdown**: headings, backticks, lists. `cargo doc --open`
  builds an HTML manual from them and opens it in a browser — your crate gets
  the same style of reference pages the standard library has.
- **The neat trick:** a fenced code block inside a doc comment is a
  **doc-test**. `cargo test` compiles it *and runs it*, exactly like
  Lesson 36's tests. An `# Examples` section isn't decoration — it's part of
  your test suite.

## 3. A tiny example to read

**A documented function.** The `///` block is Markdown, and its example is a
doc-test:

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

This compiles as a library; `cargo doc --open` renders it into a browsable
page; and `cargo test` runs the `assert_eq!` inside the comment:

```
   Doc-tests adder

running 1 test
test src/lib.rs - add (line 7) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Read that middle line: the *test's name* is a place in your documentation —
`src/lib.rs - add (line 7)`. The example in the comment just ran.

## 4. Common pitfalls / real failures — the example that lies

Change the doc example to claim `add(2, 3)` is `6` (the code still says
`a + b`) and run `cargo test`:

```
   Doc-tests adder

running 1 test
test src/lib.rs - add (line 7) ... FAILED

failures:

---- src/lib.rs - add (line 7) stdout ----
Test executable failed (exit status: 101).

stderr:

thread 'main' panicked at doctest_bundle_2024.rs:6:1:
assertion `left == right` failed
  left: 5
 right: 6

failures:
    src/lib.rs - add (line 7)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Read the verdict: `left: 5, right: 6` — the *code* said 5, the *docs* claimed
6, and the docs lost. This is Lesson 36's assertion failure wearing a new
uniform: your documentation is now part of what must stay correct. That's the
feature working — examples that lie get caught before a reader ever trusts
them.

(One environment note: doc-tests need `cargo`, so this lesson's practice is a
project exercise — the Sandbox compiles single files and doesn't run
doc-tests.)

## 5. Predict-then-run practice (your turn — write this yourself)

On your own machine or in Termux: `cargo new docs_demo --lib` (or open any
crate you've made). **Predict before each command.**

1. **Document and view.** Put a `///` doc comment with a `# Examples` code
   block on a public function (part 3 shows the shape — write your own
   function and example). Run `cargo doc --open`. **Predict** what the page
   shows before you look.
2. **Prove it's a test.** Run `cargo test`. **Predict** the doc-test's *name*
   in the output before you read it — what file, which item, roughly which
   line?
3. **Make it lie, watch it get caught.** Change the expected value in your
   doc example to something wrong. **Predict**: does `cargo test` pass — and
   which two numbers will the failure report as `left` and `right`? Fix it
   and watch the suite go green again.

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next — the last lesson: shipping. Profiles, publishing,
and workspaces: the ten-minute tour of everything between "it works" and
"it's out there.")*

## 6. What surprised you?

A sentence or two: did "your documentation examples are *tested*" change how
you think about writing docs? Does the failing doc-test feel like the same
muscle as Lesson 36's failing assert, or something new? Tell me, and I'll
shape the wrap-up to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§14.2** (the doc-comment
  half): `///`, `//!`, `# Examples`, `cargo doc --open`, and doc-tests.
- **CR** — *Comprehensive Rust* (Google): the documentation and doc-tests
  slides.
- Both `cargo test` outputs captured live on **rustc/cargo 1.95.0**, edition
  2024, from a real `cargo new adder --lib` project (the run-specific merged
  doctest bundle path and thread id are normalized; edition-2024 doctests
  compile merged, hence the `doctest_bundle_2024.rs` filename in the panic).

---

<!-- lesson-nav -->
[← Lesson 36 — Automated Tests](36-automated-tests.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 37b — Shipping with Cargo: profiles, publishing & workspaces →](37b-shipping-with-cargo.md)
