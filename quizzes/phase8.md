# Phase 8 Quiz — Concurrency (threads & async)

A self-check for the Phase-8 **Concurrency** lessons (Lessons 30–31: spawning &
joining threads, `mpsc` channels, `Mutex<T>` + `Arc<T>` for shared state, `Send`/`Sync`,
and the shape of `async`/`.await`). Same rule as before: **predict each answer before**
you look at the **Answers** section. Don't run the code first; predict, then verify.
Thirteen questions.

> Tip: cover the Answers section until you've committed to an answer for every question.
> The **thread** snippets here are runnable and their outputs are deterministic (no
> interleaving guesswork). The **async** snippets are compile-checked only — there's no
> runtime offline to drive a future, so an async question asks *"does this compile?"*,
> never *"what does it print at runtime?"*

---

## Questions

**Q1 — concept.** Rust has two marker traits the compiler checks automatically to keep
threads safe. In one line each: what does **`Send`** mean about a value, and what does
**`Sync`** mean about it?

**Q2 — predict the output.**
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
Is the printed number the **same** every run, or does it sometimes come out less? Say which.

**Q3 — does this compile? If not, what's the error code?**
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

**Q4 — does this compile? If not, what's the error code?**
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

**Q5 — predict the output.**
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for n in [10, 20, 30] {
            tx.send(n).unwrap();
        }
    });

    for received in rx {
        println!("got: {received}");
    }
}
```

**Q6 — does this compile? If not, what's the error code?**
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

**Q7 — predict the output.**
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for n in 1..=5 {
            tx.send(n).unwrap();
        }
    });

    let total: i32 = rx.iter().sum();
    println!("total: {total}");
}
```

**Q8 — predict the output.**
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..=5 {
            sum += i;
        }
        sum
    });

    let result: i32 = handle.join().unwrap();
    println!("sum = {result}");
}
```

**Q9 — does this compile? If not, what's the error code?**
```rust
async fn double(x: u32) -> u32 {
    x * 2
}

fn main() {
    let result = double(21).await;
    println!("{result}");
}
```

**Q10 — does this compile? If not, what's the error code?**
```rust
async fn main() {
    println!("hi");
}
```

**Q11 — does this compile?** (Compiled as a library — no `main`.)
```rust
use std::future::Future;

async fn double(x: u32) -> u32 {
    x * 2
}

fn _check() -> impl Future<Output = u32> {
    double(21)
}
```

**Q12 — predict the output.** (This one really runs — a normal `fn main`, no runtime.)
```rust
async fn double(x: u32) -> u32 {
    println!("doubling {x}");   // a side effect INSIDE the future
    x * 2
}

fn main() {
    let _future = double(21);
    println!("made the future");
}
```

**Q13 — fill in the blanks (concept).** (a) Threads give you **`____`** — work running at
the same time, good for CPU-bound jobs. (b) Async on a single thread gives you **`____`**
concurrency — tasks taking turns, each stepping aside while it waits, good for I/O.
(c) In `mpsc::channel()`, the letters **`mpsc`** stand for **`____`**.

---

## Answers

*(Verified on rustc 1.95.0, edition 2024. Thread snippets compiled and run; async snippets
compile-checked only — no runtime offline, so no async runtime output is shown.)*

**A1 — `Send`: the value can be *moved to* another thread. `Sync`: the value can be
*shared by reference* (`&T`) across threads.** Most types are both; the compiler derives
them automatically and `thread::spawn` requires its closure to be `Send`. (Lesson 30.)

**A2 — `Result: 10`, the same every run** (never sometimes 9). The `Mutex` serialises the
ten increments so they can't collide, and the `Arc` is what lets all ten threads *own* the
one `Mutex`. That determinism is the whole point of `Arc<Mutex<T>>`. (Lesson 30.)

**A3 — No: `error[E0382]`** ("borrow of moved value: `counter`"). A bare `Mutex` has a
single owner; the **first** loop iteration `move`s it into its thread, leaving nothing for
the next iteration (or for the final `println!`). You need **many owners** — which points
you toward `Rc`/`Arc`. (Lesson 30.)

**A4 — No: `error[E0277]`** ("`Rc<std::sync::Mutex<i32>>` cannot be sent between threads
safely"). `Rc` updates its reference count *without* synchronisation, so it is **not
`Send`** — and `thread::spawn` requires a `Send` closure, so the compiler rejects moving an
`Rc` into a thread at all (this is the data race Rust exists to prevent). The fix is `Arc`:
same job as `Rc` (many owners of one value), but it updates its count **atomically**, so it
*is* `Send` and `Sync`. (Lesson 30.)

**A5 — `got: 10` / `got: 20` / `got: 30`**, in that order. With a **single** sender, order
is preserved. The `for received in rx` loop ends on its own once `tx` is dropped (the
spawned thread finished). Each `send` **moves** its value into the channel. (Lesson 30.)

**A6 — No: `error[E0373]`** ("closure may outlive the current function, but it borrows
`v`"). Without `move`, the closure only *borrows* `v`, but the thread might outlive
`main`'s frame, so the borrow could dangle. The compiler's `help:` line tells you the fix:
add **`move`** to give the thread ownership of `v`. (Lesson 30.)

**A7 — `total: 15`.** The thread sends `1, 2, 3, 4, 5`; `rx.iter()` yields them until `tx`
is dropped, and `.sum()` adds them: `1+2+3+4+5 = 15`. The *sum* is deterministic regardless
of timing. (Lesson 30.)

**A8 — `sum = 15`.** The spawned closure returns `sum` (`1+2+3+4+5`), and `handle.join()`
returns a `Result` carrying that value — `.unwrap()` pulls out the `15`. `join` both waits
for the thread *and* hands back what its closure returned. (Lesson 30.)

**A9 — No: `error[E0728]`** ("`await` is only allowed inside `async` functions and
blocks"). `main` here is a plain `fn`, not `async`, so there's nowhere to pause — `.await`
is illegal. (Lesson 31.)

**A10 — No: `error[E0752]`** ("`main` function is not allowed to be `async`"). `main` is the
entry point; there's no async caller above it to drive its future, so it can't be `async`
on its own. In real code you attach a runtime (e.g. `#[tokio::main]`, out of scope here).
Both E0728 and E0752 point at the same missing piece: **a runtime.** (Lesson 31.)

**A11 — Yes, it compiles.** `async fn double(x: u32) -> u32` is **sugar** for a function
returning `impl Future<Output = u32>`, so `_check`'s hand-written signature matches the
future that `double(21)` builds. Note: calling `double(21)` builds a future and computes
nothing. (Lesson 31.)

**A12 — `made the future`** — and that's *all*. `"doubling 21"` does **not** print. Calling
an `async fn` only **builds** a future; its body (including the `println!` inside it) runs
nothing until the future is **driven** by a `.await` or a runtime. Here `_future` is never
driven, so its body never executes. A future is **lazy**. (Lesson 31.)

**A13 — (a) parallelism; (b) cooperative; (c) multiple producer, single consumer.**
Threads run work at the same time (parallelism); async on one thread interleaves tasks that
take turns (cooperative concurrency); an `mpsc` channel allows many senders feeding one
receiver. (Lessons 30–31.)

---

*How did you do?* Anything you missed points at the lesson to reread. You can now spawn and
join threads, pass data through a channel, share one value with `Arc<Mutex<T>>`, explain why
`Rc` is rejected across threads (`Send`), and read the *shape* of async code — including the
one fact that drives everything: a future does nothing until something drives it. Next phase
moves on from concurrency.

— *Sources:* questions written for this corpus from Lessons 30–31 (BOOK Ch.16 "Fearless
Concurrency" & Ch.17 §17.1; CR Concurrency/Async slides). Every thread snippet was compiled
and run, and every error code captured live, on **rustc 1.95.0**, edition 2024; the async
snippets were compile-checked only (no runtime offline), so no async runtime output is shown.
