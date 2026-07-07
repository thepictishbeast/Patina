# Lesson 2 — Mutability (`mut`)

## 1. Why it exists

Lesson 1 showed that a name is **locked by default** — try to re-point it and the
compiler stops you (that was `E0384`). But plenty of real values are *meant* to
change: a score climbs, a counter ticks, a total grows. `mut` is how you ask Rust
for a name you're allowed to change — out loud, so anyone reading the code knows
"this one moves."

## 2. The idea

`mut` (short for **mutable**, meaning *changeable*) goes right after `let`:

```
let mut score = 0;
```

That's the whole syntax — one word. It unlocks the name, so a later line like
`score = 10;` is *allowed* instead of being the `E0384` error from Lesson 1.

One thing to plant now — you'll feel it in part 4: `mut` unlocks the **value**, not
the **kind of value**. A name that started as a whole number stays a whole-number
name; `mut` just lets *which* whole number it holds change.

## 3. A tiny example to read

`mut` lets the value move from `0` to `10`:

```rust
fn main() {
    let mut score = 0;
    println!("Score: {score}");
    score = 10;
    println!("Score: {score}");
}
```

**Before you scroll — what two lines does this print?**

```
Score: 0
Score: 10
```

The first `println!` runs while `score` is `0`; then `score = 10` re-points the
name; the second `println!` sees `10`. (Compare Lesson 1: without `mut`, that
`score = 10` line wouldn't compile at all.)

## 4. Common pitfalls / real compiler errors

Here's the one to feel. We make `count` mutable, then try to change it to *text*:

```rust
fn main() {
    let mut count = 5;
    println!("Count: {count}");
    count = "five";
    println!("Count: {count}");
}
```

**Before you scroll — will this compile? If not, which error code?**

It won't. Real output from `rustc` (1.95.0), unedited:

```
error[E0308]: mismatched types
 --> b.rs:4:13
  |
2 |     let mut count = 5;
  |                     - expected due to this value
3 |     println!("Count: {count}");
4 |     count = "five";
  |             ^^^^^^ expected integer, found `&str`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Read it slowly:

- `error[E0308]` — a **new** code: "mismatched types" (Lesson 1's was `E0384`).
- It points at the original `5` as *"expected due to this value"* — the kind was
  fixed the moment you wrote `count = 5`.
- `^^^^^^ expected integer, found &str` — `"five"` is text (`&str`), not a number.
  (Read `&str` simply as "a piece of text" for now — text gets its full lesson in
  Lesson 12.)

So `mut` let the **value** change, but the **kind** (whole number) was locked at
the first assignment. Changing the *kind* is a different move — and it's exactly
what Lesson 3 is about.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, `cargo new mutability` or a playground works too.)* Write each piece,
and **predict before every run**:

1. Make a `mut` name for something that *counts* — your choice (not `score`, not
   `count`). Print it, change it to a new number, print it again. Predict the two
   lines first.
2. Now write a binding **without** `mut` and try to re-point it on the next line.
   Before running: which **error code** appears? (You met it in Lesson 1.) Run and
   check.
3. Fix #2 the way the compiler suggests. Predict the output, then run.

*(You write every line — I won't. Your predictions are the answer key.)*

## 6. What surprised you?

One or two sentences: did you expect `mut` to also let you switch a number to text?
Did the `E0308` message point where you thought it would? Tell me, and I'll pitch
Lesson 3 to match where you are.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.1 "Variables and Mutability" (the
  `mut` opt-out; mutability keeps the type fixed).
- **CR** — *Comprehensive Rust* (Google), "Values / mutability." Cited for contrast.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Mutable variables." Cited for
  contrast.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 1 — Bindings & Immutability](01-bindings-and-immutability.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 3 — Shadowing →](03-shadowing.md)
