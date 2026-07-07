# Lesson 19c — `match` patterns in depth

*(Phase 5, part 4. Lesson 11 introduced `match` and deliberately kept the patterns
simple. Now that you have enums and `Option`, here's the vocabulary it was saving.)*

## 1. Why it exists

So far your `match` arms mostly named a value or a variant. Patterns can do much
more: pull data *out*, cover several values in one arm, test ranges, add
conditions — while the compiler still checks you've covered everything. This is
the lesson where `match` becomes the tool you reach for first.

## 2. The idea — the pattern vocabulary

- **Bind** the inner data: `Some(x) => …`, `Message::Move { x, y } => …`.
- **Multiple patterns** with `|`: `1 | 2 | 3 => …`.
- **Ranges** with `..=`: `4..=9 => …`.
- **Guards** — an extra condition on an arm: `Some(x) if x < 0 => …`.
- A catch-all that **binds** (`other => …`) vs `_` that **ignores**. The catch-all
  goes **last** — arms are tried top to bottom.
- And always: `match` must be **exhaustive** — miss a case and it won't compile
  (the `E0004` you met in Lesson 19b).

## 3. A tiny example to read (you type this one — the write-rep)

**The full vocabulary on an `Option`.** Predict each line:

```rust
fn describe(n: Option<i32>) -> String {
    match n {
        Some(0) => String::from("zero"),
        Some(x) if x < 0 => format!("negative: {x}"),   // guard
        Some(1 | 2 | 3) => String::from("small"),         // multiple patterns
        Some(4..=9) => String::from("medium"),            // range
        Some(x) => format!("big: {x}"),                   // bind the rest
        None => String::from("nothing"),
    }
}
fn main() {
    for n in [Some(0), Some(-5), Some(2), Some(7), Some(100), None] {
        println!("{}", describe(n));
    }
}
```

```
zero
negative: -5
small
medium
big: 100
nothing
```

## 4. Common pitfalls — order matters

Arms are tried **top to bottom**, so a catch-all placed *before* a specific arm
swallows everything — the compiler warns that the later arm is **unreachable**.
Try it: move `Some(x) => …` above `Some(0) => …` in the example and read the
warning (`unreachable pattern`). It compiles — a *warning*, not an error — which
makes it sneakier than the walls you've hit so far: the program runs, but an arm
you wrote can never fire. Read warnings too.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new patterns` works too.)* **Predict on paper before each
run.**

1. **One of each.** Write a `match` over `Option<i32>` that uses — at least once
   each — a guard, a `|` multi-pattern, a `..=` range, and a binding catch-all.
   Feed it values that hit every arm. **Predict** every line first.
2. **Make an arm unreachable** on purpose (catch-all first). **Predict**: error or
   warning? Does it still run?

*(You write every line here — I won't. The predictions are your answer key. Next:
the concise one-case forms — `if let`, `while let`, `let…else`.)*

## 6. What surprised you?

A sentence or two: which pattern form do you expect to use most? Did
"unreachable is only a warning" surprise you? Tell me, and I'll pitch Lesson 19d
to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §6.2 (binding inner data, catch-all
  vs `_`, exhaustiveness).
- **CR** — *Comprehensive Rust* (Google), §12.2 — the pattern vocabulary (`|`,
  `..=`, guards).
- Compiler output captured live on **rustc 1.95.0** (edition 2024). The still
  deeper pattern features (`@` bindings, refutability as a concept) come in
  Lesson 33.

---

<!-- lesson-nav -->
[← Lesson 19b — Option: Rust has no null](19b-option.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 19d — Concise matching: if let, while let, let…else →](19d-concise-matching.md)
