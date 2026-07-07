# Lesson 6 — Expressions, statements & the semicolon

## 1. Why it exists

The semicolon is the single most common thing Rust beginners trip on. Once you see
what it actually *does*, a whole family of confusing errors turns obvious. This is
also the bridge to Lesson 7 (functions), which lean on this idea completely.

## 2. The idea

Rust lines come in two flavours:

- A **statement** *does* something and produces **no value**. `let x = 5;` is a
  statement.
- An **expression** *evaluates to* a value. `5`, `x + 1`, and even a whole
  `{ ... }` block are expressions.

The rule that ties it together:

> An expression with **no semicolon** *is* a value. Add a semicolon and it becomes
> a statement that **throws the value away** — producing the "nothing" value,
> written `()` and called **unit**.

And one more: a `{ }` block is *itself* an expression. Its value is its **last
line, when that line has no semicolon**.

## 3. A tiny example to read

A block used as a value:

```rust
fn main() {
    let y = {
        let a = 3;
        a + 1
    };
    println!("y is {y}");
}
```

**Predict the line, then check:**

```
y is 4
```

The block runs `let a = 3;` (a statement), then its last line `a + 1` has **no
semicolon**, so that's the block's value — `4` — and `y` becomes `4`. (Put a
semicolon after `a + 1` and `y` would be `()` — nothing.)

## 4. Common pitfalls / real compiler errors — the stray semicolon

Here's the classic. A function that promises to give back an `i32`, but whose last
line ends in a semicolon. (Functions get their full lesson **next**, in Lesson 7 —
for now read `fn plus_one(x: i32) -> i32` as "a recipe named `plus_one` that takes
a whole number and promises a whole number back," and `println!("{}", …)` as "print
whatever fills the `{}`.")

```rust
fn plus_one(x: i32) -> i32 {
    x + 1;
}

fn main() {
    println!("{}", plus_one(5));
}
```

**Before you scroll — will this compile?**

No. Real output from `rustc` (1.95.0), unedited:

```
error[E0308]: mismatched types
 --> b.rs:1:24
  |
1 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     x + 1;
  |          - help: remove this semicolon to return this value

error: aborting due to 1 previous error
```

Read it slowly — it's the same `E0308` ("mismatched types") from Lesson 2, used a
new way:

- The function header promises `-> i32`, but the body "implicitly returns `()`."
- Why? The `;` after `x + 1` turned the value into a thrown-away statement, so the
  function hands back `()` (nothing) instead of the number.
- The compiler points at the **exact semicolon**: "remove this semicolon to return
  this value." That one character *is* the whole bug.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine,
`cargo new semicolons` works too.)* Predict before each run:

1. Write `let n = { ... };` where the block does a little arithmetic and its last
   line has **no** semicolon. Predict the printed value, then run.
2. Now add a semicolon to that last line. Predict what `n` becomes before running.
   (What's the "nothing" value called?)
3. Write a tiny function `-> i32` and deliberately end its body with `value;`
   (with a semicolon). Predict the **error code** and *which character* the
   compiler will blame — then run and check.

*(Every line is yours. Predictions are your answer key.)*

## 6. What surprised you?

Did "a block is an expression" land, or feel strange? Did the compiler blame the
character you expected? Tell me — you're now ready for Lesson 7, where this exact
rule powers how functions return values.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.3 "Functions → Statements and
  Expressions" and §3.5 (block-as-value).
- **CR** — *Comprehensive Rust* (Google), "Blocks and expressions." Cited for
  contrast.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre). Cited for contrast.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 5b — Integer overflow: your first runtime panic](05b-integer-overflow.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 7 — Functions →](07-functions.md)
