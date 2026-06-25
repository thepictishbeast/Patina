# Lesson 27 — Closures: Anonymous Functions That Capture

*(Phase 7 — Functional features & smart pointers, part 1. Until now a piece of
behaviour you wanted to reuse had to be a named `fn` (Lesson 7). A **closure** is
a lighter thing: an unnamed function you write inline, hand to another function,
or save in a variable — and, unlike a plain `fn`, it can reach out and grab values
from the code around it. That capturing is the whole point, and it leans on
everything you learned about ownership and borrowing in Lessons 15–16.)*

## 1. Why it exists

Sometimes you need a small piece of behaviour *right where you are* — a rule for
sorting, a thing to do on each item of a list, a fallback to compute on failure.
Defining a whole separate named `fn` for it is heavy: you have to name it, and a
plain `fn` can't see any of the local variables sitting next to it.

A **closure** solves both. It's an **anonymous function** — no name needed — that
you can save in a variable or pass as an argument. And it can **capture** the
variables in the scope where you wrote it, so the values it needs are already in
hand. You write the behaviour once, inline, next to the data it works on.

> **How the sources frame it:** the **BOOK** Ch.13.1 is the backbone — it defines a
> closure as "anonymous functions you can save in a variable or pass as arguments,"
> and teaches capture through one anchor idea: the three ways a closure can grab a
> value are the same three ways a function takes a parameter — borrow it, borrow it
> mutably, or take ownership. **CR** adds the crisp rule about the `Fn`/`FnMut`/`FnOnce`
> trait family (part 2). No metaphor is invented here — the "it's a function that can
> see its surroundings" idea carries itself.

## 2. The idea

**The syntax.** A closure is written with a pair of bars holding its parameters,
then its body:

```
|x| x + 1            // takes x, returns x + 1
|a, b| a + b         // takes two, returns their sum
|| println!("hi")    // takes nothing (empty bars)
```

Save it in a variable and call it with parentheses, exactly like a function:

```
let add_one = |x| x + 1;
add_one(41);         // 42
```

You usually don't write the parameter or return types — the compiler reads them
off the body and the first use. (A named `fn` always makes you spell them out; a
closure lets you skip them.)

**Capturing — the part that makes it a closure.** A closure can use variables from
the surrounding code without them being passed in. *How* it grabs each one depends
on what the body does with it, and it mirrors the three ways a function takes a
parameter:

- **Borrow immutably** — the body only *reads* the value. The original stays usable.
- **Borrow mutably** — the body *changes* the value (e.g. pushes to a vector).
- **Take ownership** — the value is *moved* into the closure.

The compiler picks the **least demanding** option that the body needs. A closure
that only prints a list borrows it immutably; one that pushes to it borrows it
mutably. You don't ask for a mode — the body decides.

**The `move` keyword.** Put `move` before the bars and you *force* the closure to
take **ownership** of everything it captures, even if the body would have been
happy just borrowing:

```
move || println!("{list:?}")
```

The main reason you'd need this: handing the closure to something that outlives the
current scope — most often a new thread — where a borrow wouldn't be guaranteed to
stay valid. `move` says "the closure keeps these values; they go with it."

That's the kernel: bars for the parameters, a body that quietly captures what it
needs, and `move` when you must hand ownership over. The `Fn`/`FnMut`/`FnOnce`
trait names that describe *which* capture a closure uses are the subject of part 2;
here, just notice the three modes in the examples.

## 3. Tiny examples to read

**Capture by immutable borrow — the value stays usable.** The body only reads
`list`, so the closure borrows it immutably; `list` is still available before and
after:

```rust
fn main() {
    let list = vec![1, 2, 3];
    let only_borrows = || println!("from closure: {list:?}");
    println!("before call: {list:?}");
    only_borrows();
    println!("after call:  {list:?}");
}
```

```
before call: [1, 2, 3]
from closure: [1, 2, 3]
after call:  [1, 2, 3]
```

**Capture by mutable borrow — the body changes the value.** Pushing to `list`
needs a mutable borrow, so the closure takes one. Note `list` is `mut`, and the
*closure variable* is `mut` too (it holds a mutable borrow):

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    let mut adds_one = || list.push(7);
    adds_one();
    println!("after call: {list:?}");
}
```

```
after call: [1, 2, 3, 7]
```

**A closure with a parameter, passed to a method.** Many standard-library methods
take a closure. Here `map` calls `|n| n * 2` once per element. The closure captures
nothing — it just transforms its argument:

```rust
fn main() {
    let add_one = |x| x + 1;
    println!("{}", add_one(41));

    let nums = [1, 2, 3];
    let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();
    println!("{doubled:?}");
}
```

```
42
[2, 4, 6]
```

**Force ownership with `move` — handing data to a thread.** A new thread might
outlive `main`'s use of `list`, so a borrow won't do; `move` gives the thread its
own copy of ownership. (`join().unwrap()` just waits for the thread to finish.)

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    thread::spawn(move || println!("from thread: {list:?}"))
        .join()
        .unwrap();
}
```

```
from thread: [1, 2, 3]
```

Same printing body as the first example — the only change is `move`, which switches
the capture from "borrow" to "own."

## 4. Common pitfalls / real compiler errors

**Moving a captured value *out* of a closure that gets called repeatedly — `E0507`.**
`sort_by_key` calls its closure once per element, so the closure must be reusable.
This one tries to *give away* (move) the captured `value` on each call — which can
only happen once — so the compiler refuses:

```rust
fn main() {
    let mut list = [3, 1, 2];
    let mut log = Vec::new();
    let value = String::from("sorted");
    list.sort_by_key(|n| {
        log.push(value);
        *n
    });
    println!("{list:?}");
}
```

```
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
 --> main.rs:6:18
  |
4 |     let value = String::from("sorted");
  |         -----   ---------------------- move occurs because `value` has type `String`, which does not implement the `Copy` trait
  |         |
  |         captured outer variable
5 |     list.sort_by_key(|n| {
  |                      --- captured by this `FnMut` closure
6 |         log.push(value);
  |                  ^^^^^ `value` is moved here
  |
help: consider cloning the value if the performance cost is acceptable
  |
6 |         log.push(value.clone());
  |                       ++++++++
```

The phrase **"an `FnMut` closure"** is the clue to part 2: because `sort_by_key`
calls the closure many times, the closure has to be one that *can* be called many
times — and a closure that moves a value out can only run **once**. The fix the
compiler suggests (`value.clone()`) hands the closure its own copy each time, so
nothing is moved out of the environment.

**Using a value in `main` after `move` took it — `E0382`.** `move` transfers
ownership of `list` *into* the closure. After that, `main` no longer owns it, so the
last line can't use it:

```rust
fn main() {
    let list = vec![1, 2, 3];
    let owns_it = move || println!("from closure: {list:?}");
    owns_it();
    println!("back in main: {list:?}");
}
```

```
error[E0382]: borrow of moved value: `list`
 --> main.rs:5:30
  |
2 |     let list = vec![1, 2, 3];
  |         ---- move occurs because `list` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     let owns_it = move || println!("from closure: {list:?}");
  |                   -------                          ---- variable moved due to use in closure
  |                   |
  |                   value moved into closure here
4 |     owns_it();
5 |     println!("back in main: {list:?}");
  |                              ^^^^ value borrowed here after move
  |
help: consider cloning the value before moving it into the closure
```

This is the ownership rule from Lesson 15, now triggered by `move`: once a value is
moved, the old name is dead. Drop the `move` (the closure would borrow instead and
`list` stays usable), or clone before moving if you genuinely need both.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new closures`. **Predict on paper before each run.**

1. **The three capture modes, by hand.** Write three closures over a `let mut v =
   vec![10, 20, 30]`: one that only *prints* `v`, one that *pushes* a number onto
   `v`, and one written with `move` that prints `v`. Call each. **Predict** which
   of the three leaves `v` usable on the line *after* the closure call, and which
   does not — then check by trying to `println!("{v:?}")` afterwards. (One of them
   will refuse to compile; predict which, and why.)

2. **A closure as an argument.** Make an array of a few numbers. Use `.iter()` then
   `.map(...)` with a closure that triples each value, then `.collect()` into a
   `Vec<i32>`. **Predict** the printed vector before you run it. Then change the
   closure to *add* the index instead of tripling (hint: `.enumerate()` gives you
   `(i, n)` pairs) and predict again.

3. **Trigger `E0382` on purpose.** Write a closure that uses `move` to capture a
   `String`, call it, then try to print the original `String` in `main`.
   **Predict the error code** before compiling. Then fix it *two* ways: once by
   removing `move`, and once by cloning the `String` before the closure. Which fix
   matches what you actually wanted — to share the value, or to hand it over?

*(You write every line here — I won't. The predictions are your answer key; the code
is yours. The thing to feel by the end: you never *told* a closure how to capture —
the body did, and `move` was the one override you reached for.)*

## 6. What surprised you?

A sentence or two: did "an unnamed function that can see the variables around it"
land? Did it click that you don't choose the capture mode — the body does, and the
compiler picks the lightest one? Did the `E0507` "an `FnMut` closure" wording make
you curious what `FnMut` even *is* (that's the next lesson)? Did `move` feel like
the same ownership rule from Lesson 15 showing up in a new place? Tell me, and I'll
fold it into the Phase-7 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.13.1** ("Closures: Anonymous
  Functions That Capture Their Environment"): the definition ("anonymous functions
  you can save in a variable or pass as arguments"), the three capture modes mapped
  to the three ways a function takes a parameter (Listings 13-4 immutable borrow,
  13-5 mutable borrow, 13-6 `move` into a thread), and the `E0507` move-out-of-an-
  `FnMut`-closure failure (Listing 13-8). My snippets adapt these listings.
- **CR** — *Comprehensive Rust* (Google), closures section: the framing of the
  `Fn`/`FnMut`/`FnOnce` trait family that part 2 picks up. Not yet detailed here —
  this lesson stays on the three capture modes and `move`.
- **BLOG** — not used; it lists closures only as a future "learn next" topic.
- Every snippet compiled and run, and every error captured live, on **rustc 1.95.0**,
  edition 2024 (`rustc --edition 2024 FILE.rs`). The E0507 output had one
  environment-specific stdlib path line trimmed; error code, message, and spans are
  verbatim. Next in Phase 7: the `Fn`/`FnMut`/`FnOnce` traits in full, then iterators
  (L28) — where closures passed to `map`/`filter` become the everyday tool.

---

<!-- lesson-nav -->
[← Lesson 26 — Lifetimes: annotations (`'a`), the borrow checker, elision](26-lifetimes.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 28 — Iterators →](28-iterators.md)
