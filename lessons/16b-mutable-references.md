# Lesson 16b — Mutable references & the borrowing rules

*(Phase 4 — Ownership, part 3. Lesson 16 lent values out to be READ. Now: lending a
value out to be CHANGED — and the two rules that make lending safe at all.)*

## 1. Why it exists

A read-only view can't help a function that needs to *modify* your value in place.
For that there's a second kind of borrow — `&mut` — and with it comes **the** rule
of this phase, the one the borrow checker exists to enforce. The three errors in
this lesson *are* Phase 4: feel each one and the rules become muscle memory.

## 2. The idea

**A mutable reference, `&mut T` — borrow to change.** Mark both the binding
(`let mut s`) and the reference (`&mut s`), and the borrower can modify the value
through it:

```
fn add_excitement(text: &mut String) {
    text.push_str("!!!");
}
```

But there's a catch.

**The two rules of references** (straight from the Book):

1. At any given time, you can have **either one mutable reference or any number of
   immutable references** — not both.
2. References must **always be valid** — a reference cannot outlive the value it
   points at.

CR says rule 1 in three words: **shared *xor* mutable.** Either many readers, or
one writer — never a writer alongside readers. That's not Rust being fussy; it's
exactly the condition that rules out a whole class of bugs (something changing out
from under you while you're reading it).

One nuance that saves confusion: a borrow is "active" only until its **last use**,
not until the end of the block — so borrows that don't overlap in use don't
conflict. (The fine print is in the Book, §4.2, and CR §23.)

## 3. A tiny example to read (you type this one, 30-second rep)

Predict the output:

```rust
fn add_excitement(text: &mut String) {
    text.push_str("!!!");
}
fn main() {
    let mut s = String::from("wow");
    add_excitement(&mut s);   // lend it mutably
    println!("{s}");
}
```

```
wow!!!
```

`s` is owned by `main` the whole time; `add_excitement` just borrowed it long
enough to change it. (Forget either `mut` and the compiler names the missing one —
that's one of the exercises below.)

## 4. Common pitfalls / real compiler errors

**Two mutable borrows at once — `E0499`.** Rule 1 says only one `&mut`:

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{r1}, {r2}");
}
```

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> main.rs:4:14
  |
3 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
4 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
5 |     println!("{r1}, {r2}");
  |                -- first borrow later used here
```

(Both `r1` and `r2` are *used* on the last line — that's what keeps both borrows
active and makes them collide.)

**A mutable borrow while a shared one is alive — `E0502`.** Rule 1 again — shared
xor mutable:

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;        // a reader
    let r2 = &mut s;    // a writer — not allowed while r1 is live
    println!("{r1}, {r2}");
}
```

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> main.rs:4:14
  |
3 |     let r1 = &s;
  |              -- immutable borrow occurs here
4 |     let r2 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
5 |     println!("{r1}, {r2}");
  |                -- immutable borrow later used here
```

**A reference that outlives its value — `E0597`.** Rule 2. Here the borrowed value
lives inside a block that ends first:

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;      // borrow x…
    }                // …but x is dropped here
    println!("{r}");
}
```

```
error[E0597]: `x` does not live long enough
 --> main.rs:5:13
  |
4 |         let x = 5;
  |             - binding `x` declared here
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     println!("{r}");
  |                - borrow later used here
```

Read it bottom-up: the borrow is *used* after `x` is *dropped* — so the reference
would point at nothing. Rust refuses at build time. (The same rule stops a function
from returning a reference to its own local — you'll meet that in the exercises,
and the machinery for *describing* reference lifetimes properly arrives in
Lesson 26.)

> The borrow checker isn't blocking you for sport. Each of these is a real bug —
> two writers racing, a write under a reader, a pointer to freed memory — caught
> at build time, at the exact line, instead of crashing later.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then work through the matching
exercises via the **Practice this lesson** links at the bottom — this lesson has
the deepest exercise set in the course. *(On your own machine, a playground or
`cargo new borrowing` works too.)* **Predict on paper before each run.**

1. **Borrow mutably.** Write a function taking `&mut String` that appends some
   text. Call it (remember both `mut`s), then print the result. **Predict** the
   output.
2. **Break rule 1 on purpose.** Take two `&mut` of the same value and use both.
   **Predict** the error code. Then change one to a shared `&` and use both —
   **predict** which error you get *now* (different code?).
3. **Break rule 2 on purpose.** Borrow a value inside a `{ }` block, store the
   reference outside it, and use it after the block. **Predict**: which line does
   the compiler blame — the borrow, the drop, or the use?

*(You write every line here — I won't. The predictions are your answer key. Next
lesson: slices in depth — where a borrowed view turns out to *prevent* a whole
category of bug.)*

## 6. What surprised you?

Did "shared xor mutable" feel restrictive or sensible once you saw the errors?
Did reading E0597 bottom-up (use → drop → borrow) work for you? Tell me, and I'll
pitch Lesson 17 (slices in depth) to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §4.2 "References and Borrowing." The
  two rules verbatim, the `&mut` mechanics, the two-`&mut` `E0499`, the
  `&mut`-while-`&` `E0502`, and the dangling-reference discussion (taught here via
  the scope-based `E0597`, which needs no lifetime syntax; annotations come in
  Lesson 26).
- **CR** — *Comprehensive Rust* (Google), §9.2 and §23. The one-line rule "shared
  **xor** mutable" and the borrow-ends-at-last-use note.
- Compiler output captured live on **rustc 1.95.0** (edition 2024); the borrow
  errors are edition-independent.

---

<!-- lesson-nav -->
[← Lesson 16 — Shared references](16-shared-references.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 17 — Slices in Depth →](17-slices-in-depth.md)
