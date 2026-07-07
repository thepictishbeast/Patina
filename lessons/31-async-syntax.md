# Lesson 31 — Async syntax: `async fn`, `.await` & async blocks

*(Phase 8 — Concurrency, part 4. Threads — Lessons 30 to 30c — give you
**parallelism**: work running at the same time on different cores. Async is a
different tool for a different problem: **waiting**. This lesson is pure shape:
the three pieces of async syntax and what each one means. It ends on a question
it deliberately does not answer — Lesson 31b delivers the one honest fact that
resolves it.)*

## 1. Why it exists

A program often spends most of its time **waiting** — for data to arrive, for a
reply to come back. A blocked thread that just sits there waiting is wasted: it
holds memory and does no work. You *could* spawn a thread per waiting task, but
threads are relatively heavy, and most of them would be asleep anyway.

**Async** is lightweight concurrency built for exactly this case. You mark code
that might need to wait as `async`. At each waiting point you write `.await`,
which means *"if the thing I need isn't ready yet, pause me here and let other
work run; wake me when it is."* Many such tasks take turns on a single thread,
each yielding whenever it would otherwise block. Nobody sits idle.

The key distinction to hold onto:

- **Threads** = **parallelism** — work running *at the same time* (good for CPU work).
- **Async** = **cooperative concurrency** — tasks taking turns, each stepping aside
  while it waits (good for I/O). On its own it is **not** parallelism: by default
  it's interleaving on one thread, not running on many cores.

## 2. The idea — three pieces of syntax

- **`async fn`** — mark a function `async` and it no longer returns its value
  directly. It returns a **future**: a value that *represents* the eventual
  result. The body doesn't run when you call the function — it's packaged up
  for later. So `async fn double(x: u32) -> u32` is really a function returning
  `impl Future<Output = u32>`.
- **`async { ... }`** — an **async block**. It's an expression whose value is a
  future, the same way `async fn` produces one. Useful when you want a future
  without naming a whole function.
- **`.await`** — postfix, written after a future: `some_future.await`. It says
  *"run this future to completion; if it's not ready, pause me and let other
  tasks run until it is, then give me its value."* Crucially, **`.await` is only
  legal inside an `async fn` or `async` block** — Lesson 31b shows you the
  error that guards this.

> **How to read `impl Future<Output = u32>`:** "some future whose eventual
> value is a `u32`." The `Output = …` part is *associated-type* syntax, which
> Lesson 34 teaches properly — until then you only ever need to **read** it, or
> copy it exactly; you never have to invent it.

> **How the sources frame it:** the **BOOK** Ch.17 §17.1 "Futures and the Async
> Syntax" is the backbone — it introduces a **future** as a value that isn't
> ready yet but *will* be, and shows that `async fn` is **sugar** for a function
> returning a future. This lesson covers that syntax half; Lesson 31b covers
> the other half of the same section.

## 3. Tiny examples to read

Each was checked with `rustc --edition 2024 --crate-type lib`; none is run —
part 4 and Lesson 31b explain why that isn't a cop-out but the point.

**`async fn` is sugar for "returns a future."** The two functions below have the
**same** return type. `double` is `async`; `_check` spells out by hand what the
`async` keyword produced:

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

This **compiles.** Note what is *not* here: no `21` and no `42`.

**`.await` chains futures inside an async fn.** `.await` is postfix, and legal
only because `add_doubles` is itself `async`. Each `.await` is a point where the
task would pause if the awaited future weren't ready:

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

This **compiles.** And `add_doubles` is just *another* async fn — calling it
would hand you yet another future.

**`async { ... }` is a future too.** An async block is an expression that
evaluates to a future, so a normal function can return one:

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

This **compiles.** Notice the question all three examples keep raising:
everything type-checks, yet no doubled number and no `14` has appeared anywhere.
**Has any async body actually *run*?** Hold that question — it is the whole of
Lesson 31b.

## 4. Common pitfalls / real compiler errors — a future is not the value

Treat the call's result as the number it will *eventually* produce, and the
compiler corrects you in exactly the right words:

```rust
async fn double(x: u32) -> u32 {
    x * 2
}

fn main() {
    let n: u32 = double(21);
    println!("{n}");
}
```

**Before you scroll — what type did `double(21)` actually give us?**

```
error[E0308]: mismatched types
 --> main.rs:6:18
  |
6 |     let n: u32 = double(21);
  |            ---   ^^^^^^^^^^ expected `u32`, found future
  |            |
  |            expected due to this
```

"Expected `u32`, **found future**" — the compiler itself states the rule:
calling an async fn gives you the *package*, not the result. Getting the `u32`
*out* requires `.await` (inside async code) — or the missing piece Lesson 31b
names.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine,
`rustc --edition 2024` works too.)* **Predict on paper before each run.**

1. **A future is not the value.** Write `async fn answer() -> u32` returning
   `42`. In a normal `fn main`, write `let n: u32 = answer();`. **Predict the
   error code** and the two types its message will name. Then change the line
   to `let _n = answer();` (let the compiler infer) — does it compile *now*?
2. **Two return types, one meaning.** Keep `answer`. Add this function, typing
   the signature **exactly** as written (it's the read-only syntax from part 2;
   you'll need `use std::future::Future;`):
   `fn also_answer() -> impl Future<Output = u32> { answer() }`
   **Predict**: does this compile? What does the `async` keyword on `answer`
   correspond to in `also_answer`'s written-out return type?
3. **Await where it's allowed.** Write an `async fn` that does
   `let n = answer().await;` and returns `n + 1`. **Predict**: does it compile?
   What single word in its signature makes the `.await` legal?
4. **A block instead of a fn.** Make a plain `fn` return a future using an
   `async { ... }` block whose body adds two numbers (part 3 shows the shape).
   **Predict**: does the addition happen when you call the function?

*(You write every line here — I won't. Notice that every task is about reading
the **shape** of async code — and that nothing you built in tasks 2–4 ever
produced a visible number. Whether it ran at all is Lesson 31b's opening move.)*

## 6. What surprised you?

A sentence or two: does "calling an `async fn` hands you a package, not a
result" sit comfortably yet — and did the compiler saying **"found future"** in
part 4 make it concrete? Is the threads-vs-async split (parallelism vs
cooperative waiting) clear enough that you can say *when* you'd pick each? Tell
me, and I'll fold it into the Phase-8 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.17 "Fundamentals of
  Asynchronous Programming," §17.1 "Futures and the Async Syntax"**: a *future*
  as a value that becomes ready later, and `async fn` as **sugar** for a
  function returning `impl Future` (paraphrased). The laziness half of that
  same section is Lesson 31b.
- **CR** — *Comprehensive Rust* (Google), **§64 "Async Basics"**: the `Future`
  trait, `poll`, and runtimes — the trait-level layer beneath this lesson,
  deferred to later study.
- Every snippet was compile-checked on **rustc 1.95.0**, edition 2024 (the
  part-3 examples with `--crate-type lib`; the part-4 error captured verbatim
  from a full `rustc --edition 2024 FILE.rs` run, temp path normalized to
  `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 30c — Shared state: Arc & Mutex](30c-shared-state.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 31b — Futures are lazy: why async needs a runtime →](31b-futures-are-lazy.md)
