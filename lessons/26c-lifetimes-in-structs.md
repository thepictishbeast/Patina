# Lesson 26c — Lifetimes in structs: closing the Lesson 18 promise

*(Phase 6, part 7 — the generics trio's finale. Lesson 18 forbade a `&str` struct
field "because that needs a lifetime — a Phase-6 topic." This is that topic,
cashed.)*

## 1. Why it exists

Sometimes a struct shouldn't *own* its data — it just needs a window into text
that already exists somewhere else, without copying it. A borrowed field is
exactly a reference with a lifetime: the struct must promise it won't outlive
what it borrows. This lesson writes that promise.

## 2. The idea

Declare the lifetime on the struct, attach it to the field:

```
struct Excerpt<'a> {
    part: &'a str,
}
```

The `<'a>` on the struct means *an `Excerpt` may not outlive the `&str` it holds
in `part`*. The compiler then guarantees you can't keep an `Excerpt` around
after the text it borrows from is dropped — the same relationship-checking as
`longest`, now attached to a type.

**When to borrow vs own:** owning the data (`String`, Lesson 18's default) is
simpler and right when the struct should stand alone. Borrowing (`&'a str`) is
right when the struct is a short-lived *view* — no copy, but it's tied to its
source. Both are correct tools; this lesson just unlocks the second one.

## 3. A tiny example to read

Slice the first sentence out of a novel and hold it in a borrowing struct. (The
`find`/`match` move is Lesson 26b's — everything here is a tool you have.)

```rust
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = match novel.find('.') {
        Some(i) => &novel[..i],
        None => novel.as_str(),
    };
    let e = Excerpt { part: first_sentence };
    println!("Excerpt: {}", e.part);
}
```

```
Excerpt: Call me Ishmael
```

`e` borrows from `novel` — and the compiler now watches that relationship: drop
`novel` while `e` is still in use and you'd get the Lesson-16b "does not live
long enough" refusal.

## 4. Common pitfalls / real compiler errors — the deferred `E0106`, closed

This is the exact snippet Lesson 18 deferred. A `&str` field with no lifetime:

```rust
struct Holder {
    text: &str,
}
```

```
error[E0106]: missing lifetime specifier
 --> main.rs:2:11
  |
2 |     text: &str,
  |           ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct Holder<'a> {
2 ~     text: &'a str,
  |
```

The fix is the `Excerpt<'a>` shape from part 3 — and the compiler prints it
verbatim. (Or, as Lesson 18 said: just **own it** with `String`. Both are right;
the lifetime version lets the struct *borrow* without copying.) The matching
exercise below hands you this wall — **predict the code** before you run.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new struct_lifetimes` works too.)* **Predict on
paper before each run.**

1. **A borrowing struct of your own.** Define `Highlight<'a>` holding a
   `&'a str`. In `main`, make a `String`, slice a piece of it (your `find`
   skills from 26b), and hold the slice in a `Highlight`. Print it. **Predict**
   the output.
2. **Trigger the deferred `E0106`.** Write the field *without* the lifetime.
   **Predict the error code** (you met it in Lesson 18) and the exact two-line
   fix the compiler will print.

*(You write every line here — I won't. The predictions are your answer key. That
completes the generics trio — `<T>` for types, bounds for behaviour, `'a` for
reference validity. Next, Phase 8: closures.)*

## 6. What surprised you?

A sentence or two: did the Lesson 18 `&str`-field mystery finally close? Does
borrow-vs-own for fields feel like a real choice now rather than a rule? Tell
me, and I'll fold it into the Phase-6 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.10.3**: lifetime annotations
  in struct definitions (`ImportantExcerpt`, adapted here as `Excerpt`).
- This lesson cashes the `E0106` forward-reference from **Lesson 18** and builds
  on the borrow checker of **Lessons 16/16b**.
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024. This completes Phase 6's generics trio
  (generics · traits · lifetimes).

---

<!-- lesson-nav -->
[← Lesson 26b — Lifetime elision](26b-lifetime-elision.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 27 — Closures: unnamed inline functions →](27-closure-syntax.md)
