# Lesson 24 — Generics: `<T>` type parameters

*(Phase 6 — Generics, the opener. You've already used `Vec<T>` (Lesson 14) and
`Option<T>` (Lesson 19) — that `<T>` is the thing this lesson finally explains.
Generics let you write a function, struct, enum, or method **once** and have it work
for many types. This is the tool that ends copy-paste-and-rename.)*

## 1. Why it exists

Say you want the largest item in a list of numbers. You write a function. Then you want
the largest item in a list of characters — same logic, every line identical, except the
word `i32` becomes `char`. So you copy the whole function and rename it. Now you have two
functions that drift apart the moment one needs a fix.

That duplication is the problem generics solve. A **generic** lets you leave a *hole* where
a concrete type would go — a placeholder named `T` — and fill that hole in later, once per
call, with whatever real type you actually pass. One function, every type.

You've been using this since Lesson 14 without naming it. `Vec<T>` is "a vector of *some*
type `T`": `Vec<i32>` fills the hole with `i32`, `Vec<String>` fills it with `String`. Same
for `Option<T>` from Lesson 19 — `Option<i32>`, `Option<char>`, all one definition. The
standard library wrote those generically so you wouldn't need a separate `VecOfI32` and
`VecOfChar`. Now you get to do the same in your own code.

> **How the sources frame it:** the **BOOK** Ch.10.1 is the backbone — it earns the feature
> the honest way: deduplicate a `largest` function by hand, watch two near-identical copies
> appear, then collapse them into one generic `largest<T>`, and walk the same `<T>` through
> all four places it can live (function, struct, enum, method). **CR** reinforces the
> placements and adds the one caveat BOOK skips (see §2's cost note). No metaphor beyond
> BOOK's own plain phrasing: generics are **"abstract stand-ins for concrete types."**

## 2. The idea

A **type parameter** is a name — by convention a single capital letter, usually `T` (for
"Type") — that stands in for a real type you haven't chosen yet. You **declare** it in angle
brackets, then **use** it as if it were a type:

```rust
fn largest<T>(list: &[T]) -> &T { ... }
// `<T>` right after the name DECLARES the generic type, named T.
// After that, T is USED like any type — here in `&[T]` and the return `&T`.
```

The `<T>` right after the name says "this thing is generic over a type I'm calling `T`."
After that, `T` behaves like any other type name inside the definition. When you *call*
`largest(&numbers)` with a `Vec<i32>`, the compiler reads `T = i32`; call it with chars and
`T = char`. You wrote it once; it works for both.

`T` can live in **four places**, and the syntax is the same idea each time:

- **Function** — `fn largest<T>(list: &[T]) -> &T` — declare `<T>` after the function name.
- **Struct** — `struct Point<T> { x: T, y: T }` — declare `<T>` after the struct name; now
  `x` and `y` are both whatever type you fill in.
- **Enum** — `enum MyOption<T> { Some(T), None }` — exactly how the real `Option<T>` is built.
- **Method** — `impl<T> Point<T> { ... }` — declare `<T>` **after `impl`** so the impl block
  knows `T` is a placeholder, *then* name the type `Point<T>`.

**One rule the body must obey:** the code inside a generic definition has to be valid for
*every* possible `T`. You don't know what `T` is, so you can only do to it things that work
on *all* types. You can move it, store it, hand back a reference to it — but you can't, say,
compare two `T`s with `>`, because not every type can be ordered. (Restricting `T` to "types
that can be ordered" is the job of **trait bounds**, the next lesson. Part 4 shows the exact
error that pushes you there.)

**What it costs at runtime: nothing.** When you compile, Rust performs **monomorphization** —
it looks at every concrete type you actually used and stamps out a separate, specialized copy
of the code for each one. `largest<T>` called on i32s and chars becomes, in the finished
binary, two ordinary functions with no `T` left in them. So a generic is *exactly* as fast as
the hand-written copies would have been — the abstraction is gone by the time the program
runs. The honest trade-off (which the BOOK glosses over, and CR names): those duplicated
copies make the **binary larger** and the **compile slower**. For everyday programs you won't
notice; for tiny targets like embedded chips or WebAssembly, it's worth knowing.

## 3. Tiny examples to read

**First, the duplication.** Two functions, identical except for the type — this is the pain:

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest_i32(&numbers));
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest_char(&chars));
}
```

```
largest number: 100
largest char: y
```

The two bodies are byte-for-byte the same. The only difference is `i32` vs `char`. That is
the signal to reach for a generic.

**Now collapse them into one.** Same logic, the concrete type replaced by `T`:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest(&numbers));
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest(&chars));
}
```

```
largest number: 100
largest char: y
```

One function, both lists, same output. (The `: PartialOrd` after `T` is a trait bound — it's
what lets the body use `>`. Ignore the *how* for now; the next lesson is entirely about it.
Part 4 shows what happens if you leave it off.)

**A generic struct, with a generic method.** `Point<T>` holds two values of the same type;
the `impl<T>` block gives it a method:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 4.0 };
    println!("int_point.x = {}", int_point.x());
    println!("float_point: x = {}, y = {}", float_point.x, float_point.y);
}
```

```
int_point.x = 5
float_point: x = 1.5, y = 4
```

You never wrote `PointI32` and `PointF64`. The compiler made them for you from one
definition, the moment you used `5` and `1.5`. Note the `<T>` appears **twice** in the impl
line — `impl<T> Point<T>` — once to declare the placeholder, once as part of the type's name.

**A generic enum — this is literally how `Option<T>` is built.** Defining it by hand:

```rust
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    let a: MyOption<i32> = MyOption::Some(5);
    let b: MyOption<f64> = MyOption::Some(2.5);
    let c: MyOption<i32> = MyOption::None;
    if let MyOption::Some(i) = a {
        println!("a holds {i}");
    }
    if let MyOption::Some(f) = b {
        println!("b holds {f}");
    }
    if let MyOption::None = c {
        println!("c holds nothing");
    }
}
```

```
a holds 5
b holds 2.5
c holds nothing
```

`MyOption<i32>` and `MyOption<f64>` are two concrete enums built from one generic definition —
exactly what the real `Option<T>` you've used all along is doing under the hood.

## 4. Common pitfalls / real compiler errors

**Forgetting `<T>` after `impl` — `E0425`.** The type parameter must be *declared* on the
impl block, not just used. Writing `impl Point<T>` (no `<T>` after `impl`) means `T` was never
introduced, so the compiler has no idea what it is:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("{}", p.x());
}
```

```
error[E0425]: cannot find type `T` in this scope
 --> main.rs:6:12
  |
6 | impl Point<T> {
  |            ^ not found in this scope
  |
help: you might be missing a type parameter
  |
6 | impl<T> Point<T> {
  |     +++

error[E0425]: cannot find type `T` in this scope
 --> main.rs:7:21
  |
7 |     fn x(&self) -> &T {
  |                     ^ not found in this scope

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.
```

The compiler prints the fix verbatim: `impl<T> Point<T>`. Declare the placeholder right after
`impl`, then the `T`s inside the block have a meaning.

**Using `>` (or `+`, `==`, …) on a bare `T` — `E0369`.** This is the wall the matrix warned
about. The generic body must work for *every* `T`, and not every type can be compared with
`>`, so the compiler refuses until you promise `T` is orderable:

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("{}", largest(&numbers));
}
```

```
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> main.rs:4:17
  |
4 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T` with trait `PartialOrd`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0369`.
```

The fix — `T: PartialOrd`, the thing you saw bolted on in part 3 — is a **trait bound**, and
it's the whole subject of the next lesson. For now, just read the message: a bare `T` can only
do what *all* types can do, and `>` isn't one of those things. The compiler tells you exactly
what promise to add.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new generics`. **Predict on paper before each run.** You
write every line here — I won't.

1. **Make a function generic.** Start by writing a *non-generic* `fn first_i32(list: &[i32])
   -> &i32` that returns a reference to the first element (`&list[0]`). Get it working on a
   number list. Then convert it to `fn first<T>(...)` so it works on any slice — note that the
   body never compares anything, so you should *not* need a trait bound. **Predict:** will this
   one compile without `PartialOrd`? Call it on a list of numbers *and* a list of chars and
   check.

2. **A generic struct of your own.** Define `struct Pair<T> { first: T, second: T }`. In
   `main`, build one `Pair` of integers and one `Pair` of chars, then print all four fields.
   **Predict** the output before running. (You never declared `PairOfChar` — convince yourself
   the compiler built it for you.)

3. **Add a method.** Give `Pair<T>` a method `first(&self) -> &T` that returns a reference to
   `first`. Remember the impl line needs the placeholder declared. **Predict the error code**
   you'll get if you write `impl Pair<T>` instead of `impl<T> Pair<T>` — then deliberately
   write the wrong one, confirm the code, and fix it.

4. **Walk into `E0369` on purpose.** Take a generic function and try to use `>` (or `==`) on a
   bare `T` with no bound. **Predict the error code** before compiling. Read the compiler's
   suggested fix carefully — you'll meet that fix in full next lesson, so don't apply it yet;
   just notice *what* it's asking you to promise about `T`.

*(The predictions are your answer key; the code is yours. After this, the next lesson —
trait bounds — finally lets a generic `T` *do* something, like be compared or printed.)*

## 6. What surprised you?

A sentence or two: did it click that `Vec<T>` and `Option<T>` were generics you'd been using
all along? Did "the body must be valid for *every* `T`" explain why the bare `largest<T>`
wouldn't compile? Did "monomorphization means generics cost nothing at runtime" land — or did
the binary-size caveat surprise you? Tell me, and I'll fold it into the Phase-6 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.10.1 "Generic Data Types"**: the
  deduplication arc (the two `largest` copies → one generic `largest<T>`), generics in structs
  (`Point<T>`), enums (`Option<T>`/`Result<T, E>`), and methods (`impl<T> Point<T>`), plus
  monomorphization and the phrase "abstract stand-ins for concrete types." The `largest`
  example is adapted from BOOK Listings 10-3 through 10-8.
- **CR** — *Comprehensive Rust* (Google): reinforces the four placements, and supplies the cost
  caveat BOOK omits — monomorphization trades **binary size and compile time** for its runtime
  speed (matters for embedded / WebAssembly targets). Its "like C++ templates" framing is
  dropped per the no-other-languages rule; the substance is kept.
- **BLOG** — not used here; it has no generics material.
- Every snippet compiled and run, and every error captured live, on **rustc 1.95.0**, edition
  2024 (`rustc --edition 2024 FILE.rs`). This opens Phase 6; the next lesson adds **trait
  bounds** (`T: PartialOrd`), which lets a generic `T` finally be compared, printed, and added.

---

<!-- lesson-nav -->
[← Lesson 23 — The `use` Keyword](23-the-use-keyword.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 25 — Traits: Defining Shared Behavior →](25-traits.md)
