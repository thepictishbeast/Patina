# Lesson 28b — Adapter chains: `map`, `filter`, `collect`

*(Phase 7, part 4. Lesson 28 gave you the cursor. Now you stack small named steps
on it — and the code starts reading like the sentence you'd say out loud.)*

## 1. Why it exists

You constantly walk a list and do something to each item: double every number,
keep only the even ones, add them up. A `for` loop with an `if` and a running
total works — but the *what* (double, keep-even, sum) gets tangled up with the
*how* (loop bookkeeping, the accumulator). Adapter chains separate them: each
step does exactly one job, and the chain reads top-to-bottom — "take the
numbers, keep the even ones, double them, collect the result."

## 2. The idea

The steps split into two kinds:

- **Adapters** — `map` (transform each item), `filter` (keep items that pass a
  test). Each returns a **new lazy iterator**. Stacking adapters just builds a
  bigger *plan*; no work is done.
- **Consumers** — `sum`, `collect`, or a `for` loop. These actually *drive* the
  iterator: they call `next` over and over until `None`, doing the work. A chain
  with no consumer on the end does nothing at all (part 4 proves it).

**`collect`** runs the iterator and gathers the results into a collection —
usually a `Vec`. It can build several different types, so it needs to know
*which* one you want: put the type on the binding (`let v: Vec<i32> = …`).
(There's also a spell-it-on-collect form nicknamed the turbofish — you met the
`::<>` shape in Lesson 20; Book Ch. 13.2 shows it here. One of the exercises has
you feel *why* the annotation is needed.)

## 3. A tiny example to read

**Filter, then map, then collect** — the payoff in one expression. `.copied()`
turns the `&i32`s from `iter` into plain `i32`s so the closures work on values:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let result: Vec<i32> = v
        .iter()
        .copied()
        .filter(|x| x % 2 == 0)   // keep 2, 4, 6
        .map(|x| x * 2)           // double them → 4, 8, 12
        .collect();               // run it, gather into a Vec
    println!("{result:?}");
}
```

```
[4, 8, 12]
```

Read it top to bottom like a sentence. But remember: `filter` and `map` did
**nothing** on their own — they only built the plan. The single `collect` at the
end drove the whole chain. (A range like `1..=4` is itself an iterator, so you
can hang adapters straight off one — no `.iter()` needed.)

## 4. Common pitfalls — the laziness rule, made visible

An adapter with no consumer does nothing. The `.map()` here builds an iterator
and then drops it on the floor:

```rust
fn main() {
    let v = vec![1, 2, 3];
    v.iter().map(|x| x * 2);
    println!("done");
}
```

```
warning: unused `Map` that must be used
 --> main.rs:3:5
  |
3 |     v.iter().map(|x| x * 2);
  |     ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: iterators are lazy and do nothing unless consumed
```

It compiles (a **warning**, not an error — the second warning-class trap in this
course, after Lesson 19c's unreachable arm) and runs, printing only `done`. The
doubling *never happened*. The compiler's note says it plainly: "iterators are
lazy and do nothing unless consumed." The fix is a consumer on the end —
`.collect()`, `.sum()`, or a `for` loop.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then work through the three
matching exercises via the **Practice this lesson** links at the bottom — all
three are about telling a consumer its target type. *(On your own machine, a
playground or `cargo new chains` works too.)* **Predict on paper before each
run.**

1. **A three-step chain.** From `vec![1, 2, 3, 4, 5, 6, 7, 8]`, build one chain
   that keeps numbers greater than 3, squares the survivors, and collects into a
   `Vec`. **Predict the exact contents** before running. Then delete the final
   `collect` and bind the bare chain — **predict** the warning and whether any
   squaring happens.
2. **Leave `collect` guessing.** Write a chain ending in `.collect()` with NO
   type anywhere. **Predict**: error or warning? which code? Read what the
   compiler asks you to add.
3. **Sum a range.** Total `1..=100` with `.sum()` — remember the binding needs a
   type. **Predict** the number first (a famous shortcut exists; check yourself
   against the compiler).

*(You write every line here — I won't. The predictions are your answer key. Once
chains click, the old loop-with-an-accumulator starts to feel like doing by hand
what one sentence would say. Next: smart pointers.)*

## 6. What surprised you?

A sentence or two: did the laziness rule — a chain with no consumer doing
*nothing* — catch you off guard? Does the chain read like a sentence to you yet?
Tell me, and I'll fold it into the Phase-7 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.13.2**: adapters vs consuming
  adapters, and the `unused_must_use` warning that proves a bare adapter does
  nothing.
- **CR** — *Comprehensive Rust* (Google), §26: the one-expression
  `filter`→`map`→`collect` chain and the `collect` type-annotation point.
- Every snippet compiled and run, and every warning captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 28 — Iterators: the next() cursor](28-iterator-cursor.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 29 — Box: values on the heap →](29-box.md)
