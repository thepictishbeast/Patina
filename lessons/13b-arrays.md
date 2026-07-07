# Lesson 13b — Arrays: a fixed row of one type

*(Phase 3 — Text & collections, part 3. Tuples bundle MIXED types; an
**array** is a fixed row where every slot holds the SAME type — and its length
is baked into the type itself. This lesson also pays a debt: Lesson 10c
promised you'd see the indexing bug `for` exists to kill. Here it is.)*

## 1. Why it exists

A row of scores, a board of cells, twelve monthly totals — same type, known
count. An **array** holds a **fixed number** of values of the **same** type,
side by side. Because the length is fixed at compile time, Rust can check
every access against it — sometimes before the program even runs. That check,
and what it catches, is most of this lesson.

Rule of thumb: if the count might ever need to *grow*, reach for a `Vec`
(Lesson 14). An array's length is fixed forever.

## 2. The idea

Written with square brackets; the type is **`[T; N]`** — element type **and
length together**, so `[i32; 5]` and `[i32; 6]` are different types:

```
let scores = [10, 20, 30, 40, 50];   // [i32; 5]
let zeros = [0; 3];                   // shorthand: three 0s → [0, 0, 0]
```

Index with `scores[0]` (0-based, like tuple `.0`), count with `scores.len()`.
And because the length is part of the type, "past the end" isn't a grey area —
every index is checked against `N`, at the earliest moment Rust can manage:
compile time if the index is visible, run time if it isn't. Part 4 shows both.

## 3. A tiny example — you type this one (30-second rep)

Predict all of it:

```rust
fn main() {
    let scores = [10, 20, 30, 40, 50];
    println!("first = {}, len = {}", scores[0], scores.len());
    let zeros = [0; 3];
    println!("{zeros:?}");
}
```

```
first = 10, len = 5
[0, 0, 0]
```

(`{:?}` is the "debug" form from Lesson 8 — it's how you print a whole array.)

## 4. Common pitfalls / real panics — past the end, caught at two different times

**A bad index the compiler can *see* is a build error.** The length (3) and
the index (5) are both right there, so it refuses to build:

```rust
fn main() {
    let a = [10, 20, 30];
    let x = a[5];
    println!("{x}");
}
```

```
error: this operation will panic at runtime
 --> main.rs:3:13
  |
3 |     let x = a[5];
  |             ^^^^ index out of bounds: the length is 3 but the index is 5
  |
  = note: `#[deny(unconditional_panic)]` on by default
```

**A bad index computed at run time panics instead.** The compiler can't see
what `pick()` returns, so the check moves to run time:

```rust
fn pick() -> usize { 5 }
fn main() {
    let a = [10, 20, 30];
    let x = a[pick()];   // compiler can't see what pick() returns
    println!("{x}");
}
```

This compiles, then crashes:

```
thread 'main' panicked at main.rs:4:13:
index out of bounds: the len is 3 but the index is 5
```

> The rule to keep: **the compiler catches the bad index it can see; the
> runtime catches the one it can't.** Either way Rust refuses to hand you junk
> from past the end — that's the safety the bounds check buys.

**How this bites in real code — the off-by-one walk.** Nobody writes `a[5]` on
purpose; they write a loop whose bound is one step too generous (`<=` where it
should be `<`):

```rust
fn main() {
    let a = [10, 20, 30];
    let mut index = 0;
    while index <= a.len() {
        println!("{}", a[index]);
        index += 1;
    }
}
```

**Before you scroll — will this compile, and if it runs, does it finish
cleanly?**

It compiles fine. Then it prints `10`, `20`, `30`… and crashes, because
`a.len()` is `3` and there is no `a[3]`:

```
10
20
30

thread 'main' panicked at main.rs:5:24:
index out of bounds: the len is 3 but the index is 3
```

The fix isn't to fiddle with `<=` vs `<` until it works — it's to stop
indexing by hand. This is the debt Lesson 10c promised: `for` walks the items
themselves, so it *cannot* run off the end:

```rust
fn main() {
    let a = [10, 20, 30];
    for value in a {
        println!("{value}");
    }
}
```

```
10
20
30
```

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new collections` works too.)* **Predict on paper before
each run.**

1. **The row.** Make an array of five numbers, print its first element and its
   `.len()`. Then make a `[0; 4]`-style array and print it with `{:?}`.
   **Predict** both lines.
2. **The two out-of-bounds.** Index your array with a literal that's too big
   (e.g. `a[9]`). **Predict**: compile error or runtime panic? Then write a
   function that *returns* a too-big index and index with that instead.
   **Predict** again — same answer, or different? Run both and see why.
3. **Cause and fix the off-by-one.** Walk your array with a `while` and an
   index bound that's one too far. **Predict** where it crashes and what the
   panic message counts. Then rewrite it as a `for` so the bug can't be
   written at all.

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next: Lesson 13c — borrowing a WINDOW into this row,
without copying it.)*

## 6. What surprised you?

A sentence or two: did "the length is part of an array's type" land? Was the
compile-time-vs-runtime split on the bad index surprising — and did the
off-by-one walk make Lesson 10c's "never name an index" advice concrete? Tell
me, and I'll pitch the slices lesson to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.2 "The Array Type": the two
  defining properties, the array-vs-vector heuristic, and the out-of-bounds
  panic framed as safety.
- **CR** — *Comprehensive Rust* (Google), §8.1: the dual out-of-bounds demo
  (const index = compile error, computed index = runtime panic).
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), §2.2: the `<` vs `<=`
  off-by-one caveat (the while-walk demo, relocated here from the loops lesson
  now that arrays are taught).
- Compiler/runtime output captured live on **rustc 1.95.0** (edition 2024;
  `rustc --edition 2024 FILE.rs`, temp paths and run-specific thread ids
  normalized).

---

<!-- lesson-nav -->
[← Lesson 13 — Tuples: grouping mixed values](13-tuples.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 13c — Slices: a borrowed window →](13c-slices.md)
