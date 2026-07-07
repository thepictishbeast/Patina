# Lesson 31 — Async / Await: `async fn`, `.await`, `Future`

*(Phase 8 — Concurrency. Threads (the previous lessons in this phase) give you
**parallelism**: two pieces of work running at the same time on different cores.
Async is a different tool for a different problem: **waiting**. When a task is
stuck waiting — for a file, a network reply, a timer — async lets one thread set
that task aside and do other useful work, instead of sitting idle. This lesson is
short and conceptual: you'll learn the **shape** of async code and one honest fact
that explains everything else — a future does **nothing** until something runs it.)*

## 1. Why it exists

A program often spends most of its time **waiting** — for data to arrive, for a
reply to come back. A blocked thread that just sits there waiting is wasted: it
holds memory and does no work. You *could* spawn a thread per waiting task, but
threads are relatively heavy, and most of them would be asleep anyway.

**Async** is lightweight concurrency built for exactly this case. You mark code
that might need to wait as `async`. At each waiting point you write `.await`, which
means *"if the thing I need isn't ready yet, pause me here and let other work run;
wake me when it is."* Many such tasks take turns on a single thread, each yielding
whenever it would otherwise block. Nobody sits idle.

The key distinction to hold onto:

- **Threads** = **parallelism** — work running *at the same time* (good for CPU work).
- **Async** = **cooperative concurrency** — tasks taking turns, each stepping aside
  while it waits (good for I/O). On its own it is **not** parallelism: by default
  it's interleaving on one thread, not running on many cores.

> **How the sources frame it:** the **BOOK** Ch.17 §17.1 "Futures and the Async
> Syntax" is the backbone — it introduces a **future** as a value that isn't ready
> yet but *will* be, shows that `async fn` is **sugar** for a function returning a
> future, and stresses that futures are **lazy**. **CR** §64 adds the trait-level
> detail (the `Future` trait, `poll`, runtimes, and tasks) for later. This lesson
> stays at the BOOK's intuition level.

## 2. The idea

Three pieces of syntax, one big rule.

- **`async fn`** — mark a function `async` and it no longer returns its value
  directly. It returns a **`Future`**: a value that *represents* the eventual
  result. The body doesn't run when you call the function — it's packaged up for
  later. So `async fn double(x: u32) -> u32` is really a function that returns
  `impl Future<Output = u32>` ("a future that will produce a `u32`").
- **`async { ... }`** — an **async block**. It's an expression whose value is a
  future, the same way `async fn` produces one. Useful when you want a future
  without naming a whole function.
- **`.await`** — postfix, written after a future: `some_future.await`. It says
  *"run this future to completion; if it's not ready, pause me and let other tasks
  run until it is, then give me its value."* Crucially, **`.await` is only legal
  inside an `async fn` or `async` block** — you'll see what happens otherwise in
  part 4.

**The big rule — a future is lazy.** Calling an `async fn` builds a future and runs
**none** of its body. Nothing happens until the future is **driven** — either by
`.await`ing it from inside other async code, or, for the *top-level* future, by
handing it to a **runtime** (also called an executor) that polls it to completion.

> **The honest picture (anchored to Lesson 28, iterators):** a future is lazy in
> exactly the way an iterator is. Building an iterator with `.map(...)` runs no
> code — nothing happens until something calls `.next()` to pull values through. A
> future is the same: building it runs nothing; the work happens only when it's
> driven to completion. "Async code does nothing until you ask it to" is not a
> footnote — it's the whole model.

**Where does the runtime come from?** A real async program hands its top-level
future to a runtime — commonly written `#[tokio::main]` on `main`, or a
`block_on(future)` call. The runtime is an external crate (tokio, async-std, the
book's `trpl`), so it's **out of scope here and not available offline.** That's why
every example below is *compile-checked only*: we prove the syntax is correct and
that a future is built, but we never spin up a runtime to drive one. (The single
example we *do* run, in part 3, is built precisely to show that **nothing executes**
without a runtime.)

## 3. Tiny examples to read

These compile but are not run (there's no runtime to drive a future) — except the
last one, which runs to make a point. Each was checked with
`rustc --edition 2024 --crate-type lib`.

**`async fn` is sugar for "returns a `Future`."** The two functions below have the
**same** return type. `double` is `async`; `_check` spells out by hand what the
`async` keyword produced — a function returning `impl Future<Output = u32>`:

```rust
use std::future::Future;

// `async fn` desugars to a fn returning `impl Future<Output = u32>`.
async fn double(x: u32) -> u32 {
    x * 2
}

// Proof: a plain fn with the desugared signature accepts double's future.
fn _check() -> impl Future<Output = u32> {
    double(21)
}
```

This **compiles.** Note what is *not* here: no `21` and no `42`. Calling
`double(21)` produced a future; it didn't compute anything.

**`.await` chains futures inside an async fn.** `.await` is postfix, and legal only
because `add_doubles` is itself `async`. Each `.await` is a point where the task
would pause if the awaited future weren't ready:

```rust
async fn double(x: u32) -> u32 {
    x * 2
}

// `.await` suspends `add_doubles` until each awaited future is ready.
async fn add_doubles(a: u32, b: u32) -> u32 {
    let x = double(a).await;   // await the first future
    let y = double(b).await;   // then the second
    x + y
}
```

This **compiles.** And still nothing has run — `add_doubles` is just *another*
async fn, so calling it would build yet another lazy future.

**`async { ... }` is a future too.** An async block is an expression that evaluates
to a future, so a normal function can return one:

```rust
use std::future::Future;

// `async { ... }` evaluates to a future.
fn make_future() -> impl Future<Output = i32> {
    async {
        let a = 10;
        let b = 4;
        a + b
    }
}
```

This **compiles.** `make_future()` hands back a future; the `a + b` inside has not
been evaluated.

**Proof that a future is lazy — this one we actually run.** The `async fn` has a
`println!` *inside* it. We build the future and never drive it:

```rust
async fn double(x: u32) -> u32 {
    println!("doubling {x}");   // a side effect INSIDE the future
    x * 2
}

fn main() {
    // Calling an async fn does NOT run its body — it only builds a future.
    let _future = double(21);
    println!("made the future");
    // `_future` is never driven by a runtime, so its body never runs.
}
```

```
made the future
```

Read that output twice. `"doubling 21"` is **not** there. The future was built but
never polled, so the code *inside* it — including its `println!` — never executed.
This is the entire async model in one run: **build is free; nothing happens until
something drives the future.**

## 4. Common pitfalls / real compiler errors

**`.await` outside an `async` context — `E0728`.** `.await` is only allowed inside
an `async fn` or `async` block. A plain `fn main` is not async, so awaiting in it is
a compile error:

```rust
async fn double(x: u32) -> u32 {
    x * 2
}

fn main() {
    let result = double(21).await;
    println!("{result}");
}
```

```
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> main.rs:6:29
  |
5 | fn main() {
  | --------- this is not `async`
6 |     let result = double(21).await;
  |                             ^^^^^ only allowed inside `async` functions and blocks
```

The compiler is precise: `main` "is not `async`," so there's nowhere to pause. The
*tempting* fix — mark `main` itself `async` — is the second pitfall below.

**You cannot make `main` async on its own — `E0752`.** It looks like the obvious
escape from `E0728`, but a bare `async fn main` doesn't compile either:

```rust
async fn main() {
    println!("hi");
}
```

```
error[E0752]: `main` function is not allowed to be `async`
 --> main.rs:1:1
  |
1 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

Why both errors? They point at the same missing piece: **a runtime.** `main` is the
entry point — there's no async caller above it to `.await` it, and Rust's `main`
can't be a future, because *something* has to drive futures and `main` is where the
program starts. In real code you attach a runtime to `main` (e.g. `#[tokio::main]`,
out of scope here), which sets up the executor that turns your top-level future into
actual running work. Without a runtime, async syntax type-checks but has nothing to
run it — which is exactly what these two errors are telling you.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu) — they compile without an async
runtime. *(On your own machine, `rustc --edition 2024` works too.)* **Predict on
paper before each run.**

1. **A future that doesn't run.** Write an `async fn greet(name: &str)` whose body
   does `println!("hello {name}")` and returns `()`. In `main` (a normal `fn`),
   *call* `greet("world")` and bind it to a variable, then `println!("done")`.
   **Predict the exact output** before you compile and run. Does `"hello world"`
   appear? Explain to yourself **why or why not** in one sentence — this is the whole
   lesson in one prediction.

2. **Two return types, one meaning.** Write an `async fn answer() -> u32` returning
   `42`. Then write a *non-async* `fn also_answer() -> impl Future<Output = u32>`
   whose body is just `answer()`. **Predict**: does this compile? What does the
   `async` keyword on `answer` correspond to in `also_answer`'s written-out return
   type? (You'll need `use std::future::Future;`.)

3. **Await where it's allowed, and where it isn't.** Start from task 2's `answer`.
   First, write an `async fn` that does `let n = answer().await;` and returns `n` —
   confirm it compiles. Then move that exact `.await` line into a **non-async** `fn`.
   **Predict the error code and the gist of its message** before compiling. Which
   error from part 4 do you expect, and what phrase will it use about your function?

4. **The tempting non-fix.** Take any program that hit `E0728` and try to fix it by
   writing `async fn main()`. **Predict**: does it now compile? If not, **which** new
   error appears, and what single missing thing do *both* errors really point to?
   Name it.

*(You write every line here — I won't. Notice that every task above is about reading
the **shape** of async code and predicting whether anything runs; none of them needs
a runtime, because the point of this lesson is what happens *before* a runtime ever
gets involved. When you later reach for tokio or async-std, this is the foundation:
`async`/`.await` describe the work; the runtime is what finally does it.)*

## 6. What surprised you?

A sentence or two: did "a future does nothing until something drives it" land — and
did the `made the future` example (where `"doubling 21"` never prints) make it
concrete? Did the iterator parallel from Lesson 28 (`.map` does nothing until
`.next()`) help? Were you surprised that **both** `E0728` and `E0752` are really
complaining about the same missing thing — a runtime? And is the threads-vs-async
split (parallelism vs cooperative waiting) clear enough that you can say *when* you'd
pick each? Tell me, and I'll fold it into the Phase-8 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.17 "Fundamentals of Asynchronous
  Programming," §17.1 "Futures and the Async Syntax"**: the framing of a *future* as
  a value that becomes ready later, `async fn` as **sugar** for a function returning
  `impl Future`, and the central point that **futures are lazy** — they do nothing
  until driven (paraphrased, and likened here to iterators doing nothing until
  `.next()`).
- **CR** — *Comprehensive Rust* (Google), **§64 "Async Basics"**: the `Future` trait
  and `poll`, runtimes/executors, and tasks — the trait-level layer beneath this
  lesson's intuition (deferred to later study; the runnable end-to-end examples there
  need an external runtime crate, which is out of scope offline).
- **BLOG** — not used here; it mentions async only in passing.
- Every snippet was compile-checked on **rustc 1.95.0**, edition 2024. The
  compile-only examples used `rustc --edition 2024 --crate-type lib`; the one runnable
  example and both error captures used `rustc --edition 2024 FILE.rs` (errors shown
  verbatim, with the temp path normalized to `main.rs`). **No async output is
  fabricated:** async bodies cannot run here without a runtime (tokio / async-std /
  the book's `trpl`, all out of scope), so the only execution output shown is the
  deliberately-lazy `made the future` — which proves the async body did *not* run.
  This opens the async portion of Phase-8 (concurrency).

---

<!-- lesson-nav -->
[← Lesson 30 — Threads, Channels & Shared State](30-threads-and-concurrency.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 32 — Trait Objects & OOP in Rust: `dyn Trait`, Encapsulation, States as Types →](32-trait-objects-and-oop.md)
