# Lesson 32c — States as types: broken states won't compile

*(Phase 9, part 3 — the sharpest move in the Book's OOP chapter. A value that
moves through stages — draft, then published — could carry an "am I published?"
flag you check at runtime. Rust prefers a stronger answer: give each stage its
OWN TYPE, and the wrong-stage operation isn't a bug you catch — it's a program
that won't compile.)*

## 1. Why it exists

Think about a blog post. While it's a **draft** you can edit it, but nobody
should be able to *read* it; once **published**, it can be read. The classic
way to model this is one `Post` type with a state field and methods that check
it — `content()` returns `""` if the post isn't published yet. That works, but
every method must remember to check, and forgetting is a silent runtime bug
(the Book builds exactly this "state pattern" first, then critiques it).

**States as types** is the Rust-native alternative: a `DraftPost` type and a
`PublishedPost` type. Each type carries **only the methods valid in that
state** — `DraftPost` simply *has no* `content()` method. "Read an unpublished
draft" stops being a case to handle and becomes a line of code that **cannot be
written**. The compiler does the checking, at zero runtime cost.

## 2. The idea

- **One type per state.** `DraftPost` and `PublishedPost` are separate structs,
  even if their data is identical. The *type* is the state.
- **Methods only where they're valid.** Reading lives on `PublishedPost` only.
  There is nothing to check and nothing to forget.
- **Transitions consume.** `publish(self)` takes `self` **by value** (not
  `&self` — Lesson 18b's distinction), so publishing *uses up* the draft and
  hands back a `PublishedPost`. The old state doesn't linger to be misused:
  ownership (Lesson 15) retires it.

> **How the sources frame it:** the **BOOK** Ch.18 §18.3 builds the runtime
> state pattern with trait objects first, then rewrites it this way —
> "encoding states and behavior as types" — and notes the trade: invalid
> states become **unrepresentable**, at the price of callers converting
> between types at each stage. This lesson teaches the destination directly.

## 3. A tiny example to read

**Two types, two stages (BOOK).** A draft can only `publish`; a published post
can be read. A draft *has no* `content()` at all:

```rust
struct DraftPost {
    text: String,
}

struct PublishedPost {
    text: String,
}

impl DraftPost {
    fn new(text: &str) -> DraftPost {
        DraftPost { text: String::from(text) }
    }

    // A draft cannot be read — there is no content() here.
    fn publish(self) -> PublishedPost {
        PublishedPost { text: self.text }
    }
}

impl PublishedPost {
    fn content(&self) -> &str {
        &self.text
    }
}

fn main() {
    let draft = DraftPost::new("hello world");
    let post = draft.publish();        // draft is consumed, becomes published
    println!("{}", post.content());
}
```

```
hello world
```

`publish` takes `self` **by value**, so it *consumes* the draft and hands back a
`PublishedPost`. The draft no longer exists to misuse — and reading it before
publishing is a compile error, next.

## 4. Common pitfalls / real compiler errors — the payoff

**Using a value in the wrong state — `E0599`.** Try to read a `DraftPost`
before publishing it:

```rust
struct DraftPost {
    text: String,
}

struct PublishedPost {
    text: String,
}

impl DraftPost {
    fn new(text: &str) -> DraftPost {
        DraftPost { text: String::from(text) }
    }
    fn publish(self) -> PublishedPost {
        PublishedPost { text: self.text }
    }
}

impl PublishedPost {
    fn content(&self) -> &str {
        &self.text
    }
}

fn main() {
    let draft = DraftPost::new("hello");
    println!("{}", draft.content());   // a draft has no content() — wrong state
}
```

```
error[E0599]: no method named `content` found for struct `DraftPost` in the current scope
  --> main.rs:26:26
   |
 1 | struct DraftPost {
   | ---------------- method `content` not found for this struct
...
26 |     println!("{}", draft.content());   // a draft has no content() — wrong state
   |                          ^^^^^^^ method not found in `DraftPost`
```

There's nothing to handle at runtime — the invalid state simply **can't be
written**. Instead of a method that checks "am I published?" and maybe fails,
the wrong call is a compile error. Invalid states become **unrepresentable**.

And notice the quieter guard: because `publish` consumed the draft, touching
`draft` *after* publishing trips Lesson 15's move rule — the retired state is
gone, not just discouraged. You'll predict that error in part 5.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new states` works too.)* **Predict on paper before each
run.**

1. **A state that can't be misused.** Model a two-stage value with **two
   types**. For example `LockedDoor` and `OpenDoor`: a `LockedDoor` has an
   `unlock(self) -> OpenDoor` method and **nothing else**; an `OpenDoor` has a
   `walk_through(&self)` that prints something. In `main`, make a locked door,
   `unlock` it, then `walk_through`. **Predict the output.**
2. **The wrong-state call.** Add a line that calls `walk_through` on the
   **locked** door (before unlocking). **Predict the error code** before
   compiling — and explain in one sentence why this is *better* than a single
   `Door` type with a runtime "is it open?" check.
3. **The retired state.** Remove that line. Now, *after* `let open =
   locked.unlock();`, add a second `locked.unlock();`. **Predict the error
   code** — you met it in Lesson 15. What did `unlock(self)` do to the locked
   door the first time, and why is that exactly what a state transition should
   do?

*(You write every line here — I won't. The predictions are your answer key.
That closes the OOP trio: mixed types behind one method — 32; hidden internals
that are free to change — 32b; and states the compiler retires for you — 32c.
Next: advanced patterns and matching.)*

## 6. What surprised you?

A sentence or two: did the `E0599` on a draft feel different from a runtime
check — a broken state that **can't be written** rather than one you catch
later? And did task 3 change how you read `self` (no `&`) in a signature: not
a detail, but the mechanism that retires the old state? Tell me, and I'll fold
it into the Phase-9 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.18 §18.3** "Implementing an
  Object-Oriented Design Pattern": the runtime state pattern, its critique, and
  the rewrite this lesson teaches — **encoding states as types**, the
  `DraftPost`/`PublishedPost` example where "invalid states are now impossible
  because of the type system."
- **CR** — *Comprehensive Rust* (Google): touches typestate in passing; the
  full arc is the Book's.
- Every snippet compiled and run, and the error captured live, on **rustc
  1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`; temp path normalized
  to `main.rs`). This continues Phase 9 (advanced).

---

<!-- lesson-nav -->
[← Lesson 32b — Encapsulation: private fields, public methods](32b-encapsulation.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 33 — Refutability: when a pattern can fail →](33-refutability.md)
