# Lesson 20 — `Result<T, E>`: errors are values

*(Phase 5 — Custom types & matching, the finale begins. Lesson 19b gave you
`Option<T>` for "a value might be **missing**." Its sibling `Result<T, E>` is for
"an operation might **fail**, and here's **why**.")*

## 1. Why it exists

Things go wrong. A string won't parse into a number, a file isn't where you
expected. Many languages handle this with hidden *exceptions* that jump out of
your code from somewhere you can't see. Rust doesn't. In Rust an error is an
**ordinary value**, right there in the function's return type, and it splits
failures into two kinds:

- **Unrecoverable** — a bug, like indexing past the end of a vector. The program
  should just stop. That's **`panic!`** (Lesson 20b).
- **Recoverable** — something you *expect* can fail, like bad user input. The
  function hands the problem back to its caller as a **`Result`**, and the caller
  decides what to do. That's this lesson.

Because the failure is written in the type, you can't *forget* it's possible —
the same safety `Option` gave you, now carrying a reason as well as a yes/no.

## 2. The idea

**`Result<T, E>`** is an enum with two variants (you know enums now — Lesson 19):

```
enum Result<T, E> {
    Ok(T),    // success — holds the value you wanted
    Err(E),   // failure — holds an error describing what went wrong
}
```

(`T` and `E` are placeholders, exactly like the `T` in `Option<T>` — Lesson 19b's
read-as rule applies, and you'll write your own in Lesson 24.)

`Ok` and `Err` are in the prelude, so you write them bare. A function that can
fail says so in its signature: `fn parse(...) -> Result<i32, ParseIntError>`.

**Reading a `Result` — the explicit ways:**

- **`match`** on `Ok`/`Err` — handle both cases yourself (just like matching an
  `Option`).
- **`.unwrap_or(default)`** — give me the value, or this fallback if it was `Err`.

(The blunt way — `unwrap`/`expect`, which *crash* on `Err` — is Lesson 20b; the
elegant way — `?`, which passes the error up — is Lesson 20c.)

## 3. Tiny examples to read

**Handle a `Result` with `match`.** `str::parse` returns a `Result`:

```rust
fn main() {
    let input = "42";
    match input.parse::<i32>() {
        Ok(n) => println!("parsed: {n}"),
        Err(e) => println!("not a number: {e}"),
    }
}
```

```
parsed: 42
```

> Read `parse::<i32>()` as "parse, into an `i32`." The `::<…>` is just a way to
> tell `parse` which type to aim for when Rust can't infer it (nicknamed the
> *turbofish*; the machinery behind it is Lesson 24's).

(Feed it `"oops"` instead and the `Err` arm runs — `not a number: invalid digit
found in string`.)

**A fallback with `unwrap_or`.** When you just want a default on failure:

```rust
fn main() {
    let good: i32 = "42".parse().unwrap_or(0);
    let bad: i32 = "oops".parse().unwrap_or(0);
    println!("{good} {bad}");
}
```

```
42 0
```

## 4. Common pitfalls / real compiler errors — a `Result<T, E>` is not a `T`

The same wall as Lesson 19b's `Option`, one step richer. `parse` gives you a
*maybe-failed* number, not a number:

```rust
fn main() {
    let n: i32 = "42".parse();
    println!("{n}");
}
```

**Before you scroll — does this compile?**

No — `error[E0308]: mismatched types` — "expected `i32`, found
`Result<i32, ParseIntError>`". The failure case exists whether you acknowledge it
or not; the type makes you get the value *out* (with `match`, `unwrap_or`, or the
tools of the next two lessons) before you can use it. The matching exercise below
is exactly this wall — **predict the code** before you run.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new results` works too.)* **Predict on paper
before each run.**

1. **Handle it with `match`.** Take one `parse()` result and write a `match` that
   prints `"got <n>"` on `Ok` and `"bad input"` on `Err`. **Predict** what it
   prints for `"7"` and for `"seven"`.
2. **Same job with a fallback.** Rewrite it using `unwrap_or(-1)` — **predict**
   both outputs again.
3. **Skip the handling.** Bind `"42".parse()` straight to an `i32`. **Predict**
   the error code.

*(You write every line here — I won't. The predictions are your answer key. Next:
the blunt path — `panic!`, `unwrap`, and `expect`.)*

## 6. What surprised you?

A sentence or two: did "an error is just a value in the return type" land
differently from hidden exceptions? Tell me, and I'll pitch Lesson 20b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §9.2 (`Result<T, E>`, matching on
  it, `unwrap_or`).
- **CR** — *Comprehensive Rust* (Google), error-handling chapter: the `Result`
  variants slide.
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 19d — Concise matching](19d-concise-matching.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 20b — panic!, unwrap and expect →](20b-panic-unwrap-expect.md)
