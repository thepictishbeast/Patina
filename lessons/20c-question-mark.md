# Lesson 20c — The `?` operator: pass the problem up

*(Phase 5, the finale. You can handle an error (`match`), crash on it (`unwrap`) —
now the third and most Rust-flavoured option: hand it to whoever called you, in
one character.)*

## 1. Why it exists

Real programs call functions that call functions, and most layers can't *do*
anything useful about an error — the sensible move is to pass it up to someone
who can. Doing that with a `match` at every call site buries the logic in
ceremony. The **`?` operator** is that entire `match`, in one character.

## 2. The idea

Inside a function that *itself* returns a `Result`, writing `value?` means:

> "If `value` is `Err`, **return that `Err` from this function right now**. If
> it's `Ok`, unwrap it and carry on."

Three consequences worth knowing:

- **`?` only works where there's somewhere to send the error** — the function's
  return type must be a `Result` (or an `Option`). Use it in a plain function and
  the compiler stops you (part 4).
- **The two must match in kind.** A `?` on an `Option` inside a
  `Result`-returning function (or vice versa) is refused — there'd be no way to
  turn a `None` into your `Err` type. (One of the exercises hands you exactly
  this mixed-kind wall.)
- **`?` converts error types on the way out.** If your function returns
  `Result<_, MyError>` and the inner call fails with a different error type, `?`
  will convert it — *provided* you've told Rust how, by implementing the `From`
  trait for your error type. That's what makes custom error types practical, and
  it's the third exercise below.

## 3. A tiny example to read

The function returns a `Result`, so `?` can early-return the error:

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

Notice `double` never writes a `match` — `?` did the "if error, bail out" for it,
and the success path reads as if nothing could go wrong.

## 4. Common pitfalls / real compiler errors

**Using `?` in a function that can't carry the error — `E0277`:**

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

(That `FromResidual` clause in the first line — ignore it for now; Book §9.2 has
the story.) The fix is to make the function return a `Result` so the `?` has
somewhere to send the error, or to handle it right here with a `match`. The error
message even points at which function is in the way.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then work through the three
matching exercises via the **Practice this lesson** links at the bottom. *(On
your own machine, a playground or `cargo new propagate` works too.)* **Predict on
paper before each run.**

1. **Parse-and-add with `?`.** Write
   `fn add_strs(a: &str, b: &str) -> Result<i32, std::num::ParseIntError>` that
   parses both strings with `?` and returns their sum wrapped in `Ok`. Call it
   with two good numbers, then with one bad string, printing each result with
   `{:?}`. **Predict** both outputs.
2. **Trigger `E0277` on purpose.** Put a `?` directly in a function returning
   `i32`. **Predict the error code**, then fix it two ways: return a `Result`, or
   handle it with a `match`. Which reads better to you?
3. **Mix the kinds.** Inside a `Result`-returning function, try `?` on an
   `Option`. **Predict**: same error code or different?

*(You write every line here — I won't. The predictions are your answer key. With
`Result` and `?` you can now write functions that fail *honestly* — the failure
is in the type, and the caller can't ignore it. That completes Phase 5's toolkit;
next, Phase 6: organizing code.)*

## 6. What surprised you?

A sentence or two: did `?` click as "early-return on `Err`"? Does "pass it up"
feel more natural than try/catch-style jumps? Tell me, and I'll fold it into the
Phase-5 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §9.2 — the `?` operator,
  propagating errors, and the `From` conversion that makes custom error types
  work with `?`.
- **CR** — *Comprehensive Rust* (Google), error-handling chapter.
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024. This completes the Phase-5 lessons (structs ·
  enums · Option · match · Result · `?`).

---

<!-- lesson-nav -->
[← Lesson 20b — panic!, unwrap and expect](20b-panic-unwrap-expect.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 21 — Packages, Crates & Modules →](21-packages-crates-modules.md)
