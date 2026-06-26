# Tooling Quiz — Tests & Cargo

A self-check for the **Tooling** lessons (Lessons 36–37: writing automated tests with
`#[test]`, the assertion macros, `#[should_panic]`, the `#[cfg(test)] mod tests` module and a
real failing-test output; then everyday Cargo — release profiles, doc comments + doc-tests,
publishing metadata, and workspaces). Same rule as before: **predict each answer before** you
look at the **Answers** section. Don't run the code first; predict, then verify. Fourteen
questions.

> Tip: cover the Answers section until you've committed to an answer for every question.
> The **test** snippets here are compiled and **run** with the test harness (`rustc --test`,
> the same machinery `cargo test` uses), so their `test result:` lines and `left:`/`right:`
> values are captured live. The **Cargo** parts — release profiles, publishing, workspaces —
> are *configuration and commands* (`Cargo.toml`), **shown, not compiled**; the one Cargo
> snippet that is real Rust is the `///`-documented library, which compiles. So a Cargo
> question asks *"is this true?"*, never *"what does this `Cargo.toml` print?"*

---

## Questions

**Q1 — concept.** A test function is marked with `#[test]`. In one line: **what single thing
makes a `#[test]` function count as *passed*** when the harness runs it?

**Q2 — predict the output.** (Runs under the test harness.)
```rust
fn double(n: i32) -> i32 { n * 2 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_zero() {
        assert_eq!(double(0), 0);
    }

    #[test]
    fn doubles_four() {
        assert_eq!(double(4), 8);
    }
}
```
Give the final **`test result:`** line (the counts).

**Q3 — predict the output.** This test claims something false. When it fails, `assert_eq!`
prints two labelled lines. **What are the `left:` and `right:` values?**
```rust
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_plus_four() {
        assert_eq!(add(3, 4), 8);
    }
}
```

**Q4 — predict the output.** This `assert_ne!` fails. **What two labelled lines does it
print, with what values?**
```rust
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_is_not_ten() {
        assert_ne!(add(4, 6), 10);
    }
}
```

**Q5 — predict the output.** The condition is `false`, so this test fails. **What message text
does the failure print?**
```rust
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_message() {
        let result = add(2, 2);
        assert!(result == 5, "expected 5 but got {result}");
    }
}
```

**Q6 — does this test pass or fail?** (Runs under the harness.)
```rust
fn parse_positive(n: i32) -> i32 {
    if n <= 0 {
        panic!("must be positive");
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn rejects_zero() {
        parse_positive(0);
    }
}
```

**Q7 — does this test pass or fail?** The function does **not** panic.
```rust
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn this_should_panic_but_wont() {
        let _ = add(2, 2);
    }
}
```

**Q8 — does this test pass or fail?** The code panics, but read the `expected` substring
carefully.
```rust
fn parse_positive(n: i32) -> i32 {
    if n <= 0 {
        panic!("must be positive");
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "must be negative")]
    fn rejects_zero() {
        parse_positive(0);
    }
}
```

**Q9 — does this compile? If not, what does the compiler say?**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn checks(value: i32) {
        assert_eq!(value, value);
    }
}
```

**Q10 — concept.** `#[cfg(test)]` sits above the `mod tests` block. When you build your crate
the normal way with `cargo build --release`, **is the code inside that module compiled into
the release binary?** One line: yes or no, and why.

**Q11 — does this compile?** (Compiled as a library — no `main`.) The `///` and `//!` are
documentation comments.
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

**Q12 — concept.** Two questions about doc comments: (a) `///` documents the item in which
*direction* relative to the comment, and `//!` documents what? (b) The ` ``` ` code block in a
`///` comment isn't just shown in the rendered docs — `cargo test` does **what** with it?

**Q13 — concept / true-or-false.** crates.io and `cargo publish`. (a) Name **two** fields the
`[package]` table must have before crates.io will accept `cargo publish`. (b) True or false: a
version you've published can later be **deleted** from crates.io.

**Q14 — concept.** A `Cargo.toml` with `[profile.release]` sets `opt-level`. (a) What are the
**numeric levels** `opt-level` uses, and which is the dev default vs the `--release` default?
(b) For `[workspace] members = ["adder", "calculator"]`, name **one** thing the member crates
**share**.

---

## Answers

*(Verified on rustc 1.95.0, edition 2024. Every test snippet was compiled and run with the
test harness — `rustc --edition 2024 --test` — and the `test result:` / `left:`/`right:`
lines captured live. The documented library compiles as a `--crate-type lib`. The Cargo.toml
and publishing facts are configuration/tooling, shown and described, not compiled.)*

**A1 — a `#[test]` function passes if it *doesn't panic*.** The harness calls each `#[test]`
function; one that returns normally counts as passed, and one that panics counts as failed.
That's why you check things with macros that *panic on failure* (`assert!`, `assert_eq!`,
`assert_ne!`). (Lesson 36.)

**A2 — `test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished
in 0.00s`.** Both asserts hold (`double(0) == 0`, `double(4) == 8`), so both tests pass. (The
per-test lines `test tests::doubles_four ... ok` etc. may appear in either order — the summary
counts don't.) (Lesson 36.)

**A3 — `left: 7` and `right: 8`.** `add(3, 4)` produces `7`, which the test compared against
`8`; `assert_eq!` prints the value it *got* on the **left** and the value you *expected* on
the **right** — `left: 7`, `right: 8` — under the header `assertion `left == right` failed`.
Seeing both numbers is exactly why `assert_eq!` beats `assert!(add(3,4) == 8)`. (Lesson 36.)

**A4 — `left: 10` and `right: 10`**, under the header `assertion `left != right` failed`.
`assert_ne!` fails *because the two sides were equal* — `add(4, 6)` is `10` and you asserted it
was **not** `10`. So it prints both values (both `10`) to show you they collided. (Lesson 36.)

**A5 — it prints just your custom message, `expected 5 but got 4`** — *not* a `left:`/`right:`
pair. A bare `assert!(condition, "msg")` only knows the condition was `false`; it has no two
values to compare, so it prints the message you supplied (with `{result}` filled in). That's
the trade-off: `assert!` lets you write a sentence, `assert_eq!`/`assert_ne!` show the values.
(Lesson 36.)

**A6 — it passes.** The test is marked `#[should_panic]`, and `parse_positive(0)` *does* panic
(`0 <= 0`), so the panic is exactly what the test wanted. The harness reports
`test tests::rejects_zero - should panic ... ok`. (Lesson 36.)

**A7 — it fails.** `#[should_panic]` demands a panic, but `add(2, 2)` returns `4` quietly and
never panics. The harness reports `FAILED` with `note: test did not panic as expected`. A
`should_panic` test fails when the code *doesn't* panic. (Lesson 36.)

**A8 — it fails.** The code panics with `"must be positive"`, but `#[should_panic(expected =
"must be negative")]` requires the panic message to *contain* `"must be negative"`. It doesn't,
so the harness reports `FAILED` with `note: panic did not contain expected string`, printing
`panic message: "must be positive"` and `expected substring: "must be negative"`. The `expected
=` string must be a substring of the real panic message. (Lesson 36.)

**A9 — No, it does not compile.** The compiler errors with `functions used as tests can not
have any arguments` (no `error[E0###]` code — it's a dedicated test-harness diagnostic). A
`#[test]` function is called by the harness with no arguments, so it must take none; drop the
`value: i32` parameter. (Lesson 36.)

**A10 — No, it is not in the release binary.** `#[cfg(test)]` is conditional compilation: the
module is compiled **only** when you build for tests (`cargo test`). A normal `cargo build` or
`cargo build --release` skips it entirely, so your tests add nothing to the shipped binary.
(Lesson 36.)

**A11 — Yes, it compiles.** `//!` documents the enclosing item (here, the crate) and `///`
documents the `add` function below it — both are just comments to the compiler, so the library
builds fine. (The example *inside* the `///` block is a doc-test that `cargo test` would run,
but the code as a library compiles regardless.) (Lesson 37.)

**A12 — (a) `///` documents the item *below* it; `//!` documents the *enclosing* item** (the
module or whole crate it sits inside). **(b) `cargo test` compiles and runs that code block as
a doc-test** — so an example that lies (e.g. `assert_eq!(add(2, 3), 6)`) makes `cargo test`
fail. Your documentation examples are checked, so they can't silently rot. (Lesson 37.)

**A13 — (a) `description` and `license`** (both in `[package]`) are required before crates.io
accepts a publish. **(b) False** — a published version is **permanent**; you can `yank` it (so
new projects won't pick it up) but you cannot delete it. Fill the metadata in *before* you
publish. (Lesson 37.)

**A14 — (a) the numeric levels are `0`, `1`, `2`, `3`** — `0` = no optimization (the dev
default), `3` = full optimization (the `--release` default). **(b)** Workspace members share
**one
`Cargo.lock`** and **one `target/` directory** (either is a correct answer); a top-level `cargo
build` builds all members together. (Lesson 37.)

---

*How did you do?* Anything you missed points at the lesson to reread. You can now write a
`#[cfg(test)] mod tests`, choose `assert_eq!`/`assert_ne!` over `assert!` to see the values,
read a failing test top-to-bottom (`left:`/`right:`, the `should panic` notes), and reach for
the everyday Cargo — tune `[profile.release]`, document with `///` so the examples are tested,
fill `description` + `license` before publishing, and group crates in a `[workspace]`. That's
the toolkit; go build something.

— *Sources:* questions written for this corpus from Lessons 36–37 (BOOK Ch.11 "Writing
Automated Tests" & Ch.14 "More About Cargo and Crates.io"; CR §28 unit tests, docs/doc-tests).
Every test snippet was compiled and run with the harness, and the failing-test text captured
live, on **rustc 1.95.0**, edition 2024; the documented library compiles as a library, while
the `Cargo.toml`, profile, publishing, and workspace facts are configuration and tooling —
shown and described, not compiled.
