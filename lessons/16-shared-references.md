# Lesson 16 — Shared references: borrow to *read*

*(Phase 4 — Ownership, part 4. Lessons 15–15c left us with a clumsy choice: hand a value
away (move) or pay to copy it (`.clone()`). Borrowing is the way out — use a value
without taking it.)*

## 1. Why it exists

Most of the time you don't want to *own* a value — you just want to **look at it**
and leave it where it was. Lesson 15's `takes_ownership(s)` swallowed `s` whole;
cloning everything is wasteful. A **reference** lets a function (or another part of
your code) **borrow** a value: it can use it, but the original owner keeps it. You
write a reference with `&`.

## 2. The idea

**A shared reference, `&T` — borrow to read.** `&s` hands out a *view* of `s`
without moving it. This is the clean fix to last lesson's function-move problem:

```
fn announce(text: &String) {   // borrows, doesn't take
    println!("{text}");
}
…
announce(&s);   // lend s
// s is STILL valid here
```

You can have **as many** shared references as you like at once — they're read-only,
so no one steps on anyone.

And that word *read-only* is precise: a shared borrow is a **view, not ownership**.
Through a `&` you can look at the value, call read-only methods on it, print it —
but you cannot *take* it, and you cannot change it. Taking or changing needs
something more, and that "more" is exactly the next lesson.

> **The picture (Book):** it's ordinary borrowing. If someone owns a thing, you can
> borrow it, use it, and give it back — you never owned it, and they get it back
> unchanged.

## 3. A tiny example to read

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

## 4. Common pitfalls / real compiler errors — you can look, not take

What happens if you try to *take ownership* through a borrow? Predict this one:

```rust
fn steal(text: &String) {
    let mine: String = *text;   // try to move the value out of the borrow
    println!("{mine}");
}
fn main() {
    let s = String::from("hello");
    steal(&s);
}
```

**Before you scroll — does this compile?**

No — `error[E0507]: cannot move out of `*text` which is behind a shared reference`.
The compiler is stating this lesson's whole idea back at you: the function only
*borrowed* `s`; the value still belongs to `main`, so it can't be moved out through
the view. If a function genuinely needs its own copy, that's what `.clone()` was
for (Lesson 15) — a borrow is for using, not keeping.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new borrowing` works too.)* **Predict on paper
before each run.**

1. **Lend, don't give.** Write a function that takes `&String` and prints its
   `.len()`. Call it on a `String`, then print the `String` again afterward.
   **Predict**: does it compile? Is the `String` still usable?
2. **Try to keep it.** Inside that same function, try binding the borrowed value
   to a new owned `String` (no `.clone()`). **Predict the error code** before
   running, and read *whose* value the compiler says it is.

*(You write every line here — I won't. The predictions are your answer key. Next
lesson: borrowing to **change** — and the two rules the borrow checker enforces.)*

## 6. What surprised you?

A sentence or two: did "a borrow is a view, not ownership" match your intuition?
Did the E0507 message read the way you expected? Tell me, and I'll pitch
Lesson 16b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §4.2 "References and Borrowing." The
  `&` mechanics, the "borrowing is like real-life borrowing" framing.
- **CR** — *Comprehensive Rust* (Google), §9.1. The "borrow instead of move"
  motivation. (Pointer comparisons to other languages dropped per the no-analogy
  rule.)
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 15c — Copy & Clone: when assignment duplicates](15c-copy-and-clone.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 16b — Mutable references & the borrowing rules →](16b-mutable-references.md)
