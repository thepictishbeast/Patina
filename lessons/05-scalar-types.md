# Lesson 5 — The scalar types: integers, floats, `bool` & `char`

## 1. Why it exists

Lessons 1–4 were about *naming* values. Now we look at the **kinds** of value
themselves. Rust pins down one exact kind per value (you watched it refuse to let a
number turn into text). This lesson is the map of the four everyday "scalar"
(single-value) kinds — including the fact that Rust deliberately has *more than
one* number type.

## 2. The idea

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

## 4. Common pitfalls / real compiler errors — no silent conversions

Each integer type is its own kind, and Rust **never converts between them
silently** — not even from a bigger number type to a smaller one, and not even
when the value would fit:

```rust
fn main() {
    let big: i64 = 10;
    let small: i32 = big;
    println!("{small}");
}
```

**Before you scroll — does this compile?**

No. The two kinds don't match, and Rust says so with the same error code you met
in Lesson 2 — `error[E0308]` mismatched types, "expected `i32`, found `i64`".
Moving a value between number types is something you must write *explicitly*, in
the code — the matching exercise below has you meet this wall and read the
compiler's own suggestion for the fix.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, `cargo new numbers` works too.)* Predict before each run:

1. Make one binding of **each** kind — an integer, a float, a `bool`, and a `char`
   — and print them on one line. Predict the line first.
2. Bind an `i64`, then try to hand it to a fresh `i32` binding. Predict the
   **error code** before running (you've seen it before).

*(All yours to type. Predictions are the answer key.)*

## 6. What surprised you?

Did you expect one whole-number type instead of a family of widths? Did the
refuse-to-convert rule feel strict or reassuring? Tell me — next, in Lesson 5b,
you'll see *why* the width you pick matters: what happens the moment a value
outgrows its type.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.2 "Data Types" (scalar types;
  explicit conversion between integer types).
- **CR** — *Comprehensive Rust* (Google), "Types and Values." Tiny-example style.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Data types." Cited for contrast.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 4 — Constants (`const`)](04-constants.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 5b — Integer overflow: your first runtime panic →](05b-integer-overflow.md)
