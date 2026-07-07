# Lesson 30 — Threads, Channels & Shared State

*(Phase 8 — Concurrency, the opening lesson. Until now your programs did **one thing
at a time**, top to bottom. A **thread** lets a second line of work run *alongside*
the first. That sounds dangerous — two pieces of code touching the same data is how
programs corrupt themselves in other languages — but Rust's ownership rules (L15) and
the `Send`/`Sync` traits turn most of those bugs into **compile errors**. The book
calls this "fearless concurrency": the compiler won't let you ship the classic data
race. This lesson covers the three tools you reach for first — spawning threads,
passing data through **channels**, and sharing one value with `Arc<Mutex<T>>`.)*

## 1. Why it exists

A single thread does its work in order: line, then the next line, then the next. But a
lot of real work is naturally *parallel* — handling several requests, splitting a big
job across CPU cores, keeping a UI responsive while something slow runs in the
background. For that you want **more than one thread of execution** at once.

The catch is shared data. If two threads read and write the same value at the same
time, with no coordination, you get a **data race**: the value ends up garbage, and the
bug appears only sometimes, which makes it brutal to track down. Many languages leave
you to avoid this by hand. Rust doesn't — the same ownership and borrowing rules that
prevent use-after-free at compile time *also* prevent data races at compile time. Code
that would race usually **won't compile**, and the error tells you what's wrong.

This lesson gives you the three first tools:

- **Threads** — start a second line of work with `thread::spawn`, wait for it with `join`.
- **Channels** — hand data *from* one thread *to* another safely, like a one-way pipe.
- **Shared state** — let several threads touch one value, guarded by a lock.

> **How the sources frame it:** the **BOOK** Ch.16 "Fearless Concurrency" is the spine.
> It teaches the shared-counter as an *error-driven arc* — the working version compiles
> only after the compiler rejects two wrong attempts (you'll walk that exact arc in part
> 4). **CR** (its Concurrency chapters) is the support: crisp reference slides on
> threads, channels, `Send`/`Sync`, and shared state. **BLOG** doesn't cover concurrency.

## 2. The idea

**Spawning.** `thread::spawn(closure)` starts the closure running on a new thread and
returns a **`JoinHandle`**. The new thread runs *independently* — its lines interleave
with the main thread's in an order you don't control. Calling `.join()` on the handle
**blocks** until that thread finishes, so you can be sure its work is done.

**`move`.** A spawned thread might outlive the function that started it, so any data the
closure uses must be **owned** by the closure, not borrowed. You write `move ||` to move
the captured data *into* the thread. Without `move`, the closure would only borrow — and
the compiler rejects that, because it can't prove the borrowed data lives long enough.

**Channels — passing data.** A channel is a one-way pipe between threads. `mpsc::channel()`
gives you a pair: a **transmitter** `tx` and a **receiver** `rx` (mpsc = *multiple
producer, single consumer*). One thread calls `tx.send(value)`; another calls `rx.recv()`
to wait for and take it. `send` **moves** the value — once sent, the sending thread no
longer owns it, so two threads can never touch the same value. You can clone `tx` to get
**several** senders feeding one receiver.

**Shared state — `Arc<Mutex<T>>`.** Sometimes threads must share *one* mutable value, not
pass copies. Two pieces fit together:

- **`Mutex<T>`** (*mutual exclusion*) guards the value. To touch it you call `.lock()`,
  which blocks until no one else holds the lock, then hands you a guard. While you hold
  it, no other thread can — so changes can't collide. The lock releases automatically
  when the guard goes out of scope (that's `Drop`, from L29).
- **`Arc<T>`** (*atomically reference counted*) gives the `Mutex` **multiple owners**, one
  per thread. It's the thread-safe sibling of `Rc<T>` from L29: same job — share one
  value among many owners — but its count is updated safely across threads.

The pairing **`Arc<Mutex<T>>`** is the one to memorise: `Arc` lets many threads *own* the
value; `Mutex` lets them *change* it without racing.

## 3. Tiny examples to read

**A thread, spawned and joined.** The spawned thread and `main` run at the same time, so
their lines interleave — the order below is **one possible run; it varies run to run**.
`join` makes `main` wait for the thread to finish:

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

Run it a few times — you may see the two `for` loops interleave differently. That
unpredictability *is* concurrency. The one thing `join` guarantees: by the time `main`
ends, the spawned thread has finished.

**`move` — give the thread ownership of data.** The closure needs `data`, and the thread
may outlive `main`'s frame, so `move` hands ownership in:

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

After `move`, `data` belongs to the thread — `main` can't use it again. (Forget `move`
and it won't compile; that's the first pitfall in part 4.)

**A channel — pass values from one thread to another.** The spawned thread `send`s four
strings; `main` receives them by looping over `rx`. The loop ends on its own when `tx` is
dropped (the sending thread finished). With one sender, order is preserved:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for m in messages {
            tx.send(m).unwrap();
        }
    });

    for received in rx {
        println!("got: {received}");
    }
}
```

```
got: hi
got: from
got: the
got: thread
```

Each `send` **moves** its string into the channel, so the sending thread can't touch it
afterward — the type system, not your discipline, prevents the two threads from sharing
the same value.

**`Arc<Mutex<T>>` — many threads, one shared counter.** Ten threads each add 1 to the same
number. `Arc::clone` gives each thread an owner of the `Mutex`; `.lock()` hands out the
value one thread at a time. (This is the BOOK's keystone, Listing 16-15.)

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

`Result: 10` every time — not *sometimes 9, sometimes 10*. The `Mutex` serialises the ten
increments so they can't collide, and the `Arc` is what lets all ten threads own the same
`Mutex` to begin with. `lock()` returns a `Result` (it can fail if another thread panicked
while holding the lock), so we `unwrap` it; `num` is a guard that releases the lock the
moment it goes out of scope at the end of each closure.

## 4. Common pitfalls / real compiler errors

**Forgetting `move` — the closure can't borrow `v` — `E0373`.** Without `move`, the closure
only *borrows* `v`, but the thread might outlive `main`, so the borrow could dangle. The
compiler refuses and tells you the fix:

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
note: function requires argument type to outlive `'static`
 --> main.rs:6:18
  |
6 |       let handle = thread::spawn(|| {
  |  __________________^
7 | |         println!("here's a vector: {v:?}");
8 | |     });
  | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

Add `move` (as in part 3) and it compiles. The thread now *owns* `v`, so there's nothing
to dangle.

**The shared counter, the hard way — two failures, then the fix.** This is the BOOK's
central lesson, and worth walking step by step. Say you try to share a counter *without*
the `Arc`, moving a bare `Mutex` straight into the loop:

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
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
error[E0382]: borrow of moved value: `counter`
  --> main.rs:20:29
   |
 5 |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `std::sync::Mutex<i32>`, which does not implement the `Copy` trait
...
 8 |     for _ in 0..10 {
   |     -------------- inside of this loop
 9 |         let handle = thread::spawn(move || {
   |                                    ------- value moved into closure here, in previous iteration of loop
...
20 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value borrowed here after move
```

`E0382` — the **first** iteration *moved* the single `counter` into its thread; there's
nothing left for the next iteration. You need many owners. From L29 you know the tool for
many owners: `Rc`. So you try wrapping it in `Rc`:

```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
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
error[E0277]: `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
  --> main.rs:11:36
   |
11 |           let handle = thread::spawn(move || {
   |                        ------------- ^------
   |                        |             |
   |  ______________________|_____________within this `{closure@main.rs:11:36: 11:43}`
   | |                      |
   | |                      required by a bound introduced by this call
12 | |             let mut num = counter.lock().unwrap();
13 | |             *num += 1;
14 | |         });
   | |_________^ `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
   |
   = help: within `{closure@main.rs:11:36: 11:43}`, the trait `Send` is not implemented for `Rc<std::sync::Mutex<i32>>`
note: required because it's used within this closure
  --> main.rs:11:36
   |
11 |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^
```

`E0277` — `Rc` **cannot be sent between threads safely**. Its reference count isn't built
for concurrent updates, so the compiler forbids moving it into a thread *at all*. The fix
is `Arc` — the thread-safe `Rc` — which is exactly the working version in part 3.

**Why those two errors, and what `Send`/`Sync` mean.** Rust has two marker traits the
compiler checks automatically. **`Send`** means a value can be *moved to* another thread;
**`Sync`** means it can be *shared by reference* across threads. Most types are both. `Rc`
is **neither** — it updates its count without synchronisation, so letting two threads
clone or drop the same `Rc` would corrupt the count (the exact data race Rust exists to
prevent). That's why the `Rc` attempt failed with E0277: `thread::spawn` requires its
closure to be `Send`, and an `Rc` inside it isn't. `Arc` does the same job as `Rc` — many
owners of one value (L29) — but updates its count *atomically*, so it **is** `Send` and
`Sync`. This is the L29 note paid off: `Rc` for single-threaded sharing, `Arc` the moment
a second thread is involved.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new threads` works too.)* **Predict on paper before each run** —
and for the interleaving ones, predict what's *guaranteed* versus what's *not*.

1. **Spawn and join.** Spawn a thread that prints `"in thread"` three times in a loop, and
   have `main` print `"in main"` three times. Join the handle at the end. **Predict:** is
   the *exact interleaving* of the six lines guaranteed? What single thing *is* guaranteed
   once you've called `join`? Run it several times and watch the order shift.

2. **Forget `move`, then add it.** Make a `vec![1, 2, 3]`, then `thread::spawn` a closure
   that prints it — *without* `move`. **Predict** whether it compiles. Run it, read the
   error code and the `help:` line, then add `move` and confirm it now works. Name what
   `move` changed about who owns the vector.

3. **A channel.** Make an `mpsc::channel()`. Spawn a thread that `send`s three numbers down
   `tx`. In `main`, receive them by looping over `rx` and printing each. **Predict** the
   output *and* its order (with a single sender — is order preserved?). Then try to use one
   of the sent values again *inside the spawned thread, after `send`* — **predict** whether
   that compiles, and why `send` moving the value matters.

4. **The shared counter — walk the whole arc.** Build the ten-thread counter, but
   deliberately, in three stages. **(a)** First with a bare `Mutex::new(0)` moved into the
   loop — **predict** the error code before running, then read it. **(b)** Wrap it in
   `Rc::new(...)` and `Rc::clone` per thread — **predict** the *new* error code, then read
   the `Send` line in it. **(c)** Switch `Rc` to `Arc`. **Predict** the final printed
   number — is it *always* that, or sometimes less? Run it several times to confirm which.
   In one sentence, say what `Arc` provides that `Rc` didn't, and what `Mutex` provides on
   top.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. The point of task 4 is that the compiler *taught you the design*: E0382 said "you
need many owners," E0277 said "but not `Rc` across threads," and `Arc<Mutex<T>>` is what
both errors were pushing you toward.)*

## 6. What surprised you?

A sentence or two: did "fearless concurrency" — the data race becoming a *compile error*
instead of a once-in-a-while crash — land? Did the two-failure arc (E0382 → E0277 →
`Arc<Mutex>`) make `Arc` feel like the answer the compiler was steering you to, rather
than a thing to memorise? Did it surprise you that the thread/`main` interleaving changes
run to run while `Result: 10` never does? Tell me, and I'll fold it into the Phase-8 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.16** "Fearless Concurrency": §16.1
  (`thread::spawn`, `JoinHandle`/`join`, `move` closures capturing data into a thread),
  §16.2 (`mpsc::channel`, `tx.send`/`rx.recv`, receiving by iterating `rx`, multiple
  producers via `tx.clone()` — Listings 16-8/16-11), §16.3 (the shared-counter arc:
  `Mutex<T>`, the E0382 then E0277 failures, and `Arc<Mutex<T>>` as the fix — **Listing
  16-15**, reproduced here), §16.4 (`Send` and `Sync` as the marker traits behind it all).
- **CR** — *Comprehensive Rust* (Google): the modular Concurrency reference slides
  (Threads, Channels, `Send`/`Sync`, Shared State) used as cross-check.
- **BLOG** — not used here; this topic isn't covered there.
- Every snippet compiled and run, and every error captured live, on **rustc 1.95.0**,
  edition 2024 (`rustc --edition 2024 FILE.rs`). The temp source path in each error was
  normalised to `main.rs`. Note: the interleaved `spawned:`/`main:` output in part 3 is
  *one* run — that order is not guaranteed and will differ on reruns; `Result: 10`,
  `sum`/`got:` channel output, by contrast, are deterministic. This opens Phase 8
  (concurrency); `Arc<Mutex<T>>` builds directly on `Rc`/`RefCell` from L29.

---

<!-- lesson-nav -->
[← Lesson 29d — RefCell: interior mutability](29d-refcell.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 31 — Async / Await: `async fn`, `.await`, `Future` →](31-async-await.md)
