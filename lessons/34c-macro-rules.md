# Lesson 34c — Declarative macros: `macro_rules!`

*(Phase 9 — Advanced, part 8. Every `println!`, `vec!`, and `assert!` you've
typed ends in `!` — they're **macros**: code that writes code, expanded before
compilation. This lesson opens the box: you'll read the pattern language of
`macro_rules!` and build a stripped-down `vec!` of your own.)*

## 1. Why it exists

A function takes a fixed number of arguments with fixed types. But
`println!("{a} {b}")` inspects a format string at compile time, and
`vec![1, 2, 3]` takes *any number* of elements — no function can do either.
A **macro** can, because it runs *before* compilation: it receives the raw
code you wrote, matches it against patterns, and **generates** the real code
that then gets compiled. The `!` is the marker that expansion is happening.

`macro_rules!` is the **declarative** way to write one: no logic, just
pattern → template, like a `match` whose arms produce code.

## 2. The idea

- **Patterns like `match` arms.** A `macro_rules!` definition is a list of
  `( pattern ) => { template }` rules. The compiler matches your call against
  the patterns, first hit wins.
- **Captures.** `$x:expr` captures one expression into the variable `$x`.
  (Other capture kinds exist — `ident`, `ty`, `stmt` — same idea.)
- **Repetition.** `$( … ),*` around a capture means "zero or more of these,
  comma-separated." The same `$( … )*` marker in the *template* replays the
  enclosed code once per captured item — repetition without a `for` in sight.
- **Expansion happens before compiling.** The generated code is what actually
  gets type-checked and built; the macro itself never runs at runtime.

> **How the sources frame it:** the **BOOK** Ch.20 §20.5 "Macros" is the
> backbone — the `vec!` skeleton below is its flagship listing. It also names
> what this lesson leaves out: procedural macros (`#[derive(…)]` — the
> machinery behind Lesson 18c's `derive(Debug)`), which are a different,
> heavier tool. Read §20.5 when you're curious what powers `derive`.

## 3. A tiny example to read

**A mini `vec!`.** `my_vec!` takes any number of comma-separated expressions
and builds a `Vec`. Read it as a `match`: the pattern `$( $x:expr ),*` means
"zero or more expressions, comma-separated"; the body repeats `v.push($x);`
once **per** matched expression:

```rust
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3];
    println!("v = {v:?}");
}
```

```
v = [1, 2, 3]
```

So `my_vec![1, 2, 3]` expands, *before compiling*, into roughly: make a `Vec`,
push `1`, push `2`, push `3`, hand it back. The real `vec!` does the same job
(plus pre-sizing and more patterns); this is its skeleton. (Adapted from the
BOOK's `vec!` listing in Ch.20.)

## 4. Common pitfalls / real compiler errors — the pattern is literal

**A trailing comma your pattern didn't allow.** Rust collections tolerate a
trailing comma everywhere — so your users will type one. But a macro matches
its pattern *literally*, and `$( $x:expr ),*` puts commas strictly *between*
items:

```rust
fn main() {
    let v = my_vec![1, 2, 3,];   // note the trailing comma
    println!("v = {v:?}");
}
```

**Before you scroll — the real `vec!` accepts this. Will `my_vec!`?**

```
error: unexpected end of macro invocation
  --> main.rs:14:29
   |
 1 | macro_rules! my_vec {
   | ------------------- when calling this macro
...
14 |     let v = my_vec![1, 2, 3,];
   |                             ^ missing tokens in macro arguments
   |
note: while trying to match meta-variable `$x:expr`
   --> main.rs:2:10
   |
 2 |     ( $( $x:expr ),* ) => {
   |          ^^^^^^^^^^
```

Read the compiler's reasoning: after the last comma it tried to match
*another* `$x:expr` — because in your pattern, a comma always promises another
expression — and the input ended instead. The fix is to *widen the pattern*:
`( $( $x:expr ),* $(,)? )` adds "and optionally one trailing comma" (`?` means
zero-or-one). One token of grammar, and `my_vec![1, 2, 3,]` builds. This is
the macro mindset in one error: **you're not writing logic, you're writing a
grammar — and your users will find its edges.**

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new macros` works too.)* **Predict on paper before each
run.**

1. **Re-derive the skeleton.** Write the `my_vec!` macro from memory (don't
   copy part 3), then call it with three numbers. **Predict** the output
   before running. If you hit an error, read *which rule* the compiler was
   trying to match — that's the macro debugging skill.
2. **Find the grammar's edge.** Call your macro with a trailing comma.
   **Predict**: does it build? Then widen the pattern with `$(,)?` and
   confirm the same call now works. In one sentence: why did the original
   pattern reject it?
3. **Bend the template.** Change *one* thing in the body: make it push each
   element *twice*, or build a `Vec` of the *string* form of each element
   (`$x.to_string()`), or print a line per element instead of collecting.
   **Predict** the output for a 3-element call before running. Notice that
   the repetition `$( … )*` in the body is what loops — there's no `for` in
   sight.

*(You write every line here — I won't. The predictions are your answer key.
That completes the advanced-features tour — `unsafe` (34), operators and
associated types (34b), and macros (34c). The capstone remains — two parts,
where the whole course builds a real multithreaded web server.)*

## 6. What surprised you?

A sentence or two: did the mini-`my_vec!` make `vec![...]` feel less magical —
code that writes code, expanded before compilation? Did the trailing-comma
error change how you think about macro patterns (a grammar with edges, not a
function signature)? Tell me, and I'll fold it into the Phase-9 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.20 §20.5 "Macros"**:
  declarative macros with `macro_rules!`, the `vec!` skeleton this lesson's
  `my_vec!` is adapted from, and the declarative-vs-procedural distinction
  (procedural macros — the `derive` machinery — are pointed at, not taught).
- **CR** — *Comprehensive Rust* (Google): brief macro mentions; the teaching
  arc here is the Book's.
- Every snippet compiled and run, and the error captured live, on **rustc
  1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`; temp paths
  normalized to `main.rs`). The `$(,)?` trailing-comma fix was verified to
  build and run.

---

<!-- lesson-nav -->
[← Lesson 34b — Operator overloading & associated types](34b-operator-overloading.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 35 — Capstone I: a single-threaded web server →](35-single-threaded-server.md)
