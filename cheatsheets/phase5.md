# Phase 5 Cheatsheet — Custom Types & Matching

Quick reference (pairs with the Phase-5 lessons — structs · enums + matching · error handling). This is where you design your own types: a struct says a value has **all** of its fields; an enum says a value is **one of** its variants; `Option` + `match` make the compiler force you to handle every case; and `Result` + `?` let a function fail honestly. Verified on rustc 1.95.0, edition 2024.

## Structs — group related values
- `struct Person { name: String, age: u32 }` — a template; each instance fills it in. Own your fields (`String`, not `&str` — a borrowed field needs a lifetime, Phase 6).
- Instantiate `Person { name, age }` (**field-init shorthand** when locals share the names); access `p.name`; the **whole instance** must be `mut` to change a field.
- **Struct update:** `User { name: n, ..old }` fills the rest from `old` (moves `old` if a non-`Copy` field comes from it). **Tuple struct:** `struct Color(i32,i32,i32);` → `c.0`. **Unit struct:** `struct Marker;`.

## Methods & `impl`
- `impl Type { fn area(&self) -> u32 {…} }` — the receiver mirrors borrow intent: **`&self`** read · **`&mut self`** change · **`self`** consume (using the value after a `self` method = `E0382`).
- **Associated function** = no receiver, called with `::` (`Rectangle::square(3)`); `Self` is the type's own name; `new` is a *convention*, not a keyword. Multiple `impl` blocks are allowed.

## `#[derive(Debug)]`
- Put `#[derive(Debug)]` above a type to make `{:?}` (and pretty `{:#?}`) work on it. Forget it → **`error[E0277]`** "doesn't implement `Debug`" — the compiler tells you the exact line to add.
- `dbg!(x)` prints to stderr and **returns** its value (use `dbg!(&x)` to keep `x`).

## Enums — a value that is one of a set
- `enum Message { Quit, Move { x: i32, y: i32 }, Write(String), ChangeColor(i32,i32,i32) }` — variants can be empty, tuple-like, or struct-like. **A variant name is a constructor** (`Message::Write("hi".into())`, `Some(5)`).
- Methods via `impl`, body usually `match self`.

## `Option<T>` — Rust's "no null"
- Either `Some(value)` or `None` (both in the prelude). A bare `None` needs a type annotation (`let x: Option<i32> = None;`) → else **`error[E0282]`** "type annotations needed".
- You can't use an `Option<T>` where a `T` is expected (`x + Some(5)` → **`E0277`**) — get the value out first. The point: a missing value can't silently pass as a real one.

## `match` in depth
- **Exhaustive** — handle every case or **`error[E0004]`** "non-exhaustive patterns" (it names the missing one, e.g. `None`). `_` or a binding name is the catch-all (goes **last**).
- **Bind** inner data: `Some(x) => …`, `Message::Move { x, y } => …`. **Multiple patterns:** `1 | 2 | 3 =>`. **Ranges:** `4..=9 =>`. **Guards:** `Some(x) if x < 0 =>`.

## Concise single-pattern forms
- `if let Some(v) = opt { … }` — handle one case (you give up exhaustiveness checking).
- `while let Some(x) = stack.pop() { … }` — loop while the pattern keeps matching.
- `let Some(v) = opt else { return; };` — bind on success, keep the **happy path** flat; the `else` must **diverge** (`return`/`break`/`panic!`).

## Error handling — `Result`, `?`, `panic!` (Lesson 20)
- **Two kinds of failure:** a bug → **`panic!("msg")`** stops the program (unrecoverable). An expected failure → return a **`Result<T, E>`** so the caller decides (recoverable). An error is an ordinary **value**, not a hidden exception.
- **`Result<T, E>`** is an enum (sibling of `Option`): **`Ok(value)`** = success · **`Err(error)`** = failure with a reason. `Ok`/`Err` are in the prelude. A fallible fn says so: `fn f(...) -> Result<i32, ParseIntError>`.
- **Reading a `Result` — four ways:**
  - `match r { Ok(v) => …, Err(e) => … }` — handle both yourself.
  - `r.unwrap_or(default)` — the value, or a fallback on `Err`.
  - `r.unwrap()` / `r.expect("msg")` — the value, or **`panic!` on `Err`** (blunt; "I'm sure it's `Ok`" — crashes if wrong, same trade as `unwrap` on `None`).
  - `r?` — the **`?` operator**: on `Err`, **return that `Err` from this function now**; on `Ok`, unwrap and continue.
- **`?` needs a carrier:** it only works in a function returning `Result` (or `Option`). In a plain `-> i32` fn → **`error[E0277]`** "the `?` operator can only be used in a function that returns `Result` or `Option`" (the compiler points at the offending signature).
- `"x".parse::<i32>().unwrap()` **compiles**, then **panics at runtime** — `called Result::unwrap() on an Err value: ParseIntError { kind: InvalidDigit }`. (Custom error types / `Box<dyn Error>` / `From` come in a later phase.)

— *Sources:* BOOK §5–6 + §9 (error handling) · CR §10/§12/§13 + error-handling ch. Snippets verified on rustc 1.95.0, edition 2024. The billion-dollar-mistake framing for `Option` (Tony Hoare) and the coin-sorting `match` picture carry over from Lessons 11/19; `Result` is taught as `Option`'s sibling (Lesson 20).
