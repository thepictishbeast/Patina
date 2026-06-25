# Lesson 28 — Iterators

*(Phase 7 — Functional features & smart pointers, part 2. Back in Lesson 10 you
wrote `for item in collection { … }` and it just worked. This lesson opens that
box: a `for` loop is asking a collection for an **iterator** — a little cursor
that hands out one item at a time. Once you can name that cursor, you can also
*transform* the stream before the loop ever sees it.)*

## 1. Why it exists

You constantly need to walk a list and do something to each item: double every
number, keep only the even ones, add them all up. You already know one way — a
`for` loop with an `if` and a running total inside it. That works, but the *what*
(double, keep-even, sum) gets tangled up with the *how* (the loop bookkeeping,
the index, the accumulator).

An **iterator** separates those. It's one object that knows how to produce the
next item, and nothing else. On top of it you stack small, named steps —
`map` to transform, `filter` to keep some, `sum` to total — each doing exactly
one job. The loop machinery disappears, and the code reads like the sentence you'd
say out loud: "take the numbers, keep the even ones, double them, collect the
result."

> **How the sources frame it:** the **BOOK** Ch.13.2 is the backbone — it states
> the laziness rule outright, shows the `Iterator` trait with its one required
> method `next`, and draws the key line between *adapters* that return a new lazy
> iterator (`map`, `filter`) and *consumers* that drive it to the end (`sum`,
> `collect`). **CR** §26 adds the one-expression `filter`→`map`→`sum` chain and the
> `collect` type-annotation point. No metaphor is invented here — the cursor idea
> carries itself.

## 2. The idea

**An iterator is a cursor.** It has one job, captured by the `Iterator` trait:
hand back the next item when asked. The method that does it is `next`:

```
fn next(&mut self) -> Option<Self::Item>
```

Each call returns `Some(item)` for the next value, and `None` once there's nothing
left (you know `Option` from Lesson 19). It takes `&mut self` because asking for
the next item *advances* the cursor — so the iterator has to be `mut`.

**Iterators are lazy.** Making an iterator, or stacking steps on it, does *no
work*. Nothing happens until something actually pulls items out. This is the rule
worth memorising: an iterator sitting there is a *plan*, not a result.

The steps split into two kinds:

- **Adapters** — `map` (transform each item), `filter` (keep items that pass a
  test). Each returns a **new lazy iterator**. Stacking adapters just builds a
  bigger plan; still no work done.
- **Consumers** — `sum`, `collect`, or a `for` loop. These actually *drive* the
  iterator: they call `next` over and over until `None`, doing the work. A chain
  with no consumer on the end does nothing at all (part 4 proves it).

**Three ways to ask a collection for its iterator** — this is the box `for` was
hiding, and the difference is *what you get to touch*:

- **`.iter()`** — borrows each item, hands you `&T` (read-only). The collection
  survives.
- **`.iter_mut()`** — borrows each item mutably, hands you `&mut T` (you can
  change items in place). The collection survives.
- **`.into_iter()`** — *takes ownership*, hands you each `T` by value. The
  collection is consumed and gone afterward.

And the tie-back to Lesson 10: a `for` loop picks one of these for you based on
how you write it. `for x in &v` uses `iter`, `for x in &mut v` uses `iter_mut`,
and `for x in v` uses `into_iter` (and eats `v`).

**`collect`** is the consumer that runs the iterator and gathers the results into
a collection — usually a `Vec`. It can build several different types, so it needs
to know *which* one you want. You tell it either with a type on the binding
(`let v: Vec<i32> = …`) or with a turbofish on `collect` itself
(`.collect::<Vec<i32>>()`). Same result; pick whichever reads clearer.

## 3. Tiny examples to read

**`next` walks the cursor by hand.** A `for` loop calls `next` for you; here we do
it ourselves so you can see the `Some`/`None`. Note `it` must be `mut`:

```rust
fn main() {
    let small = vec![1, 2, 3];
    let mut it = small.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}
```

```
Some(1)
Some(2)
Some(3)
None
```

Each call moved the cursor forward one step; after the third item it's exhausted,
so the fourth call returns `None` — the same `None` a `for` loop watches for to
know it's done.

**`iter` vs `iter_mut` vs `into_iter` — the three borrows.** Same vector, three
flavours of access. `iter_mut` lets you change items in place (`*x` to reach
through the `&mut`):

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    for x in v.iter_mut() {
        *x += 10;          // change each item through &mut
    }
    println!("{v:?}");     // v survived, now changed

    let total: i32 = v.iter().sum();  // borrow each &i32 to read
    println!("{total}");

    let owned: Vec<i32> = v.into_iter().collect();  // takes ownership of v
    println!("{owned:?}");
    // v is gone now — into_iter consumed it
}
```

```
[11, 12, 13]
36
[11, 12, 13]
```

**A chain: filter, then map, then collect.** This is the payoff — the whole
"keep the even ones, double them, gather them up" in one expression. `.copied()`
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
end is what drove the whole chain and produced the `Vec`.

**`collect` with a turbofish instead of a type on the binding.** Same job, the
type just lives on `collect` rather than on `let`:

```rust
fn main() {
    let doubled = (1..=4).map(|n| n * 2).collect::<Vec<i32>>();
    println!("{doubled:?}");
}
```

```
[2, 4, 6, 8]
```

(A range like `1..=4` is itself an iterator, so you can hang adapters straight off
it — no `.iter()` needed.)

## 4. Common pitfalls / real compiler errors

**An adapter with no consumer does nothing — the laziness warning.** This is the
rule made visible. The `.map()` here builds an iterator and then we drop it on the
floor without ever consuming it:

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
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
3 |     let _ = v.iter().map(|x| x * 2);
  |     +++++++
```

It compiles (it's a warning, not an error) and runs — printing only `done`. The
doubling *never happened*. The compiler's note says it plainly: "iterators are
lazy and do nothing unless consumed." The fix is to put a consumer on the end —
`.collect()` into a `Vec`, a `.sum()`, or a `for` loop.

**Using a collection after `into_iter` ate it — `E0382`.** `into_iter` takes
ownership, so the original is gone afterward. Touch it again and the borrow
checker stops you:

```rust
fn main() {
    let v = vec![1, 2, 3];
    let _doubled: Vec<i32> = v.into_iter().map(|x| x * 2).collect();
    println!("{v:?}");
}
```

```
error[E0382]: borrow of moved value: `v`
 --> main.rs:4:16
  |
2 |     let v = vec![1, 2, 3];
  |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     let _doubled: Vec<i32> = v.into_iter().map(|x| x * 2).collect();
  |                                ----------- `v` moved due to this method call
4 |     println!("{v:?}");
  |                ^ value borrowed here after move
  |
note: `into_iter` takes ownership of the receiver `self`, which moves `v`
help: you can `clone` the value and consume it, but this might not be your desired behavior
  |
3 |     let _doubled: Vec<i32> = v.clone().into_iter().map(|x| x * 2).collect();
  |                               ++++++++
```

The note names the exact cause: "`into_iter` takes ownership of the receiver
`self`, which moves `v`." If you need `v` again afterward, you wanted `.iter()`
(which only *borrows*) — then `v` is never moved and the last line compiles fine.
This is the whole `iter` vs `into_iter` choice, enforced by the compiler.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new iterators`. **Predict on paper before each
run.**

1. **Walk the cursor.** Make a `Vec` of three strings. Get an iterator from it
   with `.iter()`, bind it to a `mut` variable, and call `.next()` four times,
   printing each result with `{:?}`. **Predict** all four lines before running —
   in particular, what does the fourth call give you, and why? Then try removing
   the `mut` and **predict** what the compiler says.

2. **A three-step chain.** Start from `vec![1, 2, 3, 4, 5, 6, 7, 8]`. Build one
   chained expression that keeps only the numbers greater than 3, squares each
   surviving number, and collects them into a `Vec`. **Predict the exact contents
   of the result** before you run it. Then delete the final consumer (the
   `collect`) and bind the chain to a variable on its own — **predict** what the
   compiler warns and whether any squaring happens.

3. **The three borrows, side by side.** With a `mut` vector of numbers: first use
   `.iter_mut()` in a `for` loop to add `1` to every element, then use `.iter()`
   with `.sum()` to total them, then `.into_iter().collect()` into a new `Vec`.
   After the `into_iter` line, add a `println!` that uses the original vector
   again. **Predict the error code** before compiling. Which of the three methods
   would you swap in so that last line compiles?

4. **`collect`, two ways.** Take a range `1..=5`, map each number to itself times
   ten, and collect into a `Vec`. Write it **once** with the type on the `let`
   binding and **once** with a turbofish on `collect`. **Predict** that both print
   the same thing — then confirm.

*(You write every line here — I won't. The predictions are your answer key; the
code is yours. Once iterators click, that old `for`-loop-with-an-accumulator
starts to feel like doing by hand what one chain would say in a line.)*

## 6. What surprised you?

A sentence or two: did "an iterator is just a cursor that hands out one item at a
time" change how the `for` loop from Lesson 10 looks now? Did the laziness rule —
a chain with no consumer doing *nothing* — catch you off guard? Did the
`iter` / `iter_mut` / `into_iter` split land as "borrow read-only / borrow to
change / take ownership"? Tell me, and I'll fold it into the Phase-7 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.13.2** ("Processing a Series of
  Items with Iterators"): the laziness rule, the `Iterator` trait and `next`
  returning `Some`/`None`, and the split between iterator adapters (`map`,
  `filter` — return a new lazy iterator) and consuming adapters (`sum`, `collect`
  — drive the iterator), including the `unused_must_use` warning that proves a bare
  adapter does nothing.
- **CR** — *Comprehensive Rust* (Google), §26: the one-expression
  `filter`→`map`→`sum`/`collect` chain, and the `collect` type-annotation point
  (turbofish `.collect::<Vec<_>>()` vs a type on the binding).
- Ties back to **Lesson 10** (`for` loops) — a `for` is `iter`/`iter_mut`/
  `into_iter` chosen by how you write the loop head — and **Lesson 19**
  (`Option`), which `next` returns.
- Every snippet compiled and run, and every warning/error captured live, on
  **rustc 1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`).

---

<!-- lesson-nav -->
[← Lesson 27 — Closures: Anonymous Functions That Capture](27-closures.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 29 — Smart Pointers: `Box`, `Rc`, `RefCell`, `Deref`, `Drop` →](29-smart-pointers.md)
