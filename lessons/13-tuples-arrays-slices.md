# Lesson 13 — Tuples, Arrays, and Slices

*(Phase 3 — Text & collections, part 2. Three fixed-shape ways to hold more than one
value. The *growable* list, `Vec`, is the next lesson.)*

## 1. Why it exists

So far each name has held a single value. Often you want to keep several together — a
point's `x`/`y`, a row of scores, the middle few items of a list. Rust has three tools
for this when the *shape* is known up front:

- a **tuple** groups a **fixed number** of values of **different** types,
- an **array** holds a **fixed number** of values of the **same** type,
- a **slice** is a borrowed **view** into part of an array (or a string).

"Fixed" is the theme — all three have a size known at compile time. When you need a
list that grows at run time, that's `Vec` (Lesson 14).

> **How the sources frame it:** the **BOOK** defines tuples and arrays by their two
> properties and sells the array bounds-check as *safety*; **CR** owns the sharpest
> failing demo (the array out-of-bounds pair) and the slice idea ("the length drops
> out of the type"); **BLOG** contributes the word **arity** and the line that a
> `&str` *is* a slice of a `String`.

## 2. The idea

**Tuple — a fixed group of mixed types.** Written with parentheses:

```
let point = (500, 6.4, 1);
```

That's three values — an integer, a float, an integer — in one bundle (its **arity**
is 3). You get them back out two ways: **destructure** into names, or reach in by
position with `.0`, `.1`, `.2`:

```
let (x, y, z) = point;   // destructure
let first = point.0;     // by position (0-based)
```

(The empty tuple `()` is the **unit** value — it's what an expression with no real
value produces, the same `()` you met when a `;` "threw the value away" in Lesson 6.)

**Array — a fixed row of one type.** Written with square brackets; its type is
`[T; N]` — element type **and length together**, so `[i32; 5]` and `[i32; 6]` are
different types:

```
let scores = [10, 20, 30, 40, 50];   // [i32; 5]
let zeros = [0; 3];                   // shorthand: three 0s → [0, 0, 0]
```

Index with `scores[0]`, count with `scores.len()`. Rule of thumb: if you might ever
need it to grow, reach for a `Vec` instead (Lesson 14) — an array's length is fixed
forever.

**Slice — a borrowed view of part of a sequence.** `&scores[1..3]` is a **slice**: a
window onto elements 1 and 2 of `scores`. Its type is `&[i32]` — notice the length is
**gone** from the type. That's the point: a function taking `&[i32]` works on a slice
of *any* length. (This is the same idea as L12's `&str`, which is just a slice of a
string. The deeper "borrowing" rules behind the `&` are Phase 4 — here it's "a window
into a sequence.")

## 3. Tiny examples to read

**Tuple — build, destructure, dot-access.** Predict the two lines:

```rust
fn main() {
    let point = (500, 6.4, 1);
    let (x, y, z) = point;
    println!("x={x}, y={y}, z={z}");
    println!("first via .0 = {}", point.0);
}
```

```
x=500, y=6.4, z=1
first via .0 = 500
```

**Array — you type this one (30-second rep).** Predict all of it:

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

**Slice — a view into the array:**

```rust
fn main() {
    let scores = [10, 20, 30, 40, 50];
    let middle: &[i32] = &scores[1..3];   // elements 1 and 2
    println!("{middle:?}");
}
```

```
[20, 30]
```

`1..3` is the exclusive range from Lesson 10 — it takes index 1 up to *but not
including* 3. *(That array block was your write-rep; part 5 is the rest.)*

## 4. Common pitfalls / real compiler errors

**Indexing past the end — two ways, caught at two different times.** This is the most
useful thing in the lesson. First, an index the compiler can *see* is too big:

```rust
fn main() {
    let a = [10, 20, 30];
    let x = a[5];
    println!("{x}");
}
```

**Will this compile?** No — the compiler can read both the length (3) and the index (5)
right here, so it refuses at build time. Real `rustc` (1.95.0):

```
error: this operation will panic at runtime
 --> main.rs:3:13
  |
3 |     let x = a[5];
  |             ^^^^ index out of bounds: the length is 3 but the index is 5
  |
  = note: `#[deny(unconditional_panic)]` on by default
```

But when the index is computed at run time, the compiler *can't* see it, so the check
moves to run time — and a bad index then **panics**:

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

> The rule to keep: **the compiler catches the bad index it can see; the runtime
> catches the one it can't.** Either way Rust refuses to hand you junk from past the
> end — that's the safety the bounds check buys.

**Slicing a string mid-character.** Slices count **bytes**, and (from L12) text is
UTF-8 where one character can be several bytes. Slice through the middle of one and it
panics:

```rust
fn main() {
    let s = String::from("Здравствуйте");
    let slice = &s[0..1];
    println!("{slice}");
}
```

```
thread 'main' panicked at main.rs:3:19:
end byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
```

The first letter takes **two** bytes, so `0..1` cuts it in half. Slice on real
character boundaries (here `&s[0..2]` is the first letter) — the error even tells you
where the boundary is.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new collections` works too.)* **Predict on paper before each run.**

1. **Tuple.** Make a tuple holding a word (`&str`), a whole number, and a `bool`.
   Destructure it into three names and print them. Then print the middle one again using
   `.1`. **Predict** both outputs.

2. **Array + slice.** Make an array of five numbers. Print its `.len()`, then take a slice
   of the **last two** (think about the right range) and print it with `{:?}`. **Predict**
   the slice before you run.

3. **The two out-of-bounds.** Index your array with a literal that's too big (e.g.
   `a[9]`). **Predict**: compile or runtime error? Then make a function that returns a
   too-big index and index with *that* instead. **Predict** again — same answer, or
   different? Run both and see why.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours.)*

## 6. What surprised you?

A sentence or two: did "the length is part of an array's type" land? Was the
compile-time-vs-runtime split on the bad index surprising? Tell me, and I'll pitch
Lesson 14 (`Vec` and `HashMap` — the growable collections) to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.2 "The Tuple Type" / "The Array Type."
  Backbone for both: the two defining properties of each, the array-vs-vector heuristic,
  and the out-of-bounds panic framed as safety.
- **CR** — *Comprehensive Rust* (Google), §8.1–8.4 and §9.3. The dual out-of-bounds demo
  (const index = compile error, computed index = runtime panic) and the slice idea — the
  length drops out of `&[T]`, so one function fits any size. Range shorthands too.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), §2.1–2.3. The term **arity** and the
  framing that a `&str` is a slice of a `String`.
- The "a view / window into a sequence" framing for slices is where CR and BLOG converge.
  Deep borrowing (why the `&` matters, the `E0502` "slice outlives a change" error) is a
  Phase-4 forward reference, not taught here. Compiler output captured on **rustc 1.95.0**
  (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 12 — `String` vs `&str`](12-string-vs-str.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 14 — `Vec` and `HashMap` →](14-vec-hashmap.md)
