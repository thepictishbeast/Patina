# Lesson 34b — Operator overloading & associated types

*(Phase 9 — Advanced, part 7. You've owed yourself this one since Lesson 31:
what is the `Output =` inside `impl Future<Output = u32>` actually doing? The
answer is an **associated type** — and the friendliest place to meet one is
the trait that makes `+` work on your own structs.)*

## 1. Why it exists

Write `a + b` where `a` and `b` are your own struct and Rust refuses — it has
no idea what "+" should *mean* for your type. But `+` isn't magic for the
built-ins either: it's a **trait method**. Implement the `Add` trait from
`std::ops` and your type gains `+`, exactly the way implementing `Summary`
gained you `summarize` in Lesson 25. A trait per operator: `Add` for `+`,
`Sub` for `-`, `Mul` for `*`, and so on.

One new thing rides along. `Add` has a line you haven't written in a trait
before: `type Output`. That's an **associated type** — a type the implementor
chooses, named once per `impl`. It answers "what does `+` *produce*?" without
threading a generic parameter through every use. It's also how `Iterator`
names what it yields (`type Item`) — and it's the `Output` you read inside
Lesson 31's `impl Future<Output = u32>`.

## 2. The idea

- **Operator traits live in `std::ops`.** `a + b` is sugar for
  `Add::add(a, b)` — a plain method call you can implement yourself.
- **An associated type is a slot the implementor fills.** The trait says
  `type Output;`; your `impl` says `type Output = Point;`. Callers then just
  use `+` and get a `Point` back — no extra type parameters anywhere.
- **Reading the angle-bracket form.** When a trait's associated type appears
  in bounds or return types, it's written `Trait<Output = T>` — so
  `impl Future<Output = u32>` (Lesson 31) reads: "some future whose `Output`
  associated type is `u32`," i.e. a future that eventually produces a `u32`.
  You can now read that line exactly, not just approximately.

> **How the sources frame it:** the **BOOK** Ch.20 §20.2 "Advanced Traits"
> carries both ideas — associated types (via `Iterator::Item`) and operator
> overloading (via `Add`, "default generic type parameters" and all). The
> section goes further (supertraits, the newtype pattern, fully-qualified
> syntax) — read it there when a real API pushes you past this lesson.

## 3. A tiny example to read

**`Add` makes `+` work on your type (BOOK).** `type Output` names what `+`
produces:

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let a = Point { x: 1, y: 0 };
    let b = Point { x: 2, y: 3 };
    println!("{:?}", a + b);
}
```

```
Point { x: 3, y: 3 }
```

`a + b` calls the `add` method you wrote — `+` is just sugar for `Add::add`.
Note the three appearances of the same fact: `type Output = Point;` declares
what addition produces, `-> Point` on the method agrees, and the caller's
`{:?}` prints a `Point`.

## 4. Common pitfalls / real compiler errors

**Using `+` before implementing `Add` — `E0369`.** Delete the `impl Add`
block from part 3 and the caller's `+` has nothing to call:

```rust
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 0 };
    let b = Point { x: 2, y: 3 };
    println!("{:?}", a + b);
}
```

```
error[E0369]: cannot add `Point` to `Point`
  --> main.rs:10:24
   |
10 |     println!("{:?}", a + b);
   |                      - ^ - Point
   |                      |
   |                      Point
   |
note: an implementation of `Add` might be missing for `Point`
  --> main.rs:2:1
   |
 2 | struct Point {
   | ^^^^^^^^^^^^ must implement `Add`
note: the trait `Add` must be implemented
```

You met `E0369` in Lesson 24 as the generic `T + T` wall — this is its
concrete-struct flavour, and the note **names the exact fix**: "an
implementation of `Add` might be missing." The compiler isn't saying your
type *can't* be added; it's saying you haven't *defined what adding means*
yet. Same error, same cure, both times: a trait.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new operators` works too.)* **Predict on paper before
each run.**

1. **Hit the wall first.** Make a small struct holding two numbers (a point, a
   pair, a 2-D vector — your call) and try `a + b` on two of them **without**
   implementing anything. **Predict the error code** and which trait the note
   will tell you to implement. Read the note out loud — it's your task list.
2. **Overload `+`.** Now implement `Add` for it so that `+` combines two of
   them field-by-field (part 3 shows the shape). **Predict** the output of
   adding two specific values *before* you run it. Which three places in your
   code all state what `+` produces?
3. **A second operator.** Implement `Sub` as well so `-` works too. **Predict**
   before compiling: what changes compared to `Add` — the trait name, the
   method name, and what stays identical? (It's the same shape, in `std::ops`.)
4. **Read the L31 line, exactly.** No code — in one sentence, say what
   `impl Future<Output = u32>` means, using the words "associated type."
   Compare with what Lesson 31's gloss told you to *approximately* read it as.

*(You write every line here — I won't. The predictions are your answer key.
Next: the third advanced corner — macros, the code that writes code.)*

## 6. What surprised you?

A sentence or two: did seeing `+` resolve to an `Add::add` method call change
how you read operators? And did `Output =` click — the same `E0369` you met
with generics in Lesson 24, the same `Output` you read on futures in
Lesson 31, all one trait-system idea? Tell me, and I'll fold it into the
Phase-9 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.20 §20.2 "Advanced
  Traits"**: associated types (introduced via `Iterator`'s `type Item`),
  operator overloading via `std::ops::Add` (the `Point` example this lesson
  reproduces), and the further material — supertraits, newtype,
  fully-qualified syntax — this lesson points at rather than teaches.
- **CR** — *Comprehensive Rust* (Google): brief slides on operator
  overloading and associated types; the fuller arc is the Book's.
- Every snippet compiled and run, and the error captured live, on **rustc
  1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`; temp paths
  normalized to `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 34 — unsafe: a small, audited escape hatch](34-unsafe.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 34c — Declarative macros: macro_rules! →](34c-macro-rules.md)
