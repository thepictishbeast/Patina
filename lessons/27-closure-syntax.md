# Lesson 27 — Closures: unnamed inline functions

*(Phase 7 — Functional features & smart pointers, part 1. Until now a piece of
behaviour you wanted to reuse had to be a named `fn` (Lesson 7). A **closure** is
a lighter thing: an unnamed function you write inline, save in a variable, or
hand to another function.)*

## 1. Why it exists

Sometimes you need a small piece of behaviour *right where you are* — a rule for
sorting, a fallback to compute on failure. Defining a whole separate named `fn`
for it is heavy: you have to name it, park it somewhere, and spell out its types.
A **closure** is an **anonymous function** you write inline — and (next lesson's
subject) it can even *capture* the variables sitting around it. This lesson is
the syntax and one sharp edge of its type inference.

## 2. The idea

**The syntax.** A closure is a pair of bars holding its parameters, then its
body:

```
|x| x + 1            // takes x, returns x + 1
|a, b| a + b         // takes two, returns their sum
|| println!("hi")    // takes nothing (empty bars)
```

Save it in a variable and call it with parentheses, exactly like a function:

```
let add_one = |x| x + 1;
add_one(41);         // 42
```

You usually don't write the parameter or return types — the compiler reads them
off the body and **the first use**. (A named `fn` always makes you spell them
out; a closure lets you skip them. Under the hood each closure has its own
unique type described by a small trait family — Book Ch. 13.1 when you're
curious; you won't need the names yet.)

## 3. Tiny examples to read

**Save it, call it:**

```rust
fn main() {
    let add_one = |x| x + 1;
    println!("{}", add_one(41));
}
```

```
42
```

**Hand one to a method.** Many standard-library methods take a closure. Here
`sort_by_key` calls `|w| w.len()` once per element to get each word's sort key —
no loop written by you:

```rust
fn main() {
    let mut words = vec!["hello", "hi", "hey"];
    words.sort_by_key(|w| w.len());
    println!("{words:?}");
}
```

```
["hi", "hey", "hello"]
```

## 4. Common pitfalls / real compiler errors — inference locks on first use

Skipping the types is convenient, but the compiler pins them down at the
**first call** — and after that the closure is locked to those types:

```rust
fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
}
```

**Before you scroll — does this compile?**

```
error[E0308]: mismatched types
 --> main.rs:4:29
  |
4 |     let n = example_closure(5);
  |             --------------- ^ expected `String`, found integer
  |             |
  |             arguments to this function are incorrect
  |
note: expected because the closure was earlier called with an argument of type `String`
```

Read the `note:` — it's unusually direct: the *earlier call* fixed the type.
A closure isn't generic over its uses; inference fills the blank once. The
matching exercise below hands you this wall — **predict the code** before you
run.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new closures` works too.)* **Predict on paper
before each run.**

1. **Write and call.** A closure that doubles its argument; call it twice with
   different numbers. **Predict** both outputs.
2. **Hand one to `sort_by_key`.** Sort a vector of numbers by their *distance
   from 50* (`|n| (n - 50).abs()` — or pick any key you like). **Predict** the
   order before running.
3. **Lock it on purpose.** Write `|x| x`, call it with a `String`, then with a
   number. **Predict the error code** and *which call* the compiler blames.

*(You write every line here — I won't. The predictions are your answer key.
Next: the part that makes a closure a closure — capturing the variables around
it, and the `move` keyword.)*

## 6. What surprised you?

A sentence or two: did "an unnamed function you write inline" land? Did the
first-use type lock surprise you — or does it follow from Lesson 1's "Rust pins
one fixed kind"? Tell me, and I'll pitch Lesson 27b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.13.1**: the definition
  ("anonymous functions you can save in a variable or pass as arguments") and
  the inference-locks-on-first-use example (Listing 13-3, adapted).
- **CR** — *Comprehensive Rust* (Google), closures section.
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 26c — Lifetimes in structs](26c-lifetimes-in-structs.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 27b — Closure capture & move →](27b-closure-capture.md)
