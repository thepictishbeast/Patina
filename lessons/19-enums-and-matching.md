# Lesson 19 — Enums & Matching

*(Phase 5 — Custom types & matching, part 2. Structs (Lesson 18) group values that go
*together*. Enums model a value that is *one of* several shapes — and `match` makes you
handle every shape. This is the other half of designing your own types.)*

## 1. Why it exists

A struct says "a thing has all of these." An **enum** says "a thing is **one of** these."
A message is *either* a quit *or* a move *or* some text; a lookup *either* found a value
*or* didn't. Modelling that with an enum, plus `match` to handle each case, lets the
compiler guarantee you've covered every possibility — you literally can't forget one.

Rust also has **no null**. Instead, "a value might be missing" is its own type,
`Option<T>` (`Some(v)` or `None`), so the *type* tells you to handle the missing case and
the compiler enforces it.

> **How the sources frame it:** the **BOOK** is the backbone — enums-vs-structs, the
> `Option` "no null" pitch (Tony Hoare's "billion-dollar mistake"), binding data out of a
> `match`, and the exhaustiveness error; **CR** owns the pattern vocabulary (`|`, ranges,
> guards) and `while let`. The matrix supplied the one piece no source had: a method on an
> enum.

## 2. The idea

**Enums and their data.** Variants can be empty, or carry data — and the shapes can mix:

```
enum Message {
    Quit,                        // no data
    Move { x: i32, y: i32 },     // struct-like
    Write(String),               // tuple-like
    ChangeColor(i32, i32, i32),  // tuple-like
}
```

A variant name is really a **constructor**: `Message::Write(String::from("hi"))` makes one.
You attach methods with `impl`, and the body usually `match`es on `self`.

**`Option<T>` — Rust's "no null."** Instead of a value that might secretly be null, you get
`Option<T>`: either `Some(value)` or `None`. A bare `None` needs a type annotation (the
compiler can't guess what the `Some` would hold). And you **cannot use an `Option<T>` where
a `T` is expected** — you have to get the value out first (you'll see the error in part 4).
As CR puts it: you can choose to panic on `None`, but you can't *accidentally forget* to
check for it.

**`match` in depth** (this picks up the deep patterns Lesson 11 deferred):
- **Bind** the inner data: `Some(x) => …`, `Message::Move { x, y } => …`.
- **Multiple patterns** with `|`: `1 | 2 | 3 => …`. **Ranges** with `..=`: `4..=9 => …`.
- **Guards** — an extra condition: `Some(x) if x < 0 => …`.
- A catch-all that **binds** (`other => …`) vs `_` that **ignores**. The catch-all goes
  **last**, and `match` must be **exhaustive** — miss a case and it won't compile.

**Concise single-pattern forms:**
- `if let Some(v) = opt { … }` — handle just one case (you trade away exhaustiveness checking).
- `while let Some(x) = stack.pop() { … }` — loop while the pattern keeps matching.
- `let Some(v) = opt else { return; };` — bind on success and keep the **happy path** flat;
  the `else` must **diverge** (`return`/`break`/`panic!`).

## 3. Tiny examples to read

**An enum with a method (`match self`).** Predict the four lines:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn describe(&self) -> String {
        match self {
            Message::Quit => String::from("quit"),
            Message::Move { x, y } => format!("move to ({x}, {y})"),
            Message::Write(text) => format!("write: {text}"),
            Message::ChangeColor(r, g, b) => format!("color ({r}, {g}, {b})"),
        }
    }
}
fn main() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("hi")),
        Message::ChangeColor(255, 0, 0),
    ];
    for m in &msgs {
        println!("{}", m.describe());
    }
}
```

```
quit
move to (1, 2)
write: hi
color (255, 0, 0)
```

**The full `match` vocabulary on an `Option` (you type this one).** Predict each line:

```rust
fn describe(n: Option<i32>) -> String {
    match n {
        Some(0) => String::from("zero"),
        Some(x) if x < 0 => format!("negative: {x}"),   // guard
        Some(1 | 2 | 3) => String::from("small"),         // multiple patterns
        Some(4..=9) => String::from("medium"),            // range
        Some(x) => format!("big: {x}"),                   // bind the rest
        None => String::from("nothing"),
    }
}
fn main() {
    for n in [Some(0), Some(-5), Some(2), Some(7), Some(100), None] {
        println!("{}", describe(n));
    }
}
```

```
zero
negative: -5
small
medium
big: 100
nothing
```

**The concise forms:**

```rust
fn main() {
    let config: Option<i32> = Some(42);
    if let Some(v) = config { println!("configured: {v}"); }

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() { println!("popped {top}"); }

    let maybe: Option<i32> = Some(10);
    let Some(n) = maybe else { return; };
    println!("got {n}");
}
```

```
configured: 42
popped 3
popped 2
popped 1
got 10
```

*(That `Option` match was your write-rep; part 5 is the rest.)*

## 4. Common pitfalls / real compiler errors

**Forgetting the `None` arm — `E0004`.** A `match` on an `Option` must handle both cases:

```rust
fn plus_one(n: Option<i32>) -> i32 {
    match n {
        Some(x) => x + 1,
    }
}
```

```
error[E0004]: non-exhaustive patterns: `None` not covered
 --> main.rs:2:11
  |
2 |     match n {
  |           ^ pattern `None` not covered
  …
  = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a
      wildcard pattern or an explicit pattern as shown
```

The compiler even names the case you missed (`None`). That's the whole point of `Option`:
the "what if it's missing?" question can't be skipped.

**Using an `Option<T>` as if it were a `T` — `E0277`.** You can't add a `Some` to a number:

```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
}
```

```
error[E0277]: cannot add `Option<i8>` to `i8`
 --> main.rs:4:17
  |
4 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
```

`y` *might* be `None`, so Rust won't let you treat it as a plain number — you have to get
the value out first (with `match`, `if let`, or later `unwrap`/`unwrap_or`). This is the
safety the type buys you: a missing value can never silently sneak through as a real one.

> Null has been called "the billion-dollar mistake" by the person who invented it (Tony
> Hoare). `Option<T>` is Rust's answer: nothing is null, so a value you didn't wrap in
> `Option` is *guaranteed* to be there.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new enums`. **Predict on paper before each run.**

1. **An enum with a method.** Make an enum `Shape` with at least two data-carrying variants
   (e.g. `Circle(f64)`, `Rectangle { w: f64, h: f64 }`). Add an `impl` with an `area(&self)`
   method that `match`es on `self`. Build a couple and print their areas. **Predict** the
   output.

2. **Match an `Option` exhaustively.** Write a function taking `Option<i32>` that returns a
   `String`: handle `Some` (bind the value) and `None`. **Predict** what each returns. Then
   *delete* the `None` arm — **predict** the error code, then put it back.

3. **`if let` and `let…else`.** Take an `Option`, print its value with `if let` if it's
   `Some`. Then rewrite using `let…else` (binding on success, `return`ing in the `else`).
   **Predict** both behaviours.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. Structs + enums + `Option` + `match` are how you make a type that can only ever hold
*valid* states. Next comes the Phase-5 review.)*

## 6. What surprised you?

A sentence or two: did "a variant name is a constructor" demystify `Some(5)`? Did the
compiler naming the missing `None` arm feel helpful or strict? Tell me, and I'll shape the
Phase-5 review around it.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §6.1 (defining enums, data-carrying variants,
  `Option` and the "billion-dollar mistake"), §6.2 (`match`: binding inner data, the
  `None`-not-covered `E0004`, catch-all vs `_`), §6.3 (`if let`, `let…else`).
- **CR** — *Comprehensive Rust* (Google), §10.3 (the three variant kinds), §12.2 (the
  pattern vocabulary: `|`, `..=`, guards), §12.5 (`while let`). The enum-with-a-method
  (`match self`) example was synthesis-authored to fill a catalog gap (no source had one);
  it was compiled like every other snippet.
- **BLOG** — only an incidental gloss; the concepts are sourced from BOOK/CR.
- Compiler output captured live on **rustc 1.95.0** (edition 2024). Error handling
  (`Result`, `?`, `panic!`, `unwrap`/`expect`) is the next slice of Phase 5; after this comes
  the Phase-5 review.

---

<!-- lesson-nav -->
[← Lesson 18 — Structs (your own types)](18-structs.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 20 — Error Handling: `Result`, `?`, `panic!` →](20-error-handling.md)
