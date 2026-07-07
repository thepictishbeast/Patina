# Lesson 9 — `if` / `else if` / `else`: making the program choose

*(Phase 2 — Control flow begins here. Phase 1 built the pieces; now you start
making the program **choose**. This lesson is the fork in the road itself;
Lesson 9b adds Rust's twist — the fork produces a value.)*

## 1. Why it exists

A program that always does the same thing isn't much use. The moment you want it
to do one thing *when* something is true and another thing otherwise, you need a
way to branch. `if` is that fork in the road: hand it a yes/no question, and it
runs one block of code or another.

## 2. The idea

The shape is a question, then a block to run if the answer is yes:

```
if temperature > 25 {
    // runs only when the question is true
}
```

`temperature > 25` is the **condition**. It has to be a real yes/no value — a
`bool`, which is either `true` or `false`. Rust is strict here: there is **no
"truthiness."** A number is not a stand-in for "yes," an empty string is not a
stand-in for "no." If you want a `bool`, you write something that produces one — a
comparison like `>`, `==`, `!=` does exactly that.

Add `else` for the "otherwise" path, and chain `else if` to ask further questions
in order — **the first one that's `true` wins**, and the rest are skipped:

```
if temperature > 25 {
    // hot
} else if temperature > 10 {
    // mild
} else {
    // cold
}
```

Two things to hold onto from that shape:

- **Order matters.** The questions are asked top to bottom. If two conditions
  would both be true, the one written *first* claims the run.
- **`else` is the catch-all.** It has no question — it runs only when every
  question above it answered `false`.

## 3. A tiny example to read

**Branching with `else if`** — read it, then predict its output:

```rust
fn main() {
    let temperature = 18;
    if temperature > 25 {
        println!("warm");
    } else if temperature > 10 {
        println!("mild");
    } else {
        println!("cold");
    }
}
```

`18 > 25` is `false`, so the first block is skipped; `18 > 10` is `true`, so the
second one wins and the rest are skipped. It prints:

```
mild
```

## 4. Common pitfalls / real compiler errors

**Handing `if` something that isn't a `bool`.** It's tempting to think a number
like `3` should count as "yes." Rust doesn't work that way — it wants a real
`bool`:

```rust
fn main() {
    let number = 3;
    if number {
        println!("nonzero");
    }
}
```

**Before you scroll — will this compile?** It won't. Real `rustc` (1.95.0),
unedited:

```
error[E0308]: mismatched types
 --> main.rs:3:8
  |
3 |     if number {
  |        ^^^^^^ expected `bool`, found integer

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

The fix is to ask a real question: `if number != 0 {`. The comparison produces a
`bool`, which is what `if` demands.

> A compiler error here isn't pickiness — "no truthiness" means a condition
> always *says* what it's testing. `if number != 0` tells the next reader
> exactly what counts as yes.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new branching` works too.)* **Predict on paper before you
run each part.**

1. Write `main` with a binding `score` set to any whole number you choose. Use
   an `if` / `else if` / `else` to print `"high"`, `"mid"`, or `"low"` based on
   it (you pick the cutoffs). **Predict** which line prints for your number,
   then run.
2. **Order matters — prove it.** Swap your first two conditions (put the
   *milder* cutoff first). **Predict**: for a number that clears *both*
   cutoffs, what prints now — and why does the stricter check never get asked?
   Swap them back.
3. **Break the condition.** Change your first condition to just the bare
   `score` (no comparison). **Predict the error code** and the two types its
   message will name. Then restore the comparison.

*(You write every line here — I won't. The predictions are your answer key; the
code is yours. Next, Lesson 9b — the Rust twist: your fork in the road can
hand back a value.)*

## 6. What surprised you?

In a sentence or two: did the "no truthiness — hand it a real `bool`" rule trip
you up? Did the order-matters experiment (task 2) change how you'll write
cutoff chains? Tell me, and I'll tune the next lesson to where you actually
are.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.5 "Control Flow → `if`
  Expressions": the `if` / `else if` / `else` shapes, the first-true-wins
  order, and the non-`bool` condition failure repurposed as part 4. (The
  value-producing half of §3.5 is Lesson 9b.)
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Conditional control": the
  basic branching shape.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 8 — Comments & printing](08-comments-and-printing.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 9b — if is an expression: branching that produces a value →](09b-if-as-expression.md)
