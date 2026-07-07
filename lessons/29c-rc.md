# Lesson 29c — `Rc<T>`: one value, many owners

*(Phase 7, smart pointers part 3. Ownership's first rule — ONE owner — meets the
situations where two parts of a program genuinely share a value, and neither
should clean it up until both are done.)*

## 1. Why it exists

Single ownership can't express "we both own this." Sometimes that's exactly the
truth of a program: two structures point at the same data, and the data must
outlive *whichever* of them lives longer. **`Rc<T>`** (Rc = *reference counted*)
makes multiple ownership safe by counting: every new owner adds one; every owner
that goes out of scope subtracts one (that's Lesson 29b's `Drop` hook at work);
at **zero**, the value is cleaned up — not before.

## 2. The idea

`Rc::clone(&a)` makes another owner and bumps the count. It is **cheap** — it
never copies the data, it just adds 1. `Rc::strong_count(&a)` lets you watch the
count move.

> **The metaphor (BOOK, §15.4):** *imagine `Rc<T>` as a TV in a family room.
> When one person enters to watch TV, they turn it on. Others can come into the
> room and watch the TV. When the last person leaves the room, they turn off the
> TV because it's no longer being used.* The data is the TV; each owner is a
> person in the room; the value is cleaned up only when the **last** owner
> leaves.

One limit to know now: everyone in the room can *watch* the TV — `Rc` hands out
**shared (read-only) access**. Changing the shared value needs one more tool,
and it's the next lesson.

## 3. A tiny example to read

`Rc::clone` adds an owner; the count falls when an owner goes out of scope:

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared text"));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("c reads: {c}");
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("b still reads: {b}");
}
```

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
c reads: shared text
count after c goes out of scope = 2
b still reads: shared text
```

The value isn't cleaned up while any owner remains: the last person leaves
before the TV goes off.

## 4. Common pitfalls — reaching for a move when you meant to share

The wall this lesson's exercise hands you: build a value, hand it to one owner
by *moving* it (Lesson 15), then try to use it from somewhere else — the E0382
you know. The fix isn't a clone of the *data* (wasteful) — it's `Rc::new` once,
then `Rc::clone` per owner: many owners, one heap value, count doing the
bookkeeping. **Predict the error code** in the exercise before you run, then
read how the `Rc` version differs.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new sharing` works too.)* **Predict on paper
before each run.**

1. **Watch a count rise and fall.** Make an `Rc::new(...)` holding any value.
   Print `Rc::strong_count` after creating it, after one `Rc::clone`, inside an
   inner `{ }` block with a *second* clone, and again *after* the block ends.
   **Predict the four numbers** before running. (Re-read the TV metaphor if the
   last one is unclear.)
2. **Prove the clone is cheap.** Put a large-ish `String` in the `Rc` and clone
   it three times. **Predict**: is the text copied three times, or counted?
   What does `Rc::clone`'s name *not* mean here?

*(You write every line here — I won't. The predictions are your answer key.
Next: changing a value everyone shares — `RefCell` and interior mutability.)*

## 6. What surprised you?

A sentence or two: did the count going `1 → 2 → 3 → 2` make the TV metaphor
click? Did "Rc::clone is a count bump, not a data copy" change how the word
*clone* reads? Tell me, and I'll pitch Lesson 29d to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§15.4**: `Rc<T>`, `Rc::clone`,
  `Rc::strong_count`, and the TV-in-a-family-room metaphor (quoted).
- **CR** — *Comprehensive Rust* (Google): "`Rc::clone` is cheap — it just bumps
  the count."
- Every snippet compiled and run on **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 29b — Deref & Drop](29b-deref-drop.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 29d — RefCell: interior mutability →](29d-refcell.md)
