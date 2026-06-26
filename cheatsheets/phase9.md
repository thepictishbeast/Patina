# Phase 9 Cheatsheet — Advanced (Trait Objects, Patterns, Features, Capstone)

Quick reference (pairs with the Phase-9 Advanced lessons — L32 trait objects & OOP · L33
advanced patterns & refutability · L34 `unsafe`/traits/macros · L35 the multithreaded
web-server capstone). The shape of it: a **trait object** (`dyn Trait` behind a pointer)
holds **many** concrete types behind one shared method, chosen at runtime; **patterns** get
guards, `@`-bindings, and a refutability rule that decides where each pattern is legal;
**`unsafe`** opens a small audited hole of exactly five abilities (it does **not** turn off
the borrow checker); **`macro_rules!`** writes code before compilation; and the **capstone**
combines closures + threads + `Arc<Mutex>` + `Box<dyn FnOnce>` into a thread pool. Verified
on rustc 1.95.0, edition 2024.

## Trait objects — `Box<dyn Trait>` / `&dyn Trait` (dynamic dispatch)
- **`dyn Trait` never stands alone — it lives behind a pointer.** `Box<dyn Trait>` = an **owned** trait object on the heap (goes in `Vec<Box<dyn Draw>>`); `&dyn Trait` = a **borrowed** one (e.g. `fn show(item: &dyn Draw)` accepts *any* `Draw`).
- **The mixed collection:** `Vec<Box<dyn Draw>>` holds a `Button` **and** a `SelectBox` side by side; one loop calls each value's own `draw()`. A generic `Vec<T>` (`T: Draw`) is locked to **one** type — every element the same kind. That's the whole reason trait objects exist.
- **Fat pointer:** a trait-object reference carries **two** addresses — data + **vtable** (the per-type method table). So `&Button` = **8 bytes**, `&dyn Draw` = **16 bytes** on 64-bit. The vtable is how mixed types each find their own method at runtime.
- **Static vs dynamic dispatch:** generics / `impl Trait` (L25) decide the method at **compile** time, one copy of the code **per type**, hold **one** type. `dyn Trait` looks it up at **runtime** (vtable), **one** shared copy, holds **many** types — at a small per-call lookup cost.
- **Object-safety, in one line:** a trait is usable behind `dyn` only if every method is callable through the pointer alone — a **generic method breaks it** (no single vtable entry) → **`error[E0038]`** ("the trait `…` is not dyn compatible"). Keep `dyn` methods plain (`&self`, no generics).
- **Which to reach for:** default to **static dispatch** (faster, compiler knows more); for a **closed, known** set of types prefer an **`enum`**; reach for `dyn` only when the set is **open** or you need a **mixed collection**. Beginners over-use `dyn` — prefer it last.

## OOP in Rust — encapsulation & states-as-types
- **Encapsulation:** make fields **private**, expose only **methods** (`pub fn`). Callers depend on the methods, so you can change internals freely (`AveragedCollection` keeps a cached `average` correct via a private `update_average`).
- **States as types:** give each stage its own type with only the methods valid then. A `DraftPost` **has no** `content()`; `publish(self)` consumes it and returns a `PublishedPost`. Reading a draft is then a **compile error** (`E0599`), not a runtime check — invalid states become **unrepresentable**.

## Patterns — guards, `@`, nesting, and refutability
- **Refutable vs irrefutable:** irrefutable = *always* matches (`(a, b)`, `x`); refutable = *can fail* (`Some(x)` misses `None`; `3` misses `4`).
- **The rule (where each is legal):** `let PAT = …`, `for PAT in …`, and function params need **irrefutable** patterns (no else-branch to take on a miss). `match` arms, `if let`, and `while let` allow **refutable** ones (other arms / the `else` / loop-exit are the miss path).
  - `let Some(x) = opt;` → **`error[E0005]`** ("refutable pattern in local binding"). Fix: add an else path — `if let` or `let … else { return; }`.
  - The **inverse** — an irrefutable pattern in `if let` (e.g. `if let (a, b) = …`) — is only a **warning** (`irrefutable_let_patterns`), not an error; it builds. Advice: drop the `if`, use a plain `let`.
- **Match guards:** an `if` tacked onto an arm — `Some(x) if x < 0 => …`. Tests a *value*, not just a shape. Over a `|` pattern the guard covers the **whole** alternation: `(4 | 5 | 6, active) if active`.
- **`@` bindings:** `name @ pattern` captures the value **while** testing it. `id @ 3..=7` = "is `id` in 3..=7? if so, bind it as `id`." Test *and* keep, in one move.
- **Nested destructuring:** patterns nest as deep as the data. `Some(Point { x: 0, y })` matches only a `Some` holding a `Point` whose `x` is literally `0`, binding `y`.
- **Arm order matters:** the first matching arm wins, so write the narrower pattern (e.g. `@ 3..=7`) **before** a broader guard arm.
- **Recap (L19):** `|` = "or" (`1 | 2 | 3`); `..=` = an **inclusive** range (`4..=9` is 4 through 9). (You may still *see* `ref` in old code; edition-2024 match ergonomics handle it — recognise it, don't learn it.)

## Advanced features — `unsafe`, operator overloading, `macro_rules!`
- **`unsafe { }` unlocks exactly five superpowers and nothing else:** (1) **dereference a raw pointer** (`*const T` / `*mut T`); (2) **call an `unsafe` fn/method**; (3) **access/modify a mutable `static`**; (4) **implement an `unsafe` trait**; (5) **access `union` fields**.
- **It does NOT turn off the borrow checker.** Ownership, borrowing, and type-checking still apply inside the block — a borrow violation beside a genuine `unsafe` deref still fails (`E0502`). `unsafe` shifts the burden of upholding the rules to *you*; it doesn't switch them off.
- **Raw pointers:** make them with `&raw const x` (→ `*const T`) and `&raw mut x` (→ `*mut T`) — **making** one is safe; **following** one (`*r`) needs `unsafe`. Deref outside `unsafe` → **`error[E0133]`** ("dereference of raw pointer is unsafe and requires unsafe block").
- **Operator overloading:** implement a `std::ops` trait and the operator works on your type — `impl Add for Point { type Output = Point; fn add(self, other) -> … }` makes `+` work; `a + b` is just sugar for `Add::add`. `type Output` is an **associated type** (a type chosen by the implementor, like `Iterator`'s `Item`).
- **`macro_rules!` (declarative macro):** write *patterns* like `match` arms; the compiler pastes in matching code **before** compilation. `$x:expr` captures one expression; `$( … ),*` = "repeated, comma-separated"; `$( v.push($x); )*` in the body emits one line **per** captured expression. A 3-arg `my_vec![1, 2, 3]` expands to make-a-`Vec` + three `push`es — the repetition loops, no `for`. (Macros, e.g. `println!`/`vec!`/`assert!`, end in `!`.)
- **The honest summary:** you can write real Rust for a long time without `unsafe`, custom operators, or hand-written macros — this corner is for **recognition**, not daily use.

## Capstone — the multithreaded web server (conceptual; compile-check only)
- **Shape:** `TcpListener::bind("127.0.0.1:7878")` → loop `listener.incoming()` (each item a `TcpStream`). Don't handle inline (single-threaded = one slow request blocks all); hand work to a **`ThreadPool`**: `pool.execute(|| handle_connection(stream))`.
- **The pool:** `ThreadPool::new(4)` spawns four **worker** threads up front and holds the *sending* half of an `mpsc` channel. `execute` **boxes** your closure into a `Job` and sends it.
- **The job type:** `type Job = Box<dyn FnOnce() + Send + 'static>` — a **heap-boxed closure**: `dyn` (concrete type unknown), `FnOnce` (runs once), `Send` (can cross to another thread), `'static` (lives long enough).
- **Sharing the receiver — `Arc<Mutex<Receiver>>`.** All workers must pull jobs from the *one* receiving half. A `Receiver` isn't `Copy` and isn't meant to be cloned for many consumers, so giving each thread the receiver directly **fails to compile** — **`error[E0382]`** ("use of moved value: `receiver`"): the first `spawn` moves it, the next iteration has nothing left. Fix: wrap it `Arc<Mutex<Receiver>>` — **`Arc`** = multiple owners across threads, **`Mutex`** = one worker takes a job at a time.
- **Keep the lock short:** `receiver.lock().unwrap().recv()` takes the lock, gets a job, and the temporary guard is dropped **before** `job()` runs — so a long job doesn't block the other workers on the mutex. (Holding the lock across `job()` would serialize the whole pool.)
- **Graceful shutdown:** a `Drop` impl on `ThreadPool` drops the sender (each worker's `recv()` then returns `Err`, breaking its loop) and `join`s every worker thread.
- **It pulls together:** closures (L27), threads + channels + `Arc<Mutex>` (L30), `Box<dyn Trait>` (L29 + L25), `Option::take` (L19). If you can read this, you can read most real Rust.

— *Sources:* BOOK Ch.18 "OOP Features of Rust", Ch.19 "Patterns and Matching", Ch.20
"Advanced Features" & "Final Project: Building a Multithreaded Web Server" · CR trait-object
(fat-pointer / dyn-compatibility), pattern, and `unsafe` slides. Snippets verified on rustc
1.95.0, edition 2024 — trait-object, pattern, and feature snippets compiled **and run**
(deterministic outputs); the capstone snippets are **compile-checked only** (the server binds
a TCP port), so no server output is shown or fabricated. The `&raw const` / `&raw mut`
raw-borrow operators (stable since 1.82) replace the older `&x as *const T` cast; E0038 prints
"not **dyn compatible**" on 1.95.
