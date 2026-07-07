# Lesson 18c — Printing your own types: `#[derive(Debug)]`

*(Phase 5, part 3. Your struct works — now make it printable, and let the compiler
teach you how.)*

## 1. Why it exists

The moment you build your own type you'll want to *see* one. But `println!("{p}")`
and even `println!("{p:?}")` refuse to compile for your struct — Rust never guesses
how a type should look. This shortest lesson of the phase is about the one line
that fixes it, and about the compiler literally printing that line for you.

## 2. The idea

Your own types can't be printed with `{}` or `{:?}` until you say so. Adding
`#[derive(Debug)]` above the struct makes `{:?}` (compact) and `{:#?}` (pretty,
one field per line) work. That's it — one attribute, both formats.

(There's also `dbg!(x)`, a debugging macro that prints a value and hands it right
back — the Book shows it in §5.2 when you want it.)

## 3. A tiny example to read

```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }
fn main() {
    let r = Rectangle { width: 30, height: 50 };
    println!("{r:?}");    // compact
    println!("{r:#?}");   // pretty
}
```

```
Rectangle { width: 30, height: 50 }
Rectangle {
    width: 30,
    height: 50,
}
```

## 4. Common pitfalls / real compiler errors — let the compiler teach you

**Debug-printing without deriving it — `E0277`.** Drop the `#[derive(Debug)]` and:

```rust
struct Rectangle { width: u32, height: u32 }
fn main() {
    let r = Rectangle { width: 30, height: 50 };
    println!("{r:?}");
}
```

```
error[E0277]: `Rectangle` doesn't implement `Debug`
 --> main.rs:4:15
  |
4 |     println!("{r:?}");
  |               ^^^^^ `Rectangle` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  …
help: consider annotating `Rectangle` with `#[derive(Debug)]`
  |
1 + #[derive(Debug)]
2 | struct Rectangle { width: u32, height: u32 }
```

The fix is literally printed for you: add `#[derive(Debug)]` above the struct.
This fail-then-fix is worth doing once on purpose — it's the Book's own teaching
move, and it's how you'll discover most derives from now on.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new printing_types` works too.)* **Predict on paper before
each run.**

1. **Derive, then break it.** Add `#[derive(Debug)]` to your `Book` and print it
   with `{:#?}`. Then *remove* the derive and print with `{:?}` — **predict** the
   error code and the exact line the compiler will tell you to add.
2. **Compact vs pretty.** With the derive back, print the same instance with
   `{:?}` and `{:#?}`. **Predict** the difference before running.

*(You write every line here — I won't. The predictions are your answer key. Next
lesson: enums and `match` in depth — the other half of modelling your data.)*

## 6. What surprised you?

A sentence or two: was the compiler handing you the exact `#[derive(Debug)]` line
a surprise? Tell me, and I'll pitch Lesson 19 (enums + matching) to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §5.2 "An Example Program Using
  Structs" — the `#[derive(Debug)]` "let the compiler teach you" fail-then-fix,
  `{:?}` vs `{:#?}`, and `dbg!`.
- **CR** — *Comprehensive Rust* (Google), §13.1/§13.3 — "why derive" (the macro as
  shorthand for a hand-written impl).
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 18b — Methods & impl blocks](18b-methods-and-impl.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 19 — Enums: one-of types →](19-enums.md)
