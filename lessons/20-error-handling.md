# Lesson 20 — Error Handling: `Result`, `?`, `panic!`

*(Phase 5 — Custom types & matching, part 3 and the finale. Lesson 19 gave you
`Option<T>` for "a value might be **missing**." Its sibling `Result<T, E>` is for
"an operation might **fail**, and here's **why**." Same idea — a failure is a value
the type system makes you face — one step richer.)*

## 1. Why it exists

Things go wrong. A string won't parse into a number, a file isn't where you expected.
Many languages handle this with hidden *exceptions* that jump out of your code from
somewhere you can't see. Rust doesn't. In Rust an error is an **ordinary value**, right
there in the function's return type, and it splits failures into two kinds:

- **Unrecoverable** — a bug, like indexing past the end of a vector. The program should
  just stop. That's **`panic!`**.
- **Recoverable** — something you *expect* can fail, like bad user input. The function
  hands the problem back to its caller as a **`Result`**, and the caller decides what to do.

Because the failure is written in the type, you can't *forget* it's possible — the same
safety `Option` gave you, now carrying a reason as well as a yes/no.

> **How the sources frame it:** the **BOOK** Ch.9 is the backbone — `panic!` vs `Result`,
> the `match`-on-`Result` shape, the `?` operator for propagation, and the "to panic or not
> to panic" judgement. **CR** contributes the crisp `Result` slide and the
> `unwrap`/`expect` framing. No metaphor is invented here — the concepts carry themselves.

## 2. The idea

**`Result<T, E>`** is an enum with two variants (you know enums now — Lesson 19):

```
enum Result<T, E> {
    Ok(T),    // success — holds the value you wanted
    Err(E),   // failure — holds an error describing what went wrong
}
```

`Ok` and `Err` are in the prelude, so you write them bare. A function that can fail says so
in its signature: `fn parse(...) -> Result<i32, ParseIntError>`. The `T` is the success
type; the `E` is the error type.

**Reading a `Result` — four ways, from most explicit to most blunt:**

- **`match`** on `Ok`/`Err` — handle both cases yourself (just like matching an `Option`).
- **`.unwrap_or(default)`** — give me the value, or this fallback if it was `Err`.
- **`.unwrap()` / `.expect("message")`** — give me the value, and **`panic!` if it was
  `Err`**. Quick for prototypes or when you're *certain* it's `Ok`; a crash if you're wrong.
  (This is the same trade-off as calling `.unwrap()` on a `None` — convenient, but it stops
  the program.)
- **The `?` operator** — propagate the error up to *your* caller, and keep going on success.

**The `?` operator** is the one worth slowing down for. Inside a function that *itself*
returns a `Result`, writing `value?` means:

> "If `value` is `Err`, **return that `Err` from this function right now**. If it's `Ok`,
> unwrap it and carry on."

It turns a pile of `match` statements into one character, threading errors up the call chain.
The catch: `?` only works in a function whose return type can carry the error — a `Result`
(or an `Option`). Use it in a plain function and the compiler stops you (part 4).

**`panic!`** is the other path: `panic!("the message")` stops the current thread with your
message. `unwrap`/`expect` on an `Err` do exactly this for you with a standard message.

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

(Feed it `"oops"` instead and the `Err` arm runs — `not a number: invalid digit found in string`.)

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

**Propagate with `?`.** The function returns a `Result`, so `?` can early-return the error:

```rust
fn double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;   // Err → return it now; Ok → n gets the value
    Ok(n * 2)
}

fn main() {
    println!("{:?}", double("21"));
    println!("{:?}", double("nope"));
}
```

```
Ok(42)
Err(ParseIntError { kind: InvalidDigit })
```

Notice `double` never writes a `match` — `?` did the "if error, bail out" for it, and the
success path reads as if nothing could go wrong.

## 4. Common pitfalls / real compiler errors

**Using `?` in a function that can't carry the error — `E0277`.** `?` needs a `Result`
(or `Option`) return type to return *into*:

```rust
fn first_number(s: &str) -> i32 {
    let n: i32 = s.parse()?;
    n
}
```

```
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> main.rs:2:27
  |
1 | fn first_number(s: &str) -> i32 {
  | ------------------------------- this function should return `Result` or `Option` to accept `?`
2 |     let n: i32 = s.parse()?;
  |                           ^ cannot use the `?` operator in a function that returns `i32`
```

The fix is to make the function return a `Result` (so the `?` has somewhere to send the
error), or to handle the `Result` right here with a `match`. The error message tells you
exactly which function is in the way.

**`unwrap`/`expect` on an `Err` — a runtime `panic!`.** This compiles fine, then crashes
when run:

```rust
fn main() {
    let n: i32 = "ferris".parse().unwrap();
    println!("{n}");
}
```

```
thread 'main' panicked at main.rs:2:35:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
```

`unwrap` is the blunt path: it says "I'm sure this is `Ok`." When you're wrong, the program
stops — and the panic message names exactly what went wrong (`ParseIntError`). That's why
`expect("...")` is often nicer: your message replaces the generic one and explains *which*
assumption broke.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new errors`. **Predict on paper before each run.**

1. **Parse-and-add with `?`.** Write `fn add_strs(a: &str, b: &str) -> Result<i32, std::num::ParseIntError>`
   that parses both strings with `?` and returns their sum wrapped in `Ok`. Call it from
   `main` with two good numbers, then with one bad string, printing each result with `{:?}`.
   **Predict** both outputs before running.

2. **Same job, handled with `match`.** Take *one* `parse()` result and write a `match` that
   prints `"got <n>"` on `Ok` and `"bad input"` on `Err`. Then rewrite it using
   `unwrap_or(-1)` and **predict** what each version prints for `"7"` and for `"seven"`.

3. **Trigger `E0277` on purpose.** Put a `?` directly in `main` (which returns `()`).
   **Predict the error code** before you compile. Then fix it *two* ways: once by moving the
   `?` into a helper function that returns a `Result`, and once by handling the result with a
   `match` instead. Which reads better to you?

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. With `Result` and `?` you can now write functions that fail *honestly* — the failure
is in the type, and the caller can't ignore it.)*

## 6. What surprised you?

A sentence or two: did "an error is just a value in the return type" land differently from
hidden exceptions? Did `?` click as "early-return on `Err`"? Did seeing `unwrap` crash —
and name the reason — change when you'd reach for it? Tell me, and I'll fold it into the
Phase-5 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.9**: §9.1 (`panic!` — unrecoverable
  errors), §9.2 (`Result<T, E>`, matching on it, `unwrap`/`expect`, the `?` operator and
  propagating errors), §9.3 ("to `panic!` or not to `panic!`" — when each is appropriate).
- **CR** — *Comprehensive Rust* (Google), error-handling chapter: the `Result` variants and
  `unwrap`/`expect` framing.
- **BLOG** — not used here; the concepts are sourced from BOOK/CR.
- Every snippet compiled and run, and every error captured live, on **rustc 1.95.0**,
  edition 2024 (`rustc --edition 2024 FILE.rs`). This completes the Phase-5 lessons
  (L18 structs · L19 enums + matching · L20 error handling); the Phase-5 review (quiz +
  cheatsheet) is extended next to cover `Result`/`?`.

---

<!-- lesson-nav -->
[← Lesson 19 — Enums & Matching](19-enums-and-matching.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 21 — Packages, Crates & Modules →](21-packages-crates-modules.md)
