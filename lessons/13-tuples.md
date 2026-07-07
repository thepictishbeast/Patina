# Lesson 13 — Tuples: grouping mixed values

*(Phase 3 — Text & collections, part 2. So far each name has held ONE value.
The next three lessons are Rust's fixed-shape ways to hold several — one tool
per lesson: tuples (mixed types), arrays (one type), slices (a borrowed
window). The growable list, `Vec`, follows in Lesson 14.)*

## 1. Why it exists

Often several values belong together *as one thing*: a point's `x` and `y`, a
name with a score, a word with a flag. Passing them around separately invites
mismatches — the name from one record with the score from another. A **tuple**
bundles a **fixed number** of values of **different** types into one value, so
they travel together.

"Fixed" is the theme of all three lessons: the shape is known at compile
time. A tuple's size is part of what it *is* — and Rust has a word for that
size: its **arity**.

## 2. The idea

Written with parentheses:

```
let point = (500, 6.4, 1);
```

That's three values — an integer, a float, an integer — in one bundle (arity
3). You get them back out two ways: **destructure** into names, or reach in by
position with `.0`, `.1`, `.2`:

```
let (x, y, z) = point;   // destructure
let first = point.0;     // by position (0-based)
```

Destructuring is the same "pattern on the left of `=`" you've been using since
`let` — the shape on the left mirrors the shape on the right, and it must
match **exactly**: three slots for a three-tuple. Part 4 shows what happens
when it doesn't.

(The empty tuple `()` is the **unit** value — it's what an expression with no
real value produces, the same `()` you met when a `;` "threw the value away"
in Lesson 6. A tuple of arity zero: a bundle of nothing.)

## 3. A tiny example to read

**Build, destructure, dot-access.** Predict the two lines:

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

## 4. Common pitfalls / real compiler errors

**Destructuring with the wrong arity — `E0308`.** The pattern's shape must
mirror the tuple's shape. Leave a slot out and the compiler counts for you:

```rust
fn main() {
    let point = (500, 6.4, 1);
    let (x, y) = point;
    println!("{x} {y}");
}
```

**Before you scroll — the tuple has three values and the pattern has two
names. What happens to the third?**

```
error[E0308]: mismatched types
 --> main.rs:3:9
  |
3 |     let (x, y) = point;
  |         ^^^^^^   ----- this expression has type `({integer}, {float}, {integer})`
  |         |
  |         expected a tuple with 3 elements, found one with 2 elements
  |
  = note: expected tuple `({integer}, {float}, {integer})`
             found tuple `(_, _)`

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Nothing "happens to" the third value — the destructure simply refuses:
**"expected a tuple with 3 elements, found one with 2 elements."** Arity is
part of the type. If you genuinely don't need a slot, name it `_` (a slot you
deliberately ignore): `let (x, y, _) = point;` keeps the arity honest while
discarding the value.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new collections` works too.)* **Predict on paper before
each run.**

1. **Bundle and unpack.** Make a tuple holding a word (`&str`), a whole
   number, and a `bool`. Destructure it into three names and print them. Then
   print the middle one again using `.1`. **Predict** both outputs.
2. **Break the arity.** Destructure your three-tuple into just two names.
   **Predict the error code** and the exact counting phrase the compiler will
   use. Then fix it the `_` way — keep two names, ignore one slot — and
   confirm it runs.
3. **Arity zero.** Write `let nothing = ();` and then a `println!` of
   `{nothing:?}` (debug form, Lesson 8). **Predict** what prints — and say in
   one sentence where you've already been *producing* this value since
   Lesson 6 without naming it.

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next: Lesson 13b — a fixed row where every slot is the
SAME type, and what Rust does when you reach past its end.)*

## 6. What surprised you?

A sentence or two: did "arity is part of the type" land — a three-tuple and a
two-tuple being as different as a number and a string? Did the unit `()` tie
back to the semicolon lesson click? Tell me, and I'll pitch the arrays lesson
to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.2 "The Tuple Type": the two
  defining properties, destructuring vs `.0` access, and the unit type.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), §2.1: the term **arity**.
- Compiler output captured live on **rustc 1.95.0** (edition 2024;
  `rustc --edition 2024 FILE.rs`, temp paths normalized to `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 12 — `String` vs `&str`](12-string-vs-str.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 13b — Arrays: a fixed row of one type →](13b-arrays.md)
