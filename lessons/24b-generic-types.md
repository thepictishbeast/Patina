# Lesson 24b — Generic structs, enums & methods

*(Phase 6, part 2. Lesson 24 put the `<T>` hole in a function. The same hole fits
in three more places — and one of them unmasks a type you've used since Phase 5.)*

## 1. Why it exists

Data types duplicate exactly the way functions do: a `PointOfInts` and a
`PointOfFloats` differ by one word. The `<T>` placeholder works on type
*definitions* too — structs, enums, and their `impl` blocks — so one definition
serves every element type, exactly like the standard library's own containers.

## 2. The idea

`T` can live in three more places, and the syntax is the same idea each time:

- **Struct** — `struct Point<T> { x: T, y: T }` — declare `<T>` after the struct
  name; now `x` and `y` are both whatever type you fill in.
- **Enum** — `enum MyOption<T> { Some(T), None }` — exactly how the real
  `Option<T>` is built.
- **Method** — `impl<T> Point<T> { … }` — declare `<T>` **after `impl`** so the
  block knows `T` is a placeholder, *then* name the type `Point<T>`. The `<T>`
  appears twice on that line — once to declare, once as part of the type's name.

And the cost at runtime? **Nothing** — the compiler stamps out a specialized copy
per concrete type you actually use (the Book calls it *monomorphization*; the
trade-offs are in Ch. 10.1, worth a read once this lesson settles).

## 3. Tiny examples to read

**A generic struct, with a generic method.** Predict both lines:

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

You never wrote `PointI32` and `PointF64` — the compiler made them from one
definition, the moment you used `5` and `1.5`.

**A generic enum — this is literally how `Option<T>` is built:**

```rust
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    let a: MyOption<i32> = MyOption::Some(5);
    let c: MyOption<i32> = MyOption::None;
    if let MyOption::Some(i) = a {
        println!("a holds {i}");
    }
    if let MyOption::None = c {
        println!("c holds nothing");
    }
}
```

```
a holds 5
c holds nothing
```

The real `Option<T>` you've used since Lesson 19b is doing exactly this under the
hood.

## 4. Common pitfalls / real compiler errors — forgetting `<T>` after `impl`

The type parameter must be *declared* on the impl block, not just used. Writing
`impl Point<T>` (no `<T>` after `impl`) means `T` was never introduced:

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
```

**Before you scroll — what does the compiler say?**

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
```

The compiler prints the fix verbatim: declare the placeholder right after `impl`,
and the `T`s inside the block have a meaning.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new generic_types` works too.)* **Predict on paper before
each run.** You write every line here — I won't.

1. **A generic struct of your own.** Define `struct Pair<T> { first: T,
   second: T }`. Build one `Pair` of integers and one of chars, print all four
   fields. **Predict** the output. (You never declared `PairOfChar` — convince
   yourself the compiler built it.)
2. **Add a method.** Give `Pair<T>` a method `first(&self) -> &T`. **Predict the
   error code** you'll get if you write `impl Pair<T>` instead of
   `impl<T> Pair<T>` — deliberately write the wrong one, confirm the code, fix it.
3. **Rebuild `Option` by hand.** Define `MyOption<T>`, build a `Some` and a
   `None`, and `if let` both. **Predict** each printout.

*(The predictions are your answer key. Next: trait bounds — the promise that lets
a generic `T` finally be compared, printed, and added.)*

## 6. What surprised you?

A sentence or two: did unmasking `Option<T>` as an ordinary generic enum change
how magical it feels? Did the double `<T>` on the impl line make sense once the
compiler explained it? Tell me, and I'll pitch Lesson 25 to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.10.1**: generics in structs
  (`Point<T>`), enums (`Option<T>`/`Result<T, E>`), methods (`impl<T> Point<T>`),
  and monomorphization. Adapted from Listings 10-6 through 10-10.
- **CR** — *Comprehensive Rust* (Google): the placements, and the
  binary-size/compile-time caveat behind monomorphization (see Book Ch. 10.1).
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 24 — Generic functions](24-generic-functions.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 25 — Traits: declare and implement →](25-traits-declare-implement.md)
