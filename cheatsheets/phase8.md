# Phase 8 Cheatsheet — Concurrency (threads & async)

Quick reference (pairs with the Phase-8 Concurrency lessons — L30 threads, channels &
shared state · L31 async/await). The shape of it: **threads** run work *at the same time*;
**channels** hand data safely from one thread to another; **`Arc<Mutex<T>>`** lets many
threads share one value without racing; **`Send`/`Sync`** are the marker traits that make
the compiler reject data races; **async** is a *separate* tool — cooperative concurrency for
*waiting*, where a future does nothing until something drives it. Verified on rustc 1.95.0,
edition 2024. *("Fearless concurrency": the classic data race becomes a **compile error**.)*

## Threads — `spawn` + `join`
- `thread::spawn(closure)` starts the closure on a **new thread** and returns a **`JoinHandle`**. The new thread's lines interleave with `main`'s in an order you **don't control**.
- `handle.join()` **blocks** until that thread finishes, then returns a `Result` carrying **whatever the closure returned** (`handle.join().unwrap()` to get the value). The one guarantee: after `join`, the thread is done.
- **`move`** closures: a spawned thread may outlive the function that started it, so captured data must be **owned**, not borrowed. Write `thread::spawn(move || …)` to move data in. Forget `move` and the borrow could dangle → **`error[E0373]`** ("closure may outlive the current function, but it borrows …"); the `help:` line literally tells you to add `move`.

## Channels — `mpsc` (pass data between threads)
- `let (tx, rx) = mpsc::channel();` — a one-way pipe. **`mpsc`** = **m**ultiple **p**roducer, **s**ingle **c**onsumer. `tx` = transmitter, `rx` = receiver.
- `tx.send(value)` **moves** `value` into the channel (the sender can't touch it afterward — the type system, not discipline, stops two threads sharing it). `rx.recv()` waits for and takes one value.
- **Receive by looping:** `for received in rx { … }` (or `rx.iter()`) yields values until **all** senders are dropped, then ends on its own. With a **single** sender, **order is preserved**.
- **Many producers:** `tx.clone()` gives extra senders feeding the one `rx`. With multiple senders the *interleaving* isn't fixed, but an order-independent result (e.g. a `.sum()`) is still deterministic.

## Shared state — `Arc<Mutex<T>>`
- **`Mutex<T>`** (*mutual exclusion*) guards a value. `.lock()` **blocks** until no one else holds the lock, then returns a `Result` wrapping a **guard**; `*guard` reaches the value. The lock **releases automatically** when the guard goes out of scope (that's `Drop`, L29). `.lock()` returns a `Result` because it can fail if a holder panicked — so `.lock().unwrap()`.
- **`Arc<T>`** (*atomically reference counted*) = the thread-safe sibling of `Rc<T>` (L29): many **owners** of one value, with the count updated **atomically**. `Arc::clone(&x)` per thread.
- **The pairing to memorise: `Arc<Mutex<T>>`** — `Arc` lets many threads *own* the value; `Mutex` lets them *change* it without colliding. The ten-thread counter prints **`Result: 10` every run** (the `Mutex` serialises the increments) — deterministic, never "sometimes 9."

## Why `Rc` is rejected across threads — `Send` / `Sync`
- Two marker traits the compiler derives automatically: **`Send`** = the value can be *moved to* another thread; **`Sync`** = it can be *shared by reference* (`&T`) across threads. Most types are both.
- `thread::spawn` requires its closure to be **`Send`**. **`Rc` is neither `Send` nor `Sync`** — its count isn't synchronised, so two threads cloning/dropping the same `Rc` would corrupt it (the exact data race Rust prevents). Move an `Rc` into a thread → **`error[E0277]`** ("`Rc<…>` cannot be sent between threads safely"). Use **`Arc`** instead.
- The compiler *teaches the design* via two errors: bare `Mutex` moved into a loop → **`E0382`** ("you need many owners"); `Rc` across threads → **`E0277`** ("but not `Rc`"); `Arc<Mutex<T>>` is what both errors steer you to.

## Async — `async fn` / `.await` / lazy `Future` (conceptual)
- **`async fn f(..) -> T`** is **sugar** for a fn returning **`impl Future<Output = T>`** — a value that *represents* the eventual result. Calling it **builds a future and runs none of the body**.
- **`async { … }`** is an expression whose value is a future (a future without naming a whole fn).
- **`.await`** (postfix: `fut.await`) drives a future to completion, pausing the current task if it isn't ready and letting other tasks run. **Only legal inside an `async fn` or `async` block.**
- **The big rule — a future is lazy.** Nothing runs until the future is **driven**: by `.await` from inside other async code, or by handing the *top-level* future to a **runtime** (executor). A future is lazy in exactly the way an iterator is (L28): building runs nothing; the work happens only when it's pulled to completion.
- **Threads vs async:** threads = **parallelism** (same time, many cores, CPU work); async = **cooperative concurrency** (tasks take turns on one thread, each stepping aside while it waits — I/O work). Async on its own is **not** parallelism.
- **No runtime offline:** runtimes (`#[tokio::main]`, `block_on`, the book's `trpl`) are external crates, out of scope here — so async examples are **compile-checked only**, never run for output.

## Async pitfalls (real error codes)
- **`.await` in a non-`async` fn** (e.g. plain `fn main`) → **`error[E0728]`** ("`await` is only allowed inside `async` functions and blocks").
- **`async fn main()` on its own** → **`error[E0752]`** ("`main` function is not allowed to be `async`"). Both errors point at the same missing piece — **a runtime** to drive the top-level future.

— *Sources:* BOOK Ch.16 "Fearless Concurrency" (§16.1–16.4) & Ch.17 §17.1 "Futures and the
Async Syntax" · CR Concurrency / Async Basics slides. Snippets verified on rustc 1.95.0,
edition 2024 — thread snippets compiled **and run** (deterministic outputs only; interleaving
order is *not* guaranteed), async snippets **compile-checked only** (no runtime offline). The
"future is lazy, like an iterator" parallel is the BOOK's own, carried from L28/L31.
