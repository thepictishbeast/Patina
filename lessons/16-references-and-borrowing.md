# Lesson 16 — References & Borrowing

*(Phase 4 — Ownership, part 2. Lesson 15 left us with a clumsy choice: hand a value
away (move) or pay to copy it (`.clone()`). Borrowing is the way out — use a value
without taking it.)*

## 1. Why it exists

Most of the time you don't want to *own* a value — you just want to **look at it** or
**change it** and then leave it where it was. Lesson 15's `takes_ownership(s)` swallowed
`s` whole; cloning everything is wasteful. A **reference** lets a function (or another
part of your code) **borrow** a value: it can use it, but the original owner keeps it.
You write a reference with `&`.

> **How the sources frame it:** the **BOOK** states the two borrowing rules verbatim and
> shows the borrow-checker errors; **CR** has the one-line version of the rule (*shared
> **xor** mutable*) and the "borrow instead of move" payoff, and points out that a borrow
> ends at its **last use**. (Comparisons to other languages' pointers are dropped.)

## 2. The idea

**A shared reference, `&T` — borrow to read.** `&s` hands out a *view* of `s` without
moving it. This is the clean fix to last lesson's function-move problem:

```
fn announce(text: &String) {   // borrows, doesn't take
    println!("{text}");
}
…
announce(&s);   // lend s
// s is STILL valid here
```

You can have **as many** shared references as you like at once — they're read-only, so
no one steps on anyone.

**A mutable reference, `&mut T` — borrow to change.** Mark both the binding (`let mut s`)
and the reference (`&mut s`), and the borrower can modify the value through it:

```
fn add_excitement(text: &mut String) {
    text.push_str("!!!");
}
```

But there's a catch, and it's *the* rule of this lesson.

**The two rules of references** (straight from the Book):

1. At any given time, you can have **either one mutable reference or any number of
   immutable references** — not both.
2. References must **always be valid** (you can't return a reference to something that's
   already gone).

CR says rule 1 in five words: **shared *xor* mutable.** Either many readers, or one
writer, never a writer alongside readers. That's not Rust being fussy — it's exactly the
condition that rules out a whole class of bugs (something changing out from under you
while you're reading it).

One more thing that saves you confusion: a borrow is considered "active" only until its
**last use**, not until the end of the block. So two borrows that don't *overlap in use*
don't conflict — the compiler is precise about it.

> **The picture (Book):** it's ordinary borrowing. If someone owns a thing, you can borrow
> it, use it, and give it back — you never owned it, and they get it back unchanged (or, with
> `&mut`, changed exactly as agreed).

## 3. Tiny examples to read

**Borrow to read — the owner keeps it.** Predict both lines:

```rust
fn announce(text: &String) {
    println!("announcing: {text}");
}
fn main() {
    let s = String::from("hello");
    announce(&s);                  // lend it
    println!("still mine: {s}");   // still usable — never moved
}
```

```
announcing: hello
still mine: hello
```

**Borrow to change — `&mut` (you type this one, 30-second rep).** Predict the output:

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

`s` is owned by `main` the whole time; `add_excitement` just borrowed it long enough to
change it. *(That was your write-rep; part 5 is the rest.)*

## 4. Common pitfalls / real compiler errors

These three errors *are* Phase 4 — feel each one and the rules become muscle memory.

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

(Both `r1` and `r2` are *used* on the last line — that's what keeps both borrows active
and makes them collide. Drop the trailing use of one and there'd be no overlap.)

**A mutable borrow while a shared one is alive — `E0502`.** Rule 1 again — shared xor
mutable:

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

**A reference to something that's gone — `E0106`.** Rule 2: a reference must stay valid.
Returning a reference to a local fails, because the local is dropped when the function
ends:

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s   // s is freed when dangle() returns — this reference would dangle
}
```

```
error[E0106]: missing lifetime specifier
  …
  = help: this function's return type contains a borrowed value, but there is no value
          for it to be borrowed from
help: instead, you are more likely to want to return an owned value
```

The fix is exactly what the help says: **return the owned `String`** (`-> String`, and
`s` instead of `&s`) — hand ownership out, don't lend a reference to something about to
disappear.

> The borrow checker isn't blocking you for sport. Each of these is a real bug — two
> writers racing, a write under a reader, a pointer to freed memory — caught at build
> time, at the exact line, instead of crashing later.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new borrowing`. **Predict on paper before each run.**

1. **Lend, don't give.** Write a function that takes `&String` and prints its `.len()`.
   Call it on a `String`, then print the `String` again afterward. **Predict**: does it
   compile? Is the `String` still usable?

2. **Borrow mutably.** Write a function taking `&mut String` that appends some text. Call
   it (remember both `mut`s), then print the result. **Predict** the output.

3. **Break a rule on purpose.** Take two `&mut` of the same value and use both. **Predict**
   the error code. Then change one to a shared `&` and use both — **predict** which error
   you get *now* (different code?). Run both and read what the compiler points at.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. Next lesson: slices in depth — where a borrowed view turns out to *prevent* a whole
category of bug.)*

## 6. What surprised you?

A sentence or two: did "shared xor mutable" feel restrictive or sensible once you saw the
errors? Did "a borrow ends at its last use" clear anything up? Tell me, and I'll pitch
Lesson 17 (slices in depth) to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §4.2 "References and Borrowing." The two rules
  verbatim, the `&`/`&mut` mechanics, the two-`&mut` `E0499`, the `&mut`-while-`&` `E0502`,
  the dangling-reference `E0106`, and the "borrowing is like real-life borrowing" framing.
- **CR** — *Comprehensive Rust* (Google), §9.1–9.2 and §23. The one-line rule "shared **xor**
  mutable," the "borrow instead of move" motivation, and the note that a borrow ends at its
  **last use** (so non-overlapping borrows don't conflict). Its pointer comparisons to other
  languages were dropped per the no-analogy rule.
- **BLOG** — shows `&`/`&mut` in passing but never states the rules — sourced from BOOK/CR.
- Compiler output captured live on **rustc 1.95.0** (edition 2024); the borrow errors are
  edition-independent. Next: the slice type in depth (Lesson 17).

---

<!-- lesson-nav -->
[← Lesson 15 — Ownership & Moves](15-ownership-and-moves.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 17 — Slices in Depth →](17-slices-in-depth.md)
