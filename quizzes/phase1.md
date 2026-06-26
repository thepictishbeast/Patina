# Phase 1 Quiz — Foundations

A self-check for Phase 1 (Lessons 1–8: bindings & immutability, mutability, shadowing,
constants, number types & overflow, casting, expressions vs statements, functions, and
printing). Same rule as the other quizzes: **predict each answer before** you look at the
**Answers** section. Don't run the code first; predict, then verify. Thirteen questions.

> Tip: cover the Answers section until you've committed to an answer for every question.

---

## Questions

**Q1 — does this compile? If not, what's the error code?**
```rust
let x = 5;
x = 6;
println!("{x}");
```

**Q2 — predict the output.**
```rust
let mut x = 5;
x = 6;
println!("{x}");
```

**Q3 — predict the output.**
```rust
let s = "   ";          // three spaces
let s = s.len();
println!("{s}");
```

**Q4 — does this compile? If not, what's the error code?**
```rust
let mut s = "   ";
s = s.len();
```

**Q5 — concept.** A `const` requires two things that a `let` binding does not. What are
they? (Think about what you must *write*, and the naming convention.)

**Q6 — predict the output.**
```rust
let n = -5 / 3;
println!("{n}");
```

**Q7 — predict the output.**
```rust
let x = 3.7_f64 as i32;
println!("{x}");
```

**Q8 — concept.** In a function like `fn add(a: u8, b: u8) -> u8 { a + b }`, you call
`add(255, 1)`. What happens **in a debug build** versus **in a release build**?

**Q9 — predict the output.**
```rust
let y = {
    let a = 3;
    a + 1
};
println!("{y}");
```

**Q10 — does this compile? If not, what's the error code, and what's the one-character fix?**
```rust
fn five() -> i32 {
    5;
}
```

**Q11 — predict the output.**
```rust
fn add(a: i32, b: i32) -> i32 { a + b }
fn main() {
    println!("{}", add(2, 3));
}
```

**Q12 — predict the output.**
```rust
let v = (1, 2);
println!("{v:?}");
```

**Q13 — fill in the blanks (concept).** (a) Bindings are `____` by default. (b) Casting a
float to an integer with `as` `____` (rounds / truncates). (c) The default integer type is
`____` and the default float type is `____`.

---

## Answers

*(Verified on rustc 1.95.0, edition 2024.)*

**A1 — No: `error[E0384]`** ("cannot assign twice to immutable variable `x`"). A `let`
binding is immutable by default; the compiler even suggests `help: consider making this
binding mutable`. (Lessons 1–2.)

**A2 — `6`.** `mut` opts the binding in to being changed, so `x = 6` is allowed (same type).
(Lesson 2.)

**A3 — `3`.** Re-`let` **shadows** the old `s` with a new binding, and shadowing may change
the type — here from `&str` to `usize` (the length of the three-space string). (Lesson 3.)

**A4 — No: `error[E0308]`** ("mismatched types"). `mut` lets you change the *value* but not
the *type*: `s` is a `&str`, and `s.len()` is a `usize`. (Only shadowing can change type.)
(Lesson 3.)

**A5 — A written type annotation and SCREAMING_SNAKE_CASE naming.** `const MAX_POINTS: u32 =
100_000;` — the type is required (it can't be inferred), the value must be a compile-time
constant, and the convention is all-caps with underscores. (Lesson 4.)

**A6 — `-1`.** Integer division **truncates toward zero**: `-5 / 3` is `-1` (not `-2`).
(Lesson 5.)

**A7 — `3`.** `as` from a float to an integer **truncates** (chops the fractional part) — it
never rounds, so `3.7` becomes `3`. (Lesson 5.)

**A8 — Debug build: it *panics* at runtime** (`thread 'main' panicked … attempt to add with
overflow`). **Release build: it silently *wraps around*** (two's complement, so `255 + 1`
becomes `0`). Overflow is never undefined behavior in Rust — it's one or the other. (Lesson 5.)

**A9 — `4`.** A block is an **expression**: its value is the last line *without* a semicolon
(`a + 1`), so `y` is `4`. (Lesson 6.)

**A10 — No: `error[E0308]`** ("expected `i32`, found `()`"). The trailing `;` turns `5` into
a statement, so the function returns `()` instead of `i32`. The fix is to **remove the
semicolon** — the compiler literally says `help: remove this semicolon to return this value`.
(Lessons 6–7.)

**A11 — `5`.** A function's last expression (no `;`) is its return value, so `add(2, 3)` is
`2 + 3`. (Lesson 7.)

**A12 — `(1, 2)`.** `{:?}` is the **Debug** format; a tuple prints with its parentheses and
elements. (Plain `{}` / Display wouldn't work on a tuple.) (Lesson 8.)

**A13 — (a) immutable; (b) truncates; (c) `i32` and `f64`.** (Lessons 1, 5.)

---

*How did you do?* Anything you missed points at the lesson to reread. These foundations —
immutable-by-default, shadowing vs `mut`, expressions-vs-statements, and reading the error
code by hand — are what everything else builds on. Next: Phase 2, control flow.

— *Sources:* questions written for this corpus from Lessons 1–8 (BOOK §3.1–3.4, CR §5–6,
BLOG variables/types); every code snippet compiled (and the runnable ones run) on **rustc
1.95.0**, edition 2024.
