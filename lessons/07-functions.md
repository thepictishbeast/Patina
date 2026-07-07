# Lesson 7 — Functions

## 1. Why it exists

So far everything has lived inside `main`. Functions let you give a chunk of work
its own name, hand it inputs, and get a value back — so you write it once and reuse
it, and each piece stays small enough to hold in your head. You've actually used a
function every single lesson: `main` is one.

## 2. The idea

A function is declared with `fn`:

```
fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

- `fn`, then the **name** in `snake_case` (lowercase words joined by underscores).
- **Parameters** go in `( )`, each written `name: type`. The type is **required**
  here — unlike `let`, Rust will not infer a parameter's type.
- `-> u32` is the **return type**: the kind of value handed back. Leave it off and
  the function returns `()` (nothing).
- The body's **last expression with no semicolon** is the value returned — exactly
  the Lesson 6 rule. (`return x;` also works, but last-expression is the normal
  Rust style.)

## 3. A tiny example to read

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    let a = area(3, 4);
    println!("area is {a}");
}
```

**Predict the line, then check:**

```
area is 12
```

`area(3, 4)` runs the body `width * height` → `12`. There's no semicolon on that
line, so it's the function's value, which comes back into `a`.

## 4. Common pitfalls / real compiler errors — wrong argument type

Parameter types are a promise the **caller** has to keep. Pass a decimal where a
whole number is wanted:

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    let a = area(3.0, 4);
    println!("area is {a}");
}
```

**Before you scroll — will this compile?**

No. Real output from `rustc` (1.95.0), unedited:

```
error[E0308]: mismatched types
 --> b.rs:6:18
  |
6 |     let a = area(3.0, 4);
  |             ---- ^^^ expected `u32`, found floating-point number
  |             |
  |             arguments to this function are incorrect
  |
note: function defined here
 --> b.rs:1:4
  |
1 | fn area(width: u32, height: u32) -> u32 {
  |    ^^^^ ----------
```

It's `E0308` ("mismatched types") again, in yet another setting: `3.0` is a float,
but `width` was declared `u32`. The compiler says "arguments to this function are
incorrect" and even shows you **where `area` is defined** so you can check the
promise. The fix: pass `3`, not `3.0`.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the two matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, `cargo new functions` works too.)* Predict before each run:

1. Write a function that takes two numbers and returns one — your choice (a sum, a
   larger-of-two, anything). Call it from `main` and print the result. Predict the
   line first. Remember: **no** semicolon on the line you want returned.
2. Add a semicolon to that returned line. Predict the **error code** before running
   (you met it in Lesson 6).
3. Call your function with a decimal (`3.0`) where it wants a whole number. Predict
   the error code, then run and check.

*(Every line is yours. Predictions are your answer key.)*

## 6. What surprised you?

Did "the last line is the return value" feel natural, or do you still want to write
`return`? Did the wrong-type error point where you expected? Tell me — Lesson 8
finishes the foundations with comments and the full story on printing.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.3 "Functions" (parameters, return
  types, return-by-last-expression).
- **CR** — *Comprehensive Rust* (Google), "Functions." Cited for contrast.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Functions." Cited for contrast.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 6 — Expressions, statements & the semicolon](06-expressions-statements-semicolon.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 8 — Comments & printing →](08-comments-and-printing.md)
