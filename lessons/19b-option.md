# Lesson 19b — `Option<T>`: Rust has no null

*(Phase 5, part 3. One particular enum is so central it's built into the language's
whole way of thinking: the type of "a value that might be missing.")*

## 1. Why it exists

In many systems, any value might secretly be "null" — absent — and forgetting to
check is a classic crash. Rust has **no null**. Instead, "might be missing" is its
own *type*: `Option<T>`, an ordinary enum with two variants — `Some(value)` or
`None`. Because missing-ness lives in the type, the compiler can *force* you to
handle it. You can choose to panic on `None`; you can't accidentally forget it.

> Read `Option<T>` as "an Option holding some type." The `T` is a **placeholder**
> for whatever type the value has — `Option<i32>`, `Option<String>`. You'll write
> your own placeholders like this in Lesson 24 (generics); for now, just read it.

## 2. The idea

- `Some(5)` is "there's a value, and it's 5." `None` is "there's nothing here."
- A bare `None` needs a type annotation (`let n: Option<i32> = None;`) — the
  compiler can't guess what the `Some` *would have* held.
- **An `Option<T>` is not a `T`.** You can't use a maybe-number as a number; you
  must get the value out first — with `match` (below), or the concise forms of
  Lesson 19d — and that's exactly where the safety comes from.

(The story of how null got called "the billion-dollar mistake" — by its own
inventor — is in the Book, §6.1.)

## 3. A tiny example to read

Match on an `Option` — both cases, exhaustively:

```rust
fn describe(n: Option<i32>) -> String {
    match n {
        Some(x) => format!("got {x}"),
        None => String::from("nothing"),
    }
}
fn main() {
    println!("{}", describe(Some(7)));
    println!("{}", describe(None));
}
```

```
got 7
nothing
```

## 4. Common pitfalls / real compiler errors

**Forgetting the `None` arm — `E0004`.** A `match` on an `Option` must handle both
cases:

```rust
fn plus_one(n: Option<i32>) -> i32 {
    match n {
        Some(x) => x + 1,
    }
}
```

```
error[E0004]: non-exhaustive patterns: `None` not covered
 --> main.rs:2:11
  |
2 |     match n {
  |           ^ pattern `None` not covered
  …
  = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a
      wildcard pattern or an explicit pattern as shown
```

The compiler names the case you missed. That's the whole point of `Option`: the
"what if it's missing?" question can't be skipped.

**Using an `Option<T>` as if it were a `T` — `E0277`.** You can't add a `Some` to
a number:

```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
}
```

```
error[E0277]: cannot add `Option<i8>` to `i8`
 --> main.rs:4:17
  |
4 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
```

`y` *might* be `None`, so Rust won't let you treat it as a plain number. The
matching exercise below hits the same wall from a different angle (a mismatched-
types flavour) — **predict its code** before you run.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new options` works too.)* **Predict on paper
before each run.**

1. **Match an `Option` exhaustively.** Write a function taking `Option<i32>` that
   returns a `String`: handle `Some` (bind the value) and `None`. **Predict** what
   each returns. Then *delete* the `None` arm — **predict** the error code, then
   put it back.
2. **Use it as a `T`.** Try adding an `Option<i32>` straight to an `i32`.
   **Predict** whether it compiles and what the compiler suggests.

*(You write every line here — I won't. The predictions are your answer key. Next:
the full pattern vocabulary `match` was saving for you.)*

## 6. What surprised you?

A sentence or two: does "missing-ness is a type" feel heavier or lighter than
null-checking by discipline? Tell me, and I'll pitch Lesson 19c to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §6.1 (`Option`, the no-null design,
  the "billion-dollar mistake" story), §6.2 (the `None`-not-covered `E0004`).
- **CR** — *Comprehensive Rust* (Google): "you can choose to panic on `None`, but
  you can't accidentally forget to check."
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 19 — Enums: one-of types](19-enums.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 19c — match patterns in depth →](19c-match-in-depth.md)
