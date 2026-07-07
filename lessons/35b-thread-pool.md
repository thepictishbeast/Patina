# Lesson 35b — Capstone II: the thread pool & graceful shutdown

*(Phase 9 — the capstone concludes. Lesson 35 left you with a server you
personally watched stall: one slow request, everyone waits. The fix is a
**thread pool** — and building one pulls together closures (L27), threads +
channels + `Arc<Mutex>` (L30–30c), and `Box<dyn Trait>` (L29 + L32) into forty
lines of real Rust. Compile-checked here; you run it with `cargo run`.)*

## 1. Why it exists

Spawning one thread *per connection* would work — until ten thousand
connections spawn ten thousand threads. A **thread pool** caps the cost: spawn
a fixed crew of worker threads up front, queue incoming work, and let each
free worker take the next job. One slow request now occupies one worker; the
other workers keep serving. Fixing this *safely* — no data races, no leaked
threads — is exactly what Rust's ownership model gives you.

> **How the sources frame it:** the **BOOK** Ch.21's threaded half. It builds
> the pool **compiler-driven** — the design falls out of the errors, exactly
> like Lesson 30c's `Arc<Mutex>` arc. We follow it; part 4's E0382 is the
> load-bearing wall.

## 2. The idea

- **Don't handle it inline.** `main`'s loop becomes
  `pool.execute(|| handle_connection(stream));` — hand the closure off, move on.
- **The pool.** `ThreadPool::new(4)` spawns four **workers** up front and
  keeps the *sending* half of an `mpsc` channel (L30b). `execute` boxes your
  closure into a `Job` and sends it down the channel.
- **The job type.** `type Job = Box<dyn FnOnce() + Send + 'static>;` — read it
  piece by piece with tools you have: `Box<dyn …>` because each closure has
  its own unnameable type (L32's trait objects); `FnOnce` because it'll run
  once (L27b); `Send` so it may cross to another thread (L30c). **The one new
  token is `'static`**: read it as "this closure *owns* everything it
  captured — it borrows nothing that could expire while the job waits in the
  queue." It's a lifetime bound (L26's family), marking the no-borrows end of
  that spectrum.
- **Sharing the receiver.** All four workers pull jobs from the *one*
  receiving half. A `Receiver` can't be copied — part 4 shows the compiler
  refusing — so it's shared as `Arc<Mutex<Receiver>>`: `Arc` for many owners
  across threads, `Mutex` so only one worker takes a job at a time (L30c's
  pairing, third appearance in the course).
- **Shutdown.** Dropping the pool drops the sender; every worker's `recv()`
  then returns `Err`, breaking its loop; `join` waits for each. The `Drop`
  impl (L29b) makes this automatic.

## 3. The code to read

ONE program with Lesson 35's server (in a real project the pool lives in
`src/lib.rs`). It **compiles** as shown; run it yourself with `cargo run`.

**The job type and the pool's public surface:**

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));   // one receiver, shared
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender: Some(sender) }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);                            // box the closure into a Job
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
```

(That `assert!(size > 0)` is a peek at Lesson 36: it crashes with a clear
message if the condition is false — here, guarding against a zero-worker
pool. The assert family gets its real treatment next lesson.)

**A worker: a thread that loops, taking one job at a time:**

```rust
struct Worker {
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();   // lock, then wait for a job
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; running it.");
                    job();                                    // lock released before running
                }
                Err(_) => break,   // the sender was dropped -> shut this worker down
            }
        });
        Worker { handle: Some(handle) }
    }
}
```

One deliberate subtlety: `receiver.lock().unwrap().recv()` takes the lock,
gets a job, and drops the lock guard **before** `job()` runs — so a slow job
doesn't keep the other workers locked out. (Why that's true, and how holding
the guard would silently serialize the whole pool, is walked through in the
Book's multithreading section — read it there; it's a masterclass in guard
scopes.)

**Graceful shutdown — drop the sender, then join every worker:**

```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());          // close the channel -> workers' recv() returns Err
        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();    // wait for each to finish
            }
        }
    }
}
```

**`.take()` is new — here's the whole trick.** `Drop` only gives us
`&mut self`, and you can't *move* a field out through a `&mut` borrow (L16b).
`Option::take` is the escape: it swaps the `Option` to `None` in place and
hands you what it held — ownership extracted, a valid value left behind.
That's why `sender` and `handle` are wrapped in `Option` at all. (The
ordering — sender first, *then* join — and what deadlocks if you swap it, is
the Book's shutdown section; it's worth the read.)

**And `main` changes by exactly one line:**

```rust
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));   // a closure capturing this connection
    }
}
```

## 4. Common pitfalls / real compiler errors

**Why `Arc<Mutex<Receiver>>` and not just the receiver? — `E0382`.** The
natural first attempt — give each worker the receiver directly — doesn't
compile, and the error *is* the design lesson:

```rust
use std::sync::mpsc;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn main() {
    let (_sender, receiver) = mpsc::channel::<Job>();
    for _ in 0..4 {
        let handle = thread::spawn(move || {
            let job = receiver.recv().unwrap();   // each thread wants the same receiver
            job();
        });
    }
}
```

```
error[E0382]: use of moved value: `receiver`
  --> main.rs:11:36
   |
 7 |     let (_sender, receiver) = mpsc::channel::<Job>();
   |                   -------- move occurs because `receiver` has type `std::sync::mpsc::Receiver<...>`, which does not implement the `Copy` trait
...
11 |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
12 |             let job = receiver.recv().unwrap();
   |                       -------- use occurs due to use in closure
```

The first `spawn` *moves* `receiver` into its closure; the second iteration
has nothing left to move — the same "moved in a previous iteration" wall as
Lesson 30c's counter, and the same cure: wrap it in `Arc` (many owners) and
`Mutex` (one-at-a-time access). Ownership (L15), shared ownership (L29c/30c),
and channels (L30b) all paying off at once — which is what a capstone is for.

## 5. Predict-then-run practice (your turn — write this yourself)

Extend your Lesson 35 project: pool in `src/lib.rs`, server in `src/main.rs`.
**Predict before each run.**

1. **Fix the flaw you felt.** Wire in the `ThreadPool` (type it, don't paste)
   and redo Lesson 35's experiment: `/sleep` in one tab, `/` in another, pool
   of 4. **Predict** whether `/` still waits, then watch the worker lines in
   the terminal — which worker got which request?
2. **Shrink the pool to 1.** `ThreadPool::new(1)`. **Predict** what happens to
   `/` while `/sleep` runs — and say why in one sentence naming the number of
   workers. (You've rebuilt Lesson 35's flaw *deliberately*.)
3. **Watch shutdown.** Make `main` accept only two requests
   (`listener.incoming().take(2)`) so `main` ends and the pool is dropped.
   **Predict** the order of what you'll see: which prints first — worker
   job lines, or the program exiting? What would happen if `Drop` joined the
   workers *without* dropping the sender first? (Reason it out, then check
   your reasoning against the Book's shutdown section.)

*(You write every line here — I won't. The predictions are your answer key.
The capstone is built: a real multithreaded server, designed by compiler
errors you can now read fluently. Two tooling lessons remain — automated
tests (36) and Cargo (37) — the everyday kit for code you now know how to
write.)*

## 6. What surprised you?

A sentence or two: did it click that `execute` just *boxes and sends a
closure*? Did the `E0382` make `Arc<Mutex<Receiver>>` feel inevitable rather
than clever? And did `Option::take` — extracting ownership through a `&mut` —
feel like a trick worth keeping? Tell me, and I'll fold it into the capstone
review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.21 "Final Project"**, the
  threaded half: the `ThreadPool` API, boxing jobs into
  `Box<dyn FnOnce() + Send + 'static>`, sharing the receiver via
  `Arc<Mutex<Receiver>>`, the worker loop and its lock scope, and graceful
  shutdown via `Drop` + `join` — including the lock-scope and shutdown-order
  discussions this lesson points at rather than reproduces.
- It ties together Lessons 27/27b (closures), 29–29c (`Box`, smart pointers),
  30–30c (threads, channels, `Arc<Mutex>`), 32 (trait objects), and 29b
  (`Drop`).
- The full server compiles on **rustc 1.95.0**, edition 2024 (built, not
  run — it binds a TCP port); the `E0382` is captured verbatim. Run it
  yourself with `cargo run`.

---

<!-- lesson-nav -->
[← Lesson 35 — Capstone I: a single-threaded web server](35-single-threaded-server.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 36 — Automated Tests →](36-automated-tests.md)
