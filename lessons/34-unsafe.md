# Lesson 34 — `unsafe`: a small, audited escape hatch

*(Phase 9 — Advanced, part 6. Almost everything in Rust is checked for you —
that safety is the whole appeal. This lesson is about the keyword for the few
operations the compiler CAN'T verify: what `unsafe` actually unlocks, what it
absolutely does not, and why the block is kept small on purpose. It's an
escape hatch, not "C mode.")*

## 1. Why it exists

A few real operations can't be proven safe by the compiler: talking to
hardware, calling into C, following a hand-built pointer. The compiler can't
verify them, so it refuses them by default. **`unsafe`** is the keyword that
says "I've checked this by hand; let me through." It's a small, **auditable**
marker — not a different mode.

> **How the sources frame it:** the **BOOK** Ch.20 §20.1 opens: *"Rust has a
> second language hidden inside it… called unsafe Rust [that] gives us extra
> superpowers."* **CR** sharpens the responsible framing: the keyword **shifts
> the burden of upholding Rust's rules from the compiler to the programmer** —
> you're not switching the rules off, you're promising to keep them.

## 2. The idea

**`unsafe { }` is a marked block where five normally-forbidden abilities
("the five superpowers") become available.** The block is small on purpose:
it's the *only* place those abilities live, so when something goes wrong you
have a short list to audit.

The superpower you'll actually meet, and the one this lesson runs: **(1)
dereference a raw pointer.** A *raw pointer* (`*const T` read-only, `*mut T`
read-write) is a bare address with none of a reference's guarantees.
**Creating** one is safe; **following** one is the gated part. The other four
— calling `unsafe` functions, touching a mutable `static`, implementing an
`unsafe` trait, reading a `union`'s fields — are rarer still; know they exist
and find them in **Book Ch.20.1** when one crosses your path.

**Crucially, `unsafe` does not turn off the borrow checker.** Ownership,
borrowing, and type-checking all still apply inside the block. It unlocks
exactly the five extra abilities and **nothing else** — part 4 proves this
with a real error.

The honest summary: **you can write real Rust for a long time without any of
this.** The goal is recognition — an `unsafe` block is a tiny, audited hole
you open deliberately, never a way to silence the compiler wholesale.

## 3. A tiny example to read

**Dereferencing a raw pointer.** The `&raw const` / `&raw mut` operators make
raw pointers from a value; the `*r` derefs are the superpower and sit inside
the one `unsafe` block:

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

Both pointers refer to the same `num`, so the write through `r2` is visible
when we read through `r1`. Everything risky — the two `*` derefs — sits inside
the one `unsafe` block. That block is your audit boundary: if this program
ever misbehaves through a pointer, *this is the only place to look.* Outside
it, the compiler is back in charge.

## 4. Common pitfalls / real compiler errors

**Dereferencing a raw pointer *outside* `unsafe` — `E0133`.** Making the
pointer is fine; following it is the superpower, and it must be inside an
`unsafe` block:

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

The fix is to wrap the deref in `unsafe { }` (the working version is in
part 3). The note spells out *why* it's gated: a raw pointer carries none of a
reference's guarantees, so following one is on you to get right.

**`unsafe` does NOT switch off the borrow checker — `E0502`.** A common
misunderstanding is that `unsafe { }` is "anything goes." It isn't: ownership
and borrowing are checked exactly as everywhere else. Here the raw-pointer
deref genuinely needs `unsafe`, but the ordinary borrow violation right above
it still fails to compile:

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

This is the lesson's core point made concrete: `unsafe` unlocks **five
specific abilities** and *only* those. Borrow checking, ownership, and
type-checking stay on. You reach for `unsafe` to do the small thing the
checker can't verify — never to hand yourself a free pass.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new escape` works too.)* **Predict on paper
before each run.**

1. **Two pointers, one value.** Declare a `let mut n = 1;`. Make a read-only
   raw pointer to it with `&raw const n` and a read-write one with
   `&raw mut n`. In an `unsafe { }` block, add some amount through the `*mut`
   pointer, then print the value through the `*const` pointer. **Predict** the
   number that prints, and **predict** what happens if you move the `*const`
   deref *outside* the `unsafe` block — which error code, and what's the
   one-line fix? Then check both.
2. **The five-superpowers recall.** Without looking back at part 2, write down
   (in words, not code) as many of the five `unsafe` superpowers as you can.
   Then check the list. The one you forgot tells you which paragraph to
   re-read. (No compiling here — this one's a memory check.)
3. **Prove the checker stays on.** Inside an `unsafe { }` block, write an
   ordinary borrow violation — bind `let first = &v[0];` on a vector, then
   `v.push(...)`, then use `first`. **Predict**: does the `unsafe` block save
   it? Which error code fires, and is it any different from the one you'd get
   *without* `unsafe`?

*(You write every line here — I won't. The predictions are your answer key.
Next, a richer corner of the trait system you already know: making `+` work on
your own types — and finally paying off Lesson 31's IOU about `Output =`.)*

## 6. What surprised you?

A sentence or two: did "`unsafe` unlocks five specific abilities and
**doesn't** turn off the borrow checker" land — or did you expect `unsafe { }`
to mean "anything goes"? Does "audit boundary" change how you'd review code
with `unsafe` blocks in it? Tell me, and I'll fold it into the Phase-9 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.20 §20.1 "Unsafe Rust"**:
  the five superpowers (this lesson runs the first and points at the rest);
  raw pointers `*const T` / `*mut T`; dereferencing them inside `unsafe`; the
  `E0133` gate; the "second language hidden inside it… superpowers" framing,
  quoted.
- **CR** — *Comprehensive Rust* (Google): the responsible `unsafe` framing —
  the keyword **shifts the burden of upholding Rust's rules from the compiler
  to the programmer**.
- Every snippet compiled and run, and every error captured live, on **rustc
  1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`; temp paths
  normalized to `main.rs`). The `&raw const` / `&raw mut` raw-borrow operators
  (stable since 1.82) are used in place of the older `as *const` casts; the
  "unsafe doesn't disable the borrow checker" point is shown with a real
  `E0502` firing right beside a genuinely-needed `unsafe` deref.

---

<!-- lesson-nav -->
[← Lesson 33b — Guards, `@` bindings & nested patterns](33b-guards-bindings-nested.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 34b — Operator overloading & associated types →](34b-operator-overloading.md)
