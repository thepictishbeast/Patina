# Lesson 26b — Lifetime elision: the annotations you never write

*(Phase 6, part 6. You've been returning references since Lesson 17 — and never
typed a `'a` once. This lesson explains who's been writing them for you, and the
one thing no annotation can ever fix.)*

## 1. Why it exists

If every reference has a lifetime, why did `longest` need an annotation while
everything you wrote before Lesson 26 didn't? Because the compiler applies a few
**elision rules** that fill lifetimes in automatically — and they cover the
overwhelmingly common cases. Knowing the *one* rule that does most of the work
tells you in advance which functions will ask you for an annotation and which
won't.

## 2. The idea

The workhorse is **rule 2**:

> **If a function has exactly one input reference, its lifetime is given to all
> output references.**

One reference in, one out — there's only one thing the output could borrow from,
so the compiler doesn't need you to say it. That's why `longest` (with *two*
inputs) demanded an annotation and nothing before it ever did. (There are two
more rules — every parameter gets its own lifetime, and `&self` donates its
lifetime in methods — the full set is the Book, Ch. 10.3.)

## 3. A tiny example to read — zero annotations, on purpose

One `&str` in, one out. Rule 2 fills the lifetime in silently. (This version
uses only tools you have: `find` gives back an `Option` — Lesson 19b — and the
`match` pulls the index out.)

```rust
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],   // slice up to the first space
        None => s,            // no space — the whole string is the word
    }
}

fn main() {
    let sentence = String::from("hello world");
    println!("first word: {}", first_word(&sentence));
}
```

```
first word: hello
```

Same shape of problem as `longest` — a function returning a borrow — but you
write zero lifetimes. This is the common case; two-input functions are the
exception.

## 4. Common pitfalls / real compiler errors — the thing no annotation can fix

A returned reference must borrow **from an input**. Borrow from a *local* and no
lifetime can save you — the data genuinely dies at the closing brace:

```rust
fn make_owned(s: &str) -> &str {
    let owned = String::from(s);
    owned.as_str()
}
```

```
error[E0515]: cannot return value referencing local variable `owned`
 --> main.rs:3:5
  |
3 |     owned.as_str()
  |     -----^^^^^^^^^
  |     |
  |     returns a value referencing data owned by the current function
  |     `owned` is borrowed here
```

The honest limit in one sentence: you can return a reference that borrows from
an input (like `longest` returning `x` or `y`), never one that borrows from a
local — to hand back freshly-made data, return the owned `String` itself. (The
full walkthrough is the Book, Ch. 10.3.)

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new elision` works too.)* **Predict on paper before each
run.**

1. **Find the elided lifetime.** Write the simplest function that takes a single
   `&str` and returns it unchanged, with **no** annotation anywhere. **Predict
   whether it compiles** and *which rule* makes the annotation unnecessary.
2. **Try to return a reference to a local — predict `E0515`.** Write
   `fn loudest(s: &str) -> &str` that builds an uppercase `String` inside (look
   up `to_uppercase`) and tries to return a `&str` slice of it. **Predict the
   error code**, then find the smallest change that makes it compile. (Hint:
   stop returning a *borrow* of the local.)

*(You write every line here — I won't. The predictions are your answer key.
Next: the lifetime that lives on a STRUCT — and the close of Lesson 18's
`&str`-field mystery.)*

## 6. What surprised you?

A sentence or two: did elision explain why you'd never typed a lifetime before
Lesson 26, despite returning references since Lesson 17? Tell me, and I'll pitch
Lesson 26c to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.10.3**: the three elision
  rules (rule 2 taught in depth here; all three in the Book's precise wording)
  and the returned-reference-to-local failure (`E0515`).
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 26 — Lifetime annotations](26-lifetime-annotations.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 26c — Lifetimes in structs →](26c-lifetimes-in-structs.md)
