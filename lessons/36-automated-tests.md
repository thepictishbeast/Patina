# Lesson 36 — Automated Tests

*(Phase 9 — tooling. The compiler proves your code is *type-correct* and memory-safe, but it
can't know whether `add` should return `4` or `5` — that's *logic*, and logic is what tests
pin down. A test is a small piece of code that runs your code and checks the answer, so a
change that quietly breaks something gets caught the moment you run the tests.)*

## 1. Why it exists

Everything before this lesson made the compiler your ally for *correctness of form*. But a
function can compile perfectly and still do the wrong thing. Automated tests close that gap:
you write down what the code *should* do as runnable checks, and `cargo test` runs them all
on demand. Now refactoring is safe — if you break a behaviour, a test goes red and names it.

> **How the sources frame it:** the **BOOK** Ch.11 is the backbone — it builds the mechanics
> from zero (`#[test]`, the auto-generated `#[cfg(test)] mod tests`, the assertion macros, and
> `#[should_panic]`). **CR** §28 reinforces unit vs integration tests. No metaphor needed.

## 2. The idea

- **`#[test]`** above a function marks it as a test. `cargo test` builds a special test runner
  that calls every `#[test]` function and reports which passed.
- **A test passes if it doesn't panic.** So you check things with macros that panic on failure:
  - `assert!(condition)` — panics if `condition` is `false`.
  - `assert_eq!(a, b)` / `assert_ne!(a, b)` — panic if the two values are (not) equal, **and
    print both values** so you can see what went wrong. Prefer these when comparing.
  - Add a custom message: `assert!(ok, "expected ok for input {x}")`.
- **`#[should_panic]`** marks a test that's supposed to panic — it passes *when* the code
  panics (use `#[should_panic(expected = "substring")]` to also check the message).
- **`#[cfg(test)] mod tests { ... }`** holds your unit tests. `#[cfg(test)]` means this module
  is compiled **only** when you run `cargo test` — it adds nothing to a normal release build.
  Inside it, `use super::*;` brings the code-under-test into scope.

## 3. Tiny examples to read

**Two passing tests.** The `#[cfg(test)]` module sits next to the code it checks:

```rust
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn add_is_commutative() {
        assert_eq!(add(3, 5), add(5, 3));
    }
}
```

Running it (`cargo test`, or `rustc --test file.rs` then run the binary) prints:

```
running 2 tests
test tests::add_is_commutative ... ok
test tests::adds_two_numbers ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

**Asserting that something *panics*.** Some functions are *supposed* to reject bad input:

```rust
fn checked(n: i32) -> i32 {
    if n < 0 {
        panic!("n must be non-negative");
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn rejects_negative() {
        checked(-1);
    }
}
```

```
running 1 test
test tests::rejects_negative - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

It passes *because* `checked(-1)` panicked — that was the point.

## 4. Common pitfalls / real compiler errors

The "error" you'll see most is not a *compiler* error — it's a **failing test**, and reading
it is the whole skill. Make a test claim something false:

```rust
fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two_is_five() {
        assert_eq!(add(2, 2), 5);
    }
}
```

```
running 1 test
test tests::two_plus_two_is_five ... FAILED

failures:

---- tests::two_plus_two_is_five stdout ----

thread 'tests::two_plus_two_is_five' panicked at main.rs:6:33:
assertion `left == right` failed
  left: 4
 right: 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::two_plus_two_is_five

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

This is exactly why `assert_eq!` beats `assert!(add(2,2) == 5)`: it prints **`left: 4`** and
**`right: 5`**, so you instantly see your code produced `4` while the test expected `5`. The
runner names the failing test (`two_plus_two_is_five`) and the line — read it top to bottom.

## 5. Predict-then-run practice (your turn — write this yourself)

Open `cargo new tested --lib` (or any file you run with `rustc --test`). **Predict before you
run.**

1. **A passing pair.** Write a function `fn double(n: i32) -> i32`. Add a `#[cfg(test)] mod
   tests` with `use super::*;` and two `#[test]` functions using `assert_eq!` — one for `0`,
   one for a positive number. **Predict** the `test result:` line, then run.

2. **Force a failure and read it.** Change *one* expected value to something wrong. **Predict**
   which test fails and what the `left:` / `right:` lines will show, then run and compare. Put
   it back.

3. **`should_panic`.** Write a function that `panic!`s on an empty input, and a `#[test]
   #[should_panic]` that calls it with empty input. **Predict** whether it passes. Then make it
   `#[should_panic(expected = "...")]` with the wrong substring and **predict** what changes.

*(You write every line here — I won't. Tests are how you make change safe: once a behaviour has
a test, you'll know the instant anything breaks it.)*

## 6. What surprised you?

A sentence or two: did "a test passes if it doesn't panic" reframe how you think about
checking code? Did the `left:` / `right:` print make a failing test feel readable rather than
scary? Tell me, and I'll tune the wrap-up.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, Ch.11 "Writing Automated Tests": §11.1 (the
  `#[test]` attribute, the `#[cfg(test)] mod tests`, `assert!`/`assert_eq!`/`assert_ne!`, custom
  messages, `#[should_panic]`).
- **CR** — *Comprehensive Rust* (Google), §28 (unit tests; integration and doc tests are a
  later step).
- Every snippet compiled and **run with the test harness** (`rustc --test`, equivalent to
  `cargo test`) on **rustc 1.95.0**, edition 2024; the failing-test output is captured verbatim.

---

<!-- lesson-nav -->
[← Lesson 35 — Capstone: A Multithreaded Web Server](35-capstone-web-server.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 37 — More About Cargo & crates.io →](37-more-about-cargo.md)
