# Lesson 5 — Number types (and a famous overflow surprise)

## 1. Why it exists

Lessons 1–4 were about *naming* values. Now we look at the **kinds** of value
themselves. Rust pins down one exact kind per value (you watched it refuse to let a
number turn into text). The most common kinds are numbers — and Rust deliberately
has *more than one* number type, because the choice decides how large a value can
get and what happens when it gets too big.

## 2. The idea

Four everyday "scalar" (single-value) kinds:

- **Whole numbers (integers).** The default is `i32` — *signed* (can go negative),
  32 bits wide. `u32` is *unsigned* (zero or positive only). The number after the
  `i`/`u` is the width — how large it can get. A `u8`, for example, holds only
  `0` to `255`.
- **Decimals (floats).** Default `f64`. Written with a dot: `2.5`.
- **True / false (`bool`).** Exactly `true` or `false`.
- **A single character (`char`).** One letter or symbol in **single quotes**:
  `'R'`. (Text in *double* quotes — `"R"` — is the different kind, `&str`, from
  Lesson 2.)

You can let Rust infer the kind, or spell it after the name with `:` (Lesson 1).
Underscores just make big numbers readable: `3_000_000`.

## 3. A tiny example to read

```rust
fn main() {
    let whole: i32 = -42;
    let big: u32 = 3_000_000;
    let ratio: f64 = 2.5;
    let yes: bool = true;
    let letter: char = 'R';
    println!("{whole}, {big}, {ratio}, {yes}, {letter}");
}
```

**Predict the line, then check:**

```
-42, 3000000, 2.5, true, R
```

The underscores in `3_000_000` are gone when it prints — they're only for *your*
eyes. And `letter` is in single quotes because a `char` is one character.

## 4. Common pitfalls / real compiler errors — overflow

The width matters. A `u8` stops at `255`. Watch what happens when we push past it:

```rust
fn main() {
    let mut small: u8 = 255;
    println!("small = {small}");
    small = small + 1;
    println!("small = {small}");
}
```

**Before you scroll — does this compile? Does it run? What happens?**

It compiles fine. Then, running the normal (debug) build, it prints the first line
and **stops**:

```
small = 255

thread 'main' (…) panicked at b.rs:4:13:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

*(The number in parentheses is a process id — it'll differ on your run.)*

This is **not** a compile error — it's a *runtime* one. Rust didn't let `small`
silently become a wrong number; it halted the program at the exact moment of
overflow and pointed at `small + 1`: "attempt to add with overflow." The fix is a
roomier type (`u32`).

> One thing to file away: this guard is on in the normal `cargo run` (debug) build.
> An optimized `cargo run --release` build *wraps* `255 + 1` around to `0` instead
> of panicking — which is exactly why catching the mistake early, in debug, matters.

## 5. Predict-then-run practice (your turn — write this yourself)

`cargo new numbers`. Predict before each run:

1. Make one binding of **each** kind — an integer, a float, a `bool`, and a `char`
   — and print them on one line. Predict the line first.
2. Make a `u8` set near its limit, then add enough to push it over `255`. Predict:
   will it compile? will it run? what happens? Then run it with plain `cargo run`.
3. Change that `u8` to `u32` and run again. Predict the difference first.

*(All yours to type. Predictions are the answer key.)*

## 6. What surprised you?

Did you expect overflow to *crash* rather than quietly wrap? Did picking a type
width feel fussy or useful? Tell me — Lesson 6 is the last foundations idea before
functions: what the **semicolon** really does.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.2 "Data Types" (scalar types;
  integer overflow panicking in debug, wrapping in release).
- **CR** — *Comprehensive Rust* (Google), "Types and Values." Tiny-example style.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Data types." Cited for contrast.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 4 — Constants (`const`)](04-constants.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 6 — Expressions, statements & the semicolon →](06-expressions-statements-semicolon.md)
