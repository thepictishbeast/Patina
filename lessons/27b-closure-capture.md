# Lesson 27b — Closure capture & `move`

*(Phase 7, part 2. Lesson 27 wrote closures that used only their own arguments.
The real power — and the whole reason they're called *closures* — is that they
can reach out and grab the variables around them. That grabbing leans on
everything from Lessons 15–16b.)*

## 1. Why it exists

A plain `fn` can't see the local variables sitting next to it. A closure can:
it **captures** what its body uses, so the values it needs are already in hand.
The question this lesson answers: *how* does it grab each value — and who owns
what afterwards? The answer is the ownership story you already know, replayed.

## 2. The idea

**Capturing mirrors the three ways a function takes a parameter:**

- **Borrow immutably** — the body only *reads* the value. The original stays
  usable.
- **Borrow mutably** — the body *changes* the value (e.g. pushes to a vector).
- **Take ownership** — the value is *moved* into the closure.

The compiler picks the **least demanding** option the body needs. You don't ask
for a mode — **the body decides**.

**The `move` keyword** is the one override: put `move` before the bars and the
closure takes **ownership** of everything it captures, even if the body would
have been happy borrowing. You reach for it when the closure must *outlive* the
spot where you wrote it — returning it from a function, storing it for later, or
(Lesson 30) sending it to another thread. A borrow can't be guaranteed to stay
valid in those cases; `move` says "the closure keeps these values — they go with
it." (The full story, including the `Fn`/`FnMut`/`FnOnce` trait names that
describe which capture a closure uses, is Book Ch. 13.1.)

## 3. Tiny examples to read

**Capture by immutable borrow — the value stays usable:**

```rust
fn main() {
    let list = vec![1, 2, 3];
    let only_borrows = || println!("from closure: {list:?}");
    println!("before call: {list:?}");
    only_borrows();
    println!("after call:  {list:?}");
}
```

```
before call: [1, 2, 3]
from closure: [1, 2, 3]
after call:  [1, 2, 3]
```

**Capture by mutable borrow — the body changes the value.** Note `list` is
`mut`, and the *closure variable* is `mut` too (it holds a mutable borrow):

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    let mut adds_one = || list.push(7);
    adds_one();
    println!("after call: {list:?}");
}
```

```
after call: [1, 2, 3, 7]
```

**Force ownership with `move`.** Same printing body as the first example — the
only change is `move`, which switches the capture from "borrow" to "own":

```rust
fn main() {
    let list = vec![1, 2, 3];
    let owns_it = move || println!("owned by closure: {list:?}");
    owns_it();
}
```

```
owned by closure: [1, 2, 3]
```

It runs — but `main` no longer owns `list`. What happens if `main` tries to use
it afterwards is part 4's first wall.

## 4. Common pitfalls / real compiler errors

**Using a value after `move` took it — `E0382`.** Ownership transferred into the
closure; the old name is dead:

```rust
fn main() {
    let list = vec![1, 2, 3];
    let owns_it = move || println!("from closure: {list:?}");
    owns_it();
    println!("back in main: {list:?}");
}
```

```
error[E0382]: borrow of moved value: `list`
 --> main.rs:5:30
  |
2 |     let list = vec![1, 2, 3];
  |         ---- move occurs because `list` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     let owns_it = move || println!("from closure: {list:?}");
  |                   -------                          ---- variable moved due to use in closure
  |                   |
  |                   value moved into closure here
4 |     owns_it();
5 |     println!("back in main: {list:?}");
  |                              ^^^^ value borrowed here after move
  |
help: consider cloning the value before moving it into the closure
```

This is Lesson 15's rule triggered by `move`. Drop the `move` (the closure
borrows and `list` stays usable), or clone first if you genuinely need both. The
matching exercise below is this wall — **predict the code** before you run.

**Moving a captured value *out* of a reusable closure — `E0507`.**
`sort_by_key` calls its closure once per element, so the closure must be
callable repeatedly. This one tries to *give away* the captured `value` on each
call — which can only happen once:

```rust
fn main() {
    let mut list = [3, 1, 2];
    let mut log = Vec::new();
    let value = String::from("sorted");
    list.sort_by_key(|n| {
        log.push(value);
        *n
    });
    println!("{list:?}");
}
```

```
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  |
6 |         log.push(value);
  |                  ^^^^^ `value` is moved here
  |
help: consider cloning the value if the performance cost is acceptable
```

Read "an `FnMut` closure" as "a closure that must be callable repeatedly" — the
trait names are the Book's Ch. 13.1. The suggested `value.clone()` hands the
closure its own copy each call, so nothing is moved out of the environment.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new capture` works too.)* **Predict on paper
before each run.**

1. **The three capture modes, by hand.** Over a `let mut v = vec![10, 20, 30]`,
   write: a closure that only *prints* `v`; one that *pushes* onto `v`; one with
   `move` that prints `v`. After calling each, try `println!("{v:?}")`.
   **Predict** which of the three leaves `v` usable — one will refuse to
   compile; predict which, and the error code.
2. **Fix it two ways.** Take the failing `move` case: fix it once by removing
   `move`, once by cloning before the closure. **Predict** which fix matches
   "share the value" vs "hand it over."

*(You write every line here — I won't. The predictions are your answer key. The
thing to feel: you never *told* the closure how to capture — the body did, and
`move` was the one override. Next: iterators — where closures become the
everyday tool.)*

## 6. What surprised you?

A sentence or two: did it click that the body picks the capture mode and the
compiler takes the lightest one? Did `move` feel like Lesson 15's rule in a new
place? Tell me, and I'll fold it into the Phase-7 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.13.1**: the three capture
  modes mapped to the three ways a function takes a parameter (Listings 13-4/
  13-5, and 13-6's `move` — shown there with a thread; the thread itself is our
  Lesson 30), and the `E0507` move-out-of-an-`FnMut`-closure failure
  (Listing 13-8).
- **CR** — *Comprehensive Rust* (Google), closures section: the trait-family
  framing (left to the Book here).
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024 (one environment-specific stdlib path line
  trimmed from the E0507 output; code, message, and spans verbatim).

---

<!-- lesson-nav -->
[← Lesson 27 — Closures: unnamed inline functions](27-closure-syntax.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 28 — Iterators: the next() cursor →](28-iterator-cursor.md)
