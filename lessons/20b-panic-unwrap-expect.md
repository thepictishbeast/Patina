# Lesson 20b — `panic!`, `unwrap` and `expect`: the blunt path

*(Phase 5, the finale continues. Lesson 20 handled failure politely. This lesson is
about the other choice: stopping the program on the spot — and the two everyday
methods that quietly make that choice for you.)*

## 1. Why it exists

Some failures aren't worth recovering from — a bug is a bug, and limping on with
wrong data makes things worse. **`panic!`** stops the current thread with a
message. And two methods you'll see everywhere — **`unwrap`** and **`expect`** —
are exactly that: "give me the `Ok` value, and `panic!` if it's an `Err`." They're
honest tools with a sharp edge, and this lesson is about feeling that edge once
on purpose.

## 2. The idea

- `panic!("message")` — stop now, print the message, unwind. For unrecoverable
  states.
- `.unwrap()` on a `Result` — the `Ok` value, or a panic with a standard message.
  The same call on an `Option` gives the `Some` value or panics on `None` — one
  habit, both types.
- `.expect("context")` — identical, but *your* message names which assumption
  broke. (Why that's usually the better choice is Book §9.2's discussion.)
- When is the blunt path *appropriate*? Prototypes, tests, and cases you can
  prove can't fail — the judgement call is the Book's "to `panic!` or not to
  `panic!`", §9.3, worth reading once you've felt both paths.

The key fact, and it echoes Lesson 5b: **this is a runtime crash, not a compile
error.** The compiler is perfectly happy with `unwrap` — it's your *values* that
decide whether it survives.

## 3. Watch it crash

This compiles fine, then stops when run:

```rust
fn main() {
    let n: i32 = "ferris".parse().unwrap();
    println!("{n}");
}
```

**Before you scroll — compile? run? what happens?**

```
thread 'main' panicked at main.rs:2:35:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
```

Read the panic like you read a compile error — *where* (line 2, the `unwrap`
call) and *why* (`Err`, and inside it the reason: `InvalidDigit`). `unwrap` said
"I'm sure this is `Ok`"; the string `"ferris"` said otherwise.

## 4. Common pitfalls — "it compiled" strikes again

The trap is exactly Lesson 5b's, wearing error-handling clothes: **compiling
proves the types line up; it cannot prove your `Result` is `Ok`.** Both matching
exercises below are runtime panics — one on a `Result`, one on an `Option` —
and for each the question is not "will it compile?" (it will) but "what happens
when it runs, and what will the panic line *say*?" Predict the message before
you run.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the two matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new blunt` works too.)* **Predict on paper before
each run.**

1. **Feel the edge.** Parse a bad string with `.unwrap()`. **Predict**: compile?
   run? what does the panic line name?
2. **Name your assumption.** Same code with `.expect("age should be a number")`.
   **Predict** how the panic message changes.
3. **The `Option` twin.** Call `.unwrap()` on a `None`. **Predict** the message —
   how does it differ from the `Result` one?

*(You write every line here — I won't. The predictions are your answer key. Next:
the elegant path — `?`, which refuses to crash and passes the problem up.)*

## 6. What surprised you?

A sentence or two: did seeing `unwrap` crash — and *name the reason* — change
when you'd reach for it? Tell me, and I'll pitch Lesson 20c to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §9.1 (`panic!`), §9.2
  (`unwrap`/`expect`), §9.3 ("to `panic!` or not to `panic!`").
- **CR** — *Comprehensive Rust* (Google): the `unwrap`/`expect` framing.
- Every snippet compiled and run, and every panic captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 20 — Result: errors are values](20-result.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 20c — The ? operator →](20c-question-mark.md)
