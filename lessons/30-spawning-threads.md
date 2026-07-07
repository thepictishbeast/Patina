# Lesson 30 — Spawning threads: `spawn`, `join` & `move`

*(Phase 8 — Concurrency, the opener. Until now your programs did **one thing at a
time**, top to bottom. A **thread** lets a second line of work run *alongside* the
first — and Rust's ownership rules turn the classic shared-data bugs into compile
errors. The Book calls this "fearless concurrency.")*

## 1. Why it exists

A single thread does its work in order. But a lot of real work is naturally
*parallel* — handling several requests, splitting a big job across CPU cores,
keeping a UI responsive while something slow runs. For that you want **more than
one thread of execution** at once. The catch is shared data: two threads touching
the same value with no coordination is a **data race**, a bug that appears only
sometimes. Rust's answer is the one you already know — ownership — enforced at
compile time, starting with this lesson's rule: *a thread owns what it uses*.

## 2. The idea

**Spawning.** `thread::spawn(closure)` starts the closure on a new thread and
returns a **`JoinHandle`**. The new thread runs *independently* — its lines
interleave with the main thread's in an order you don't control. Calling
`.join()` on the handle **blocks** until that thread finishes, so you can be
sure its work is done.

**`move`.** A spawned thread might outlive the function that started it, so any
data the closure uses must be **owned** by the closure, not borrowed (this is
Lesson 27b's `move`, now with its sharpest motivation). Without `move` the
closure would only borrow — and the compiler rejects that, because it can't
prove the borrowed data lives long enough (part 4).

## 3. Tiny examples to read

**A thread, spawned and joined.** The two `for` loops run at the same time, so
their lines interleave — the output below is **one possible run; it varies**:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("spawned: {i}");
        }
    });

    for i in 1..=3 {
        println!("main: {i}");
    }

    handle.join().unwrap();
}
```

```
main: 1
main: 2
main: 3
spawned: 1
spawned: 2
spawned: 3
```

Run it a few times — the interleaving shifts. That unpredictability *is*
concurrency. The one thing `join` guarantees: by the time `main` ends, the
spawned thread has finished.

**`move` — give the thread ownership of data:**

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("the thread owns: {data:?}");
    });

    handle.join().unwrap();
}
```

```
the thread owns: [1, 2, 3]
```

After `move`, `data` belongs to the thread — `main` can't use it again.

## 4. Common pitfalls / real compiler errors — forgetting `move`

Without `move`, the closure only *borrows* `v`, but the thread might outlive
`main`, so the borrow could dangle:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```

```
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
 --> main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("here's a vector: {v:?}");
  |                                     - `v` is borrowed here
  |
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

Add `move` (as in part 3) and it compiles — the thread *owns* `v`, so there's
nothing to dangle. The matching exercise below is this wall: **predict the
code** before you run.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new threads` works too.)* **Predict on paper
before each run** — for the interleaving one, predict what's *guaranteed* versus
what's *not*.

1. **Spawn and join.** Spawn a thread that prints `"in thread"` three times;
   have `main` print `"in main"` three times; join at the end. **Predict:** is
   the exact interleaving guaranteed? What single thing *is* guaranteed once
   you've called `join`? Run it several times and watch the order shift.
2. **Forget `move`, then add it.** A `vec![1, 2, 3]`, a spawned closure printing
   it — *without* `move`. **Predict** the error code, read the `help:` line, add
   `move`, and name what changed about who owns the vector.

*(You write every line here — I won't. The predictions are your answer key.
Next: handing data BETWEEN threads — channels.)*

## 6. What surprised you?

A sentence or two: did the run-to-run interleaving surprise you? Did `move` here
feel like Lesson 27b's keyword finally meeting its real customer? Tell me, and
I'll pitch Lesson 30b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§16.1**: `thread::spawn`,
  `JoinHandle`/`join`, and `move` closures capturing data into a thread.
- **CR** — *Comprehensive Rust* (Google): the Threads slides.
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024. The interleaved output is *one* run and will
  differ on reruns.

---

<!-- lesson-nav -->
[← Lesson 29d — RefCell: interior mutability](29d-refcell.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 30b — Channels: passing data between threads →](30b-channels.md)
