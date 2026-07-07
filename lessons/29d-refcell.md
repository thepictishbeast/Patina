# Lesson 29d — `RefCell<T>`: interior mutability

*(Phase 7, smart pointers part 4 — the finale. Lesson 29c let many owners READ
one value. The last tool lets a shared value be CHANGED — by moving the borrow
check from compile time to run time.)*

## 1. Why it exists

Once a value is shared, the borrow rules forbid changing it — no `mut` handle
exists. Occasionally you're certain a change is safe but the compiler can't see
why. **`RefCell<T>`** is the escape valve: **interior mutability** — change a
value through a shared, immutable-looking handle. The rules (one mutable **or**
many immutable, never both) still apply — but `RefCell` checks them at
**runtime**. Break them and your program **panics** instead of failing to
compile.

When do you reach for it? Only when you **can't** just add `mut` — most often
because the value is shared through an `Rc`. For a plain local, `let mut` is
simpler and better.

## 2. The idea

- `.borrow()` — a temporary shared view (like `&`).
- `.borrow_mut()` — a temporary mutable view (like `&mut`).
- The counting happens as the program runs: two live `borrow_mut`s compile fine
  and **crash** — the same rule Lesson 16b's compiler enforced, now enforced by
  a panic.

## 3. Tiny examples to read

**Change a value through a shared handle.** Note `balance` is *not* `mut`:

```rust
use std::cell::RefCell;

fn main() {
    let balance = RefCell::new(100);   // not declared `mut`

    *balance.borrow_mut() += 50;       // change it anyway

    println!("balance = {}", balance.borrow());
}
```

```
balance = 150
```

On its own this is pointless — `let mut balance = 100` would be simpler.
`RefCell` earns its place in the closing move: sharing one *mutable* value among
several owners, by putting a `RefCell` **inside** an `Rc`:

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let balance = Rc::new(RefCell::new(100));

    let owner_a = Rc::clone(&balance);
    let owner_b = Rc::clone(&balance);

    *owner_a.borrow_mut() += 50;             // change through one owner

    println!("a sees {}", owner_a.borrow());
    println!("b sees {}", owner_b.borrow()); // the other owner sees it too
}
```

```
a sees 150
b sees 150
```

Two owners, one heap value, mutable — the thing no `let mut` can give you.
(`Rc<RefCell<T>>` is the combo worth remembering; the Book builds toward it
deliberately in §15.5, which also has the depth on when it's the right call.)

## 4. Common pitfalls / real runtime panics — the borrow rules at run time

Two live `borrow_mut`s compile fine, then crash when run:

```rust
use std::cell::RefCell;

fn main() {
    let balance = RefCell::new(100);

    let mut one = balance.borrow_mut();
    let mut two = balance.borrow_mut();   // second live borrow_mut

    *one += 50;
    *two += 50;
}
```

**Before you scroll — compile error, or something else?**

```
thread 'main' panicked at main.rs:7:27:
RefCell already borrowed
```

This is the trade-off `RefCell` makes: it accepts code the compiler would
reject, and moves the borrow check to runtime — a mistake that was a *compile*
error with `&mut` becomes a *crash*. (Third runtime-panic pattern of the course,
after 5b's overflow and 20b's unwrap — same predict-two-questions habit: will it
compile? will it *run*?) The matching exercise below is this wall.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new interior` works too.)* **Predict on paper
before each run.**

1. **Interior mutability, straight.** Make a `RefCell::new(0)`, add 5 through
   `borrow_mut`, print with `borrow`. **Predict** the output.
2. **Break it.** Hold **two** `borrow_mut` handles alive at once. **Predict**:
   compile error, or compile-and-panic? Then confirm — and notice the message
   names the rule you broke.
3. **The combo.** Build the `Rc<RefCell<..>>` two-owner example yourself with
   your own value; change through one owner, read through the other. **Predict**
   what the second owner sees.

*(You write every line here — I won't. The predictions are your answer key. That
closes Phase 7: closures · iterators · smart pointers — you can now reach past
single ownership when a real problem needs it. Next, Phase 9: threads.)*

## 6. What surprised you?

A sentence or two: did it surprise you that breaking `RefCell`'s rule is a
*crash* rather than a compile error — and does that change when you'd reach for
it versus a plain `&mut`? Tell me, and I'll fold it into the Phase-7 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§15.5**: `RefCell<T>`, interior
  mutability, runtime borrow checking, and the `Rc<RefCell<T>>` combination —
  including the when-to-use depth this lesson points at.
- **CR** — *Comprehensive Rust* (Google): the `RefCell` slide.
- Every snippet compiled and run, and the panic captured live, on
  **rustc 1.95.0**, edition 2024 (the panic prints `RefCell already borrowed`
  on 1.95.0; older Book printings show `already borrowed: BorrowMutError` —
  same failure).

---

<!-- lesson-nav -->
[← Lesson 29c — Rc: one value, many owners](29c-rc.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 30 — Threads, Channels & Shared State →](30-threads-and-concurrency.md)
