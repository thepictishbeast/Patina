# Lesson 30c — Shared state: `Arc` & `Mutex`

*(Phase 8, part 3 — the concurrency keystone. Channels pass values along.
Sometimes threads genuinely must share ONE mutable value — and the compiler
walks you, error by error, to the right design.)*

## 1. Why it exists

Ten threads all incrementing one counter can't each own a copy — they need the
*same* value. Sharing mutable data across threads is exactly where data races
live, so Rust demands two guarantees at once: many threads may **own** a handle
to the value, and only one at a time may **touch** it. Two types deliver them:

- **`Mutex<T>`** (*mutual exclusion*) guards the value. `.lock()` blocks until
  no one else holds the lock, then hands you a guard; the lock releases
  automatically when the guard goes out of scope (Lesson 29b's `Drop`).
- **`Arc<T>`** (*atomically reference counted*) is the thread-safe sibling of
  Lesson 29c's `Rc`: same job — many owners of one value — but its count is
  updated safely across threads.

The pairing **`Arc<Mutex<T>>`** is the one to memorise: `Arc` lets many threads
*own* it; `Mutex` lets them *change* it without racing.

## 2. The idea — the compiler teaches the design

The Book's keystone move is an **error-driven arc**, and it's worth knowing in
outline before you walk it yourself (part 5):

1. **A bare `Mutex` moved into the loop → `E0382`.** The first thread takes the
   only copy; iteration two has nothing. You need many owners.
2. **Wrap it in `Rc` (Lesson 29c's many-owners tool) → `E0277`:**
   *"`Rc<Mutex<i32>>` cannot be sent between threads safely … the trait `Send`
   is not implemented."* `Rc` bumps its count without synchronisation — two
   threads cloning the same `Rc` would corrupt it, the very race Rust exists to
   prevent.
3. **Switch to `Arc` → it compiles**, and prints the same answer every run.

Behind step 2 sit two marker traits the compiler checks silently: **`Send`** (a
value may *move to* another thread) and **`Sync`** (it may be *shared by
reference* across threads). Most types are both; `Rc` is neither; `Arc` is both.
(The full walkthrough with complete error dumps is Book §16.3, and the
`Send`/`Sync` story §16.4 — this lesson keeps the outline, the exercises give
you the walls.)

## 3. A tiny example to read

Ten threads each add 1 to the same counter (the Book's Listing 16-15):

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

```
Result: 10
```

`Result: 10` **every** time — not sometimes 9. The `Mutex` serialises the ten
increments; the `Arc` is what lets all ten threads own the same `Mutex` to begin
with. (`lock()` returns a `Result` — it fails only if another thread panicked
while holding the lock — so we `unwrap`; `num` is a guard that releases the lock
at the closure's end.)

## 4. Common pitfalls / real compiler errors — the two walls, in brief

**Wall 1 — `E0382`** (bare `Mutex`, moved into the first thread): *"borrow of
moved value: `counter` … value moved into closure here, in previous iteration
of loop."* The compiler is saying *you need many owners*.

**Wall 2 — `E0277`** (`Rc` across threads): *"`Rc<Mutex<i32>>` cannot be sent
between threads safely … the trait `Send` is not implemented."* Many owners,
yes — but `Rc`'s count isn't thread-safe. `Arc` is the same idea with an atomic
count.

And a third flavour the matching exercise hands you: **`Arc` alone, without the
`Mutex`** — many owners, but no licence to *mutate* — fails with `E0594`
("cannot assign"). All three walls push toward the same design: `Arc` for the
owning, `Mutex` for the changing.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the two matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new sharedstate` works too.)* **Predict on paper
before each run.**

1. **Walk the whole arc, deliberately.** Build the ten-thread counter in three
   stages: **(a)** a bare `Mutex::new(0)` moved into the loop — **predict the
   error code**; **(b)** wrapped in `Rc::new` with `Rc::clone` per thread —
   **predict the new code**, and read its `Send` line; **(c)** switch `Rc` to
   `Arc` — **predict the printed number**, and whether it's *always* that. Run
   it several times.
2. **In one sentence each:** what does `Arc` provide that `Rc` didn't? What does
   `Mutex` provide on top of `Arc`?

*(You write every line here — I won't. The predictions are your answer key. The
point of the arc: the compiler taught you the design — E0382 said "many owners,"
E0277 said "not `Rc` across threads," and `Arc<Mutex<T>>` is what both errors
were pushing you toward. Next: `async`/`await` — waiting without threads.)*

## 6. What surprised you?

A sentence or two: did the two-failure arc make `Arc<Mutex<T>>` feel like the
answer the compiler steered you to, rather than a thing to memorise? Did it
surprise you that `Result: 10` never varies while thread interleaving always
does? Tell me, and I'll fold it into the Phase-8 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§16.3** (the shared-counter
  arc: `Mutex<T>`, the E0382 and E0277 failures with full dumps, and
  `Arc<Mutex<T>>` — Listing 16-15, reproduced here) and **§16.4** (`Send` and
  `Sync`, the marker traits behind it all).
- **CR** — *Comprehensive Rust* (Google): the Shared State + `Send`/`Sync`
  slides.
- Every snippet compiled and run on **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 30b — Channels](30b-channels.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 31 — Async syntax: async fn, .await & async blocks →](31-async-syntax.md)
