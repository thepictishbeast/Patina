# Lesson 34 — Advanced Features: `unsafe`, Traits & Macros

*(Phase 9 — Advanced. A brief tour of three corners of Rust you'll **rarely**
reach for, but should recognise when you meet them: the `unsafe` keyword (a small,
audited escape hatch — **not** "C mode"), a one-paragraph nod to advanced traits
(associated types, operator overloading), and **declarative macros** with
`macro_rules!` — how `vec![...]` is built. The point of this lesson is the *map*:
what exists, and when you'd reach for it. You won't need any of it for everyday
code.)*

## 1. Why it exists

Almost everything in Rust is checked for you: the borrow checker proves your
references are valid, types line up, nothing escapes its owner. That safety is the
whole appeal. But three needs don't fit inside the everyday, fully-checked language:

- **A few operations the compiler can't prove safe.** Talking to hardware, calling
  into C, or following a hand-built pointer — the compiler can't verify these, so it
  refuses them by default. `unsafe` is the keyword that says "I've checked this by
  hand; let me through." It's a small, **auditable** marker, not a different mode.
- **Traits that need to carry a type, or hook into operators.** Some traits have an
  output type baked in (an iterator's `Item`); others let `+` or `*` work on your own
  structs. That's a richer corner of the trait system than `impl Trait for Type`.
- **Code that writes code.** `println!`, `vec!`, `assert!` all end in `!` — they're
  **macros**, expanded into real code *before* compilation. They can take any number
  of arguments and generate the matching code, which a plain function can't do. You
  can write your own.

> **How the sources frame it:** the **BOOK** Ch.20 "Advanced Features" is the
> backbone — the only source that gathers the whole umbrella (the five `unsafe`
> superpowers, advanced traits, advanced types, and `macro_rules!`) in one place,
> each framed as "rarely needed, here as reference." It opens: *"Rust has a second
> language hidden inside it… called unsafe Rust [that] gives us extra superpowers."*
> **CR** sharpens the responsible framing for `unsafe` specifically: the keyword
> **shifts the burden of upholding Rust's rules from the compiler to the
> programmer** — you're not switching the rules off, you're promising to keep them.

## 2. The idea

Three tools, each a one-line job:

- **`unsafe { }`** — a marked block where five normally-forbidden abilities ("the five
  superpowers") become available. The block is small on purpose: it's the *only* place
  those abilities live, so when something goes wrong you have a short list to audit.
  Crucially, **`unsafe` does not turn off the borrow checker.** Ownership, borrowing,
  and type-checking all still apply inside the block. It unlocks exactly five extra
  abilities and **nothing else**.

  The five superpowers are: (1) **dereference a raw pointer** (`*const T` / `*mut T`);
  (2) **call an `unsafe` function or method**; (3) **access or modify a mutable
  `static`** variable; (4) **implement an `unsafe` trait**; (5) **access the fields of
  a `union`**. We'll *run* the first one below and leave the rest as a list — they're
  rare, and the first is the one you can read at a glance.

- **Advanced traits (one paragraph).** Some traits carry an **associated type** — a
  type chosen by the implementor, written `type Item;` in the trait and filled in once
  per `impl` (this is how `Iterator` names what it yields without a generic parameter
  everywhere). And the operator traits in `std::ops` let you **overload operators**:
  implement `Add` for your struct and `+` works on it, implement `Mul` and `*` works,
  and so on. Both are part of the same trait system you already know — just richer
  corners of it. We'll see one tiny `Add` example and move on.

- **`macro_rules!`** — a **declarative macro**: you write *patterns* (like `match`
  arms) describing what the macro's input can look like, and for each pattern, the code
  to generate. The compiler matches your call against the patterns and pastes in the
  matching code before compiling. This is how a single `vec![1, 2, 3]` expands into
  "make a `Vec`, push 1, push 2, push 3." We'll build a stripped-down version.

The honest summary: **you can write real Rust for a long time without any of this.**
This lesson is so you *recognise* it — and know the `unsafe` block is a tiny, audited
hole you open deliberately, never a way to silence the compiler wholesale.

## 3. Tiny examples to read

**`unsafe` — dereferencing a raw pointer.** A *raw pointer* (`*const T` for read-only,
`*mut T` for read-write) is a bare address with none of a reference's guarantees.
**Creating** one is safe; **following** one (the `*r` deref) is the superpower that
needs an `unsafe` block. The `&raw const` / `&raw mut` operators make raw pointers
from a value:

```rust
fn main() {
    let mut num = 5;

    let r1 = &raw const num;   // a *const i32 (read-only raw pointer)
    let r2 = &raw mut num;     // a *mut i32  (read-write raw pointer)

    unsafe {
        *r2 += 10;             // write through the raw pointer
        println!("r1 reads: {}", *r1);
    }
}
```

```
r1 reads: 15
```

Both pointers refer to the same `num`, so the write through `r2` is visible when we
read through `r1`. Everything risky — the two `*` derefs — sits inside the one
`unsafe` block. That block is your audit boundary: if this program ever misbehaves
through a pointer, *this is the only place to look.* Outside it, the compiler is back
in charge.

**Operator overloading — `Add` makes `+` work on your type.** Implement the `Add`
trait (from `std::ops`) and the `+` operator works on your struct. `type Output`
is an **associated type** — it names what `+` produces:

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

`a + b` calls the `add` method you wrote — `+` is just sugar for `Add::add`. That's
the whole idea of operator overloading: a trait per operator.

**`macro_rules!` — a mini `vec!`.** This defines `my_vec!`, which takes any number of
comma-separated expressions and builds a `Vec` from them. Read it as a `match`: the
pattern `$( $x:expr ),*` means "zero or more expressions, comma-separated"; the body
repeats `v.push($x);` once **per** matched expression:

```rust
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3];
    println!("v = {v:?}");
}
```

```
v = [1, 2, 3]
```

`$x:expr` captures one expression; the `$( ... ),*` around it means "repeated, with
commas between"; and `$( v.push($x); )*` in the body emits one `push` line for each
captured `$x`. So `my_vec![1, 2, 3]` expands, *before compiling*, into roughly: make a
`Vec`, push `1`, push `2`, push `3`, hand it back. The real `vec!` does the same job
(plus pre-sizing and more patterns); this is its skeleton. (Adapted from the BOOK's
`vec!` listing in Ch.20.)

## 4. Common pitfalls / real compiler errors

**Dereferencing a raw pointer *outside* `unsafe` — `E0133`.** Making the pointer is
fine; following it is the superpower, and it must be inside an `unsafe` block:

```rust
fn main() {
    let num = 5;
    let r = &raw const num;   // making a raw pointer is fine...

    println!("{}", *r);       // ...but following it is not, without unsafe
}
```

```
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
 --> main.rs:5:20
  |
5 |     println!("{}", *r);       // ...but following it is not, without unsafe
  |                    ^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
```

The fix is to wrap the deref in `unsafe { }` (the working version is in part 3). The
note spells out *why* it's gated: a raw pointer carries none of a reference's
guarantees, so following one is on you to get right.

**`unsafe` does NOT switch off the borrow checker — `E0502`.** A common
misunderstanding is that `unsafe { }` is "anything goes." It isn't: ownership and
borrowing are checked exactly as everywhere else. Here the raw-pointer deref genuinely
needs `unsafe`, but the ordinary borrow violation right above it still fails to
compile:

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    let first = &v[0];                 // immutable borrow of v
    v.push(4);                         // mutable borrow while `first` is alive

    let r = &raw const v;              // raw pointer — making it needs no unsafe
    unsafe { println!("len = {}", (*r).len()); }   // following it does
    println!("first = {first}");
}
```

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> main.rs:5:5
  |
4 |     let first = &v[0];                 // immutable borrow of v
  |                  - immutable borrow occurs here
5 |     v.push(4);                         // mutable borrow while `first` is alive
  |     ^^^^^^^^^ mutable borrow occurs here
...
9 |     println!("first = {first}");
  |                        ----- immutable borrow later used here
```

This is the lesson's core point made concrete: `unsafe` unlocks **five specific
abilities** and *only* those. Borrow checking, ownership, and type-checking stay on.
You reach for `unsafe` to do the small thing the checker can't verify — never to hand
yourself a free pass.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new advanced` works too.)* **Predict on paper before each run.**

1. **Two pointers, one value.** Declare a `let mut n = 1;`. Make a read-only raw
   pointer to it with `&raw const n` and a read-write one with `&raw mut n`. In an
   `unsafe { }` block, add some amount through the `*mut` pointer, then print the value
   through the `*const` pointer. **Predict** the number that prints, and **predict**
   what happens if you move the `*const` deref *outside* the `unsafe` block — which
   error code, and what's the one-line fix? Then check both.

2. **The five-superpowers recall.** Without looking back at part 2, write down (in
   words, not code) as many of the five `unsafe` superpowers as you can. Then check the
   list. The one you're most likely to forget tells you which to re-read. (No
   compiling here — this one's a memory check.)

3. **Overload `+` on your own struct.** Make a small struct holding two numbers (a
   point, a pair, a 2-D vector — your call). Implement `Add` for it so that `+`
   combines two of them field-by-field. **Predict** the output of adding two specific
   values *before* you run it. Then try implementing `Sub` as well so `-` works too —
   what does the trait/method change to? (Hint: it's the same shape as `Add`, in
   `std::ops`.)

4. **Bend the mini-macro.** Write a `macro_rules!` macro of your own — start by
   re-deriving the `my_vec!` skeleton from memory (don't copy part 3), get it building,
   then change *one* thing: make it push each element *twice*, or build a `Vec` of the
   *string* form of each element, or print a line per element instead of collecting.
   **Predict** the output for a 3-element call before running. Notice that the repetition
   `$( ... )*` in the body is what loops — there's no `for` in sight.

*(You write every line here — I won't. The predictions are your answer key; the code
is yours. The goal isn't fluency in any of these — it's recognition: you can now spot
an `unsafe` block as a small audited hole (not "C mode"), read an operator-overload
`impl`, and see how a `!`-macro turns one call into many lines of generated code.)*

## 6. What surprised you?

A sentence or two: did "`unsafe` unlocks five specific abilities and **doesn't** turn
off the borrow checker" land — or did you expect `unsafe { }` to mean "anything goes"?
Did seeing `+` resolve to an `Add::add` method call change how you read operators? And
did the mini-`my_vec!` make `vec![...]` feel less magical — code that writes code,
expanded before compilation? Tell me which of the three corners felt most worth
keeping, and I'll fold it into the Phase-9 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.20 "Advanced Features"**: §20.1
  Unsafe Rust (the five superpowers; raw pointers `*const T` / `*mut T`; dereferencing
  them inside `unsafe`; the `E0133` gate; the "second language hidden inside it…
  superpowers" framing, quoted), §20.2 Advanced Traits (associated types like
  `type Output`; operator overloading via `std::ops::Add`), and §20.5 Macros (the
  `macro_rules!` declarative macro and the `vec!` listing this lesson's `my_vec!` is
  adapted from).
- **CR** — *Comprehensive Rust* (Google): the responsible `unsafe` framing — the
  keyword **shifts the burden of upholding Rust's rules from the compiler to the
  programmer** (you keep the rules; you just promise it by hand).
- **BLOG** — not used here; this topic is BOOK-backed (it gives `unsafe` only a
  one-line passing mention).
- Every snippet compiled and run, and every error captured live, on **rustc 1.95.0**,
  **edition 2024** (`rustc --edition 2024 FILE.rs`). Notes on the live build: the
  `&raw const` / `&raw mut` raw-borrow operators (stable since 1.82) are used in place
  of the older `&num as *const i32` cast; and the "unsafe doesn't disable the borrow
  checker" point is shown with a real `E0502` firing right beside a genuinely-needed
  `unsafe` deref. This is the first lesson of **Phase 9 — Advanced**.

---

<!-- lesson-nav -->
[← Lesson 33 — Advanced Patterns & Matching](33-advanced-patterns.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 35 — Capstone: A Multithreaded Web Server →](35-capstone-web-server.md)
