# Lesson 31b — Futures are lazy: why async needs a runtime

*(Phase 8 — Concurrency, the finale. Lesson 31 left a question hanging: all
that async syntax compiled, but did anything RUN? No. One honest fact — a
future does nothing until something drives it — explains `.await`, both errors
below, and why every real async program starts with a runtime.)*

## 1. Why it exists — the big rule

**A future is lazy.** Calling an `async fn` builds a future and runs **none**
of its body. Nothing happens until the future is **driven** — either by
`.await`ing it from inside other async code, or, for the *top-level* future, by
handing it to a **runtime** (also called an executor) that polls it to
completion.

That single rule is why Lesson 31's examples could all compile without a `42`
ever appearing, and why this lesson can *prove* the model with one short run.

## 2. The idea

> **The honest picture (anchored to Lesson 28, iterators):** a future is lazy
> in exactly the way an iterator is. Building an iterator with `.map(...)` runs
> no code — nothing happens until something calls `.next()` to pull values
> through. A future is the same: building it runs nothing; the work happens
> only when it's driven to completion. "Async code does nothing until you ask
> it to" is not a footnote — it's the whole model.

**Where does the runtime come from?** In real code it's an external crate —
you'll meet `#[tokio::main]` and `block_on` when you get there, and **BOOK
Ch.17 §17.1** shows a full runnable setup. External crates are out of scope and
not available offline, so every example here is *compile-checked only* — except
the one below, which runs precisely to show that **nothing executes** without a
runtime.

## 3. The proof — one run

The `async fn` has a `println!` *inside* it. We build the future and never
drive it:

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

Read that output twice. `"doubling 21"` is **not** there. The future was built
but never polled, so the code *inside* it — including its `println!` — never
executed. This is the entire async model in one run: **build is free; nothing
happens until something drives the future.**

## 4. Common pitfalls / real compiler errors — two errors, one missing piece

**`.await` outside an `async` context — `E0728`.** `.await` is only allowed
inside an `async fn` or `async` block. A plain `fn main` is not async, so
awaiting in it is a compile error:

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

The compiler is precise: `main` "is not `async`," so there's nowhere to pause.
The *tempting* fix — mark `main` itself `async` — is the second pitfall.

**You cannot make `main` async on its own — `E0752`.** It looks like the
obvious escape from `E0728`, but a bare `async fn main` doesn't compile either:

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

Why both errors? They point at the same missing piece: **a runtime.** `main`
is the entry point — there's no async caller above it to `.await` it, and
`main` can't itself be a future, because *something* has to drive futures and
`main` is where the program starts. In real code a runtime attached to `main`
(BOOK Ch.17 §17.1 again) turns your top-level future into actual running work.
Without one, async syntax type-checks but has nothing to run it — which is
exactly what these two errors are telling you.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu) — they compile without any
async runtime. *(On your own machine, `rustc --edition 2024` works too.)*
**Predict on paper before each run.**

1. **A future that doesn't run.** Write an `async fn greet(name: &str)` whose
   body does `println!("hello {name}")`. In a normal `fn main`, *call*
   `greet("world")`, bind it to a variable, then `println!("done")`. **Predict
   the exact output** before you run. Does `"hello world"` appear? Explain to
   yourself **why or why not** in one sentence — this is the whole lesson in
   one prediction.
2. **Await where it isn't allowed.** Take Lesson 31's `async fn answer() ->
   u32` and its awaiting `async fn` (practice task 3 there). Move the
   `let n = answer().await;` line into a **non-async** `fn`. **Predict the
   error code and the gist of its message** — what phrase will the compiler
   use about your function?
3. **The tempting non-fix.** Try to fix task 2 by writing `async fn main()`.
   **Predict**: does it compile now? If not, **which** new error appears — and
   what single missing thing do *both* errors really point to? Name it.

*(You write every line here — I won't. None of these needs a runtime, because
the point of this lesson is what happens *before* a runtime ever gets involved.
When you later reach for tokio or async-std, this is the foundation:
`async`/`.await` **describe** the work; the runtime is what finally **does**
it.)*

## 6. What surprised you?

A sentence or two: did "a future does nothing until something drives it" land —
and did the `made the future` run (where `"doubling 21"` never prints) make it
concrete? Did the iterator parallel from Lesson 28 help? Were you surprised
that **both** `E0728` and `E0752` are really complaining about the same missing
thing? Tell me, and I'll fold it into the Phase-8 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.17 "Fundamentals of
  Asynchronous Programming," §17.1 "Futures and the Async Syntax"**: the
  central point that **futures are lazy** — they do nothing until driven
  (paraphrased, and likened here to iterators doing nothing until `.next()`) —
  plus the runnable runtime setup this offline lesson points at instead of
  reproducing.
- **CR** — *Comprehensive Rust* (Google), **§64 "Async Basics"**: the `Future`
  trait and `poll`, runtimes/executors, and tasks — the trait-level layer
  beneath this lesson's intuition, deferred to later study.
- Every snippet was compile-checked on **rustc 1.95.0**, edition 2024; both
  error captures are verbatim `rustc --edition 2024 FILE.rs` output (temp path
  normalized to `main.rs`). **No async output is fabricated:** async bodies
  cannot run here without a runtime, so the only execution output shown is the
  deliberately-lazy `made the future` — which proves the async body did *not*
  run. This closes Phase 8 (concurrency).

---

<!-- lesson-nav -->
[← Lesson 31 — Async syntax: async fn, .await & async blocks](31-async-syntax.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 32 — Trait objects: one collection, many types →](32-trait-objects.md)
