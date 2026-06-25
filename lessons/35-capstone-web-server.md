# Lesson 35 — Capstone: A Multithreaded Web Server

*(Phase 9 — the capstone. This is where the pieces meet: a small HTTP server that hands each
incoming connection to a fixed pool of worker threads, so one slow request can't stall the
rest. It pulls together **closures** (L27), **threads + channels + `Arc<Mutex>`** (L30),
**`Box<dyn Trait>`** (L29 + L25), and **`Option::take`** (L19). Unlike every other lesson,
this one BINDS a network port — so you compile-check it here and **run it yourself** with
`cargo run`, then open a browser. Nothing in this file fabricates server output.)*

## 1. Why it exists

Every concept so far has been a tool. A capstone proves the tools combine into something real.
A web server is a perfect test: it must accept many connections, and a naive single-threaded
loop handles them one at a time — so a single slow request makes everyone else wait. Fixing
that *safely* (no data races, no leaked threads) is exactly what Rust's ownership model plus a
**thread pool** give you. If you can read and build this, you can read most real Rust.

> **How the sources frame it:** the **BOOK** Ch.20 is the only full treatment, and it builds
> the server compiler-driven: start single-threaded, watch one slow request block the rest,
> then introduce a `ThreadPool` whose `execute` boxes a closure and sends it down a channel to
> worker threads that share the receiver via `Arc<Mutex<…>>`. We follow that arc.

## 2. The idea

The shape of the whole thing, top to bottom:

- **Listen.** `TcpListener::bind("127.0.0.1:7878")`, then loop over `listener.incoming()` — each
  item is a `TcpStream` (one connection).
- **Don't handle it inline.** Instead of `handle_connection(stream)` in the loop (single-
  threaded — one slow request blocks all), hand the work to a **`ThreadPool`**:
  `pool.execute(|| handle_connection(stream))`.
- **The pool.** `ThreadPool::new(4)` spawns four **worker** threads up front and holds the
  *sending* half of an `mpsc` channel. `execute` boxes your closure into a `Job` and sends it.
- **The job type.** A job is `Box<dyn FnOnce() + Send + 'static>` — a heap-boxed closure
  (you don't know its concrete type, hence `dyn`; `Send` so it can cross to another thread;
  `'static` so it lives long enough).
- **Sharing the receiver.** All four workers must pull jobs from the *one* receiving half. A
  `Receiver` can't be copied, so it's shared as `Arc<Mutex<Receiver>>`: `Arc` for multiple
  owners across threads, `Mutex` so only one worker takes a job at a time.
- **Shutdown.** A `Drop` impl on `ThreadPool` drops the sender (so each worker's `recv()`
  returns `Err` and the loop breaks) and `join`s every worker thread.

## 3. The code to read

This is ONE program (in a real project, the pool lives in `src/lib.rs` and `main` in
`src/main.rs`). It **compiles** as shown; it isn't run here because it would bind a TCP port —
build and run it yourself with `cargo run`, then visit `http://127.0.0.1:7878`.

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

**The server itself:**

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let _ = stream.read(&mut buffer);
    let body = "<h1>Hello from Rust</h1>";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));   // a closure capturing this connection
    }
}
```

Run it (`cargo run`) and load `http://127.0.0.1:7878` in a browser — you'll see *Hello from
Rust*, and the terminal prints which worker handled each request.

## 4. Common pitfalls / real compiler errors

**Why `Arc<Mutex<Receiver>>` and not just the receiver?** The natural first attempt — give each
worker thread the receiver directly — doesn't compile, and the error *is* the lesson:

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

The first `spawn` *moves* `receiver` into its closure; the second iteration has nothing left to
move. A `Receiver` isn't `Copy` and isn't meant to be cloned for multiple consumers — so to let
several workers share *one* receiving end you wrap it in `Arc` (multiple owners) **and** `Mutex`
(one-at-a-time access): `Arc<Mutex<Receiver>>`, exactly what `ThreadPool::new` builds. This is
ownership (L15), shared ownership (L29 `Rc`/`Arc`), and `Mutex` (L30) all paying off at once.

One more, by design: keep the lock short. `receiver.lock().unwrap().recv()` takes the lock,
gets a job, and the temporary lock guard is dropped **before** `job()` runs — so a long job
doesn't keep the other workers blocked on the mutex. Holding the lock across `job()` would
serialize the whole pool.

## 5. Predict-then-run practice (your turn — write this yourself)

Build it for real: `cargo new hello-server`, put `ThreadPool` in `src/lib.rs` and the server in
`src/main.rs`. **Predict before each run.**

1. **Make it work, then make it block.** Get the threaded version serving
   `http://127.0.0.1:7878`. Then add a route: if the request is for `/sleep`, call
   `thread::sleep(Duration::from_secs(5))` before responding. With a pool of 4, open `/sleep`
   in one tab and `/` in another. **Predict** whether `/` still responds fast, then try it.

2. **Shrink the pool to 1.** Set `ThreadPool::new(1)`. **Predict** what now happens to the `/`
   request while `/sleep` is running, and confirm.

3. **Watch shutdown.** Make `main` accept only two requests (`listener.incoming().take(2)`) so
   the pool is dropped. **Predict** what the `Drop` impl prints/does as it joins the workers.

*(You write every line here — I won't. This is the end of the lesson series: if you can build
and reason about this server, you can read real Rust codebases — including this app.)*

## 6. What surprised you?

A sentence or two: did it click that `execute` just *boxes and sends a closure*? Did the
`E0382` make it obvious why the receiver needs `Arc<Mutex>`? Did dropping the lock guard
*before* running the job feel like the kind of detail Rust makes you get right? Tell me.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, Ch.20 "Final Project: Building a Multithreaded Web
  Server": the single-threaded server, the slow-request problem, the `ThreadPool` API, boxing
  jobs into `Box<dyn FnOnce() + Send>`, the shared `Arc<Mutex<Receiver>>`, and graceful shutdown
  via `Drop` + `join`.
- It ties together Lessons 27 (closures), 29 (`Box`, `Arc`), and 30 (threads, channels,
  `Mutex`).
- The full server compiles on **rustc 1.95.0**, edition 2024 (built, not run — it binds a TCP
  port); the `E0382` is captured verbatim. Run it yourself with `cargo run`.

---

<!-- lesson-nav -->
[← Lesson 34 — Advanced Features: `unsafe`, Traits & Macros](34-advanced-features.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 36 — Automated Tests →](36-automated-tests.md)
