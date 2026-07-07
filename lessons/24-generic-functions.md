# Lesson 24 — Generic functions: the `<T>` placeholder

*(Phase 6 — Generics, the opener. You've already used `Vec<T>` (Lesson 14) and
`Option<T>` (Lesson 19b) — that `<T>` is the thing this lesson finally explains.
This is the tool that ends copy-paste-and-rename.)*

## 1. Why it exists

Say you want the first item of a list of numbers. You write a function. Then you
want the first item of a list of characters — same logic, every line identical,
except the word `i32` becomes `char`. So you copy the whole function and rename
it. Now you have two functions that drift apart the moment one needs a fix.

That duplication is the problem generics solve. A **generic** lets you leave a
*hole* where a concrete type would go — a placeholder named `T` — and fill that
hole in later, once per call, with whatever real type you actually pass. One
function, every type.

You've been using this since Lesson 14 without naming it: `Vec<i32>` and
`Vec<String>` are one `Vec<T>` definition with the hole filled two ways; same for
`Option<T>`. Now you get to do it in your own code.

## 2. The idea

A **type parameter** is a name — by convention a single capital letter, usually
`T` (for "Type") — that stands in for a real type you haven't chosen yet. You
**declare** it in angle brackets right after the function name, then **use** it as
if it were a type:

```
fn first<T>(list: &[T]) -> &T { ... }
// `<T>` after the name DECLARES the placeholder.
// After that, T is USED like any type — here in `&[T]` and the return `&T`.
```

When you *call* `first(&numbers)` with numbers, the compiler reads `T = i32`;
call it with chars and `T = char`. You wrote it once; it works for both.
(Generic *structs, enums and methods* are the next lesson's subject — same idea,
three more places to put the `<T>`.)

**One rule the body must obey:** the code inside a generic function has to be
valid for *every* possible `T`. You don't know what `T` is, so you can only do to
it things that work on *all* types — move it, store it, hand back a reference to
it. You can't, say, compare two `T`s with `>`, because not every type can be
ordered. That rule has teeth, and part 4 walks you into them.

## 3. Tiny examples to read

**First, the duplication.** Two functions, identical except for the type:

```rust
fn first_i32(list: &[i32]) -> &i32 {
    &list[0]
}

fn first_char(list: &[char]) -> &char {
    &list[0]
}

fn main() {
    let numbers = vec![34, 50, 25];
    let chars = vec!['y', 'm', 'a'];
    println!("{} {}", first_i32(&numbers), first_char(&chars));
}
```

```
34 y
```

The two bodies are byte-for-byte the same. That's the signal to reach for a
generic.

**Now collapse them into one.** The body only *stores and returns a reference* —
things every type supports — so it compiles with no strings attached:

```rust
fn first<T>(list: &[T]) -> &T {
    &list[0]
}

fn main() {
    let numbers = vec![34, 50, 25];
    let chars = vec!['y', 'm', 'a'];
    println!("{} {}", first(&numbers), first(&chars));
}
```

```
34 y
```

One function, both lists. The compiler filled the hole twice for you.

## 4. Common pitfalls / real compiler errors — the wall this lesson ends on

Now try the *largest* item instead of the first. The body needs `>` — and not
every type can be ordered:

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

**Before you scroll — will this compile?**

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
```

Read the message slowly. A bare `T` can only do what *all* types can do, and `>`
isn't one of those things. The compiler's suggested fix — restricting `T` with a
**trait bound** — is the entire subject of the next-but-one lesson. Don't apply
it yet; just notice *what* it's asking you to promise about `T`. This wall is
exactly where Lesson 25 picks up.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the two matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new generics` works too.)* **Predict on paper
before each run.** You write every line here — I won't.

1. **Make a function generic.** Write the non-generic `first_i32` above, get it
   working, then convert it to `fn first<T>(...)`. **Predict:** will it compile
   with no bound? Call it on numbers *and* chars and check.
2. **Walk into `E0369` on purpose.** Write `largest<T>` using `>` on a bare `T`.
   **Predict the error code** before compiling, and read the compiler's
   suggested promise carefully — Lesson 25 is about exactly that.

*(The predictions are your answer key. Next: generic structs, enums and methods —
then trait bounds, which finally let a generic `T` DO something.)*

## 6. What surprised you?

A sentence or two: did it click that `Vec<T>` and `Option<T>` were generics you'd
been using all along? Did "the body must be valid for *every* `T`" explain the
wall? Tell me, and I'll pitch Lesson 24b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.10.1 "Generic Data Types"**:
  the deduplication arc and the phrase "abstract stand-ins for concrete types."
  The `largest` example is adapted from BOOK Listings 10-3 through 10-5.
- **CR** — *Comprehensive Rust* (Google): the declare-vs-use framing. Its
  cross-language comparison is dropped per the no-analogy rule.
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 23 — The `use` Keyword](23-the-use-keyword.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 24b — Generic structs, enums & methods →](24b-generic-types.md)
