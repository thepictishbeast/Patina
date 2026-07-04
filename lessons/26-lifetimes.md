# Lesson 26 — Lifetimes: annotations (`'a`), the borrow checker, elision

*(Phase 6 — Generics, part 3 and the last of the trio. Lesson 24 gave you generics
(`<T>`) — a stand-in for any **type**. Lifetimes are generics too, but the thing they
stand in for is **how long a reference stays valid**. They're the part of the borrow
checker (Lesson 16) you've been using all along without naming. This lesson finally
cashes the promise from Lesson 18: the `&str` struct field that needed "a lifetime,
a Phase-6 topic.")*

## 1. Why it exists

Back in Lesson 16 the borrow checker stopped you from using a reference after the data
it pointed to was gone — a **dangling reference**, a pointer to nothing. It did that
silently, by tracking how long each value lives. Lifetimes are the *names* for those
spans, and they exist for the one job the checker can't always do alone: when a function
hands back a reference, the compiler needs to know **which input that reference came
from**, so it can guarantee the result doesn't outlive its source.

Most of the time the compiler figures this out by itself and you never type a lifetime
(part 3 shows one such case). But when a function takes *more than one* reference and
returns one, the compiler can't guess which input the output borrows from — and it stops
and asks you to say so. That annotation is the whole topic.

> **How the sources frame it:** the **BOOK** Ch.10.3 is the backbone — it motivates
> lifetimes by *breaking* code three different ways (a dangling reference, an ambiguous
> `longest`, a returned reference to a local), then introduces `'a` as the fix, and lays
> out the three **elision rules** that explain why you rarely write them. **CR** reinforces
> with the "borrow both / borrow one" framing. No metaphor is invented here — the borrow
> checker's own diagrams carry it.

## 2. The idea

A **lifetime** is the span during which a reference is valid. Every reference has one;
usually the compiler infers it and you never see it. A **lifetime annotation** — written
`'a` (an apostrophe and a name, said "tick-a") — lets you *name* a lifetime so you can
say how two references' lifetimes **relate**.

Here is the single idea beginners trip on, so read it twice:

> **An annotation does not change how long anything lives.** Writing `'a` can't make a
> value live longer or a reference die sooner. It only **describes a relationship the
> compiler then checks** — "these references share a lifetime; the result lasts as long
> as the *shorter* of the inputs." If the real code violates the relationship, you get an
> error. The annotation is a *promise about* lifetimes, not a *lever on* them.

The syntax mirrors generics. You declare the lifetime in angle brackets after the
function name, then attach it to the references:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

Reading each `'a` in that signature:

- `<'a>` — declares a lifetime, named `'a`.
- `x: &'a str` — x lives at least as long as `'a`.
- `y: &'a str` — and so does y.
- `-> &'a str` — the result borrows from one of them, so it also lives at least `'a`.

That signature tells the compiler: *the returned reference is valid for as long as both
`x` and `y` are valid.* Now the compiler can enforce, at every call site, that you don't
keep the result around after either input has gone — which is exactly the dangling
reference it was trying to prevent.

**Why you rarely write them — lifetime elision.** The compiler applies three rules to
fill lifetimes in for you; if the rules cover every reference, you write nothing:

1. Each reference **parameter** gets its own lifetime.
2. If there's **exactly one** input lifetime, it's given to **all outputs**. (This is why
   `fn first_word(s: &str) -> &str` needs no annotation — one input, so the output
   obviously borrows from it.)
3. If one of the parameters is **`&self` or `&mut self`** (a method), the output gets
   `self`'s lifetime.

`longest` defeats rule 2 — it has *two* input references, so the compiler can't tell which
one the output borrows from, and you must say. That's the whole reason `longest` needs `'a`
and `first_word` doesn't.

## 3. Tiny examples to read

**The function that needs an annotation.** Two `&str` in, one `&str` out — rule 2 doesn't
apply, so you name a shared lifetime `'a`:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("abcd");
    let s2 = String::from("xyz");
    let result = longest(s1.as_str(), s2.as_str());
    println!("The longest string is {result}");
}
```

```
The longest string is abcd
```

The `'a` says "the result lives as long as the shorter of `x` and `y`." Here both live the
whole `main`, so `result` is fine. (Make one of them die early and the compiler rejects it —
that's exactly the protection the annotation buys you; you'll provoke it in part 5.)

**The function that needs *no* annotation — elision rule 2 at work.** One reference in,
one out, so the compiler fills the lifetime in silently:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

fn main() {
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("first word: {word}");
}
```

```
first word: hello
```

Same shape of problem — a function returning a borrow — but here you write zero lifetimes.
This is the common case; `longest` (two inputs) is the exception.

**A struct that holds a reference — cashing the Lesson 18 promise.** Remember in Lesson 18
the `&str` field was forbidden because "a borrowed field has to promise it won't outlive
what it borrows — that's a lifetime, a Phase-6 topic"? This is that promise, written:

```rust
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let e = Excerpt { part: first_sentence };
    println!("Excerpt: {}", e.part);
}
```

```
Excerpt: Call me Ishmael
```

The `<'a>` on the struct means *an `Excerpt` may not outlive the `&str` it holds in `part`*.
The compiler now guarantees you can't keep an `Excerpt` around after the `novel` it borrows
from is dropped. (For data you want to keep freely, owning it with `String` is still the
simpler default — Lesson 18.)

## 4. Common pitfalls / real compiler errors

**A reference that outlives its data — `E0597`.** The original dangling reference. `x` is
created in an inner scope, borrowed into the outer `r`, then dropped when the scope ends —
leaving `r` pointing at nothing:

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {r}");
}
```

```
error[E0597]: `x` does not live long enough
 --> main.rs:5:13
  |
4 |         let x = 5;
  |             - binding `x` declared here
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     println!("r: {r}");
  |                   - borrow later used here
```

This is the borrow checker doing its core job — and notice it tracks *exactly* the spans:
`x` is "dropped here while still borrowed," and the borrow is "later used here." Lifetimes
are just how it reasons about this across function boundaries.

**Two reference inputs, no annotation — `E0106`.** Drop the `'a` from `longest` and the
compiler can't tell whether the result borrows from `x` or from `y`:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

```
error[E0106]: missing lifetime specifier
 --> main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
1 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++
```

Read the `help` line — it states the problem in plain words ("does not say whether it is
borrowed from `x` or `y`") and the second help block hands you the exact fix. Elision rule 2
fails here because there are *two* inputs.

**The same `E0106` you met in Lesson 18 — a borrowed struct field.** This is the snippet
that was deferred. A `&str` field with no lifetime fails for the identical reason:

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

The fix is the `Excerpt<'a>` shape from part 3: add `<'a>` to the struct and `'a` to the
field. (Or, as Lesson 18 said, just **own it** with `String` — both are right; the lifetime
version lets the struct *borrow* without copying.)

**Returning a reference to something the function owns — `E0515`.** Here a fresh `String`
is made *inside* the function and a reference to it is returned. The moment the function
ends, that `String` is dropped — so the reference would dangle. The compiler refuses:

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

No annotation can fix this — the data genuinely dies at the closing brace. This is the
honest limit lifetimes enforce: you can return a reference that **borrows from an input**
(like `longest` returning `x` or `y`), but never one that **borrows from a local**. To hand
back freshly-made data, return the owned `String` itself.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new lifetimes`. **Predict on paper before each run** —
the error codes are the answer key.

1. **Build a `shorter` function — and watch it demand a lifetime.** Write a function that
   takes two `&str` and returns whichever is **shorter** (fewer bytes). First write the
   signature with **no** lifetime annotations and **predict the error code** before
   compiling. Then add the lifetime annotation that fixes it. Call it from `main` on two
   string literals and print the result. (Use `longest` from part 3 only as a *shape*
   reference — write `shorter` yourself, do not copy it.)

2. **Make the borrow checker bite.** Using your working `shorter`, set up a `result`
   binding in an outer scope, then assign to it from inside an **inner** scope where one of
   the two strings is created. Try to print `result` *after* the inner scope closes.
   **Predict which error code** you'll get and **which variable** the message will name as
   "does not live long enough" — then run it and check.

3. **Try to return a reference to a local — predict `E0515`.** Write a function
   `fn loudest(s: &str) -> &str` that builds a new uppercase `String` inside (look up
   `to_uppercase`) and tries to return a `&str` slice of it. **Predict the error code**
   before compiling. Then ask yourself: what's the smallest change that makes it compile?
   (Hint: stop returning a *borrow* of the local.)

4. **Find the elided lifetime.** Write the simplest possible function that takes a single
   `&str` and returns it unchanged, with **no** lifetime annotation written anywhere.
   **Predict whether it compiles** and, if so, **which elision rule** (1, 2, or 3 from
   part 2) makes the annotation unnecessary. Run it to confirm.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. Once `'a` clicks as "a name for how two references' lifetimes relate — a promise the
compiler checks, not a lever that changes anything," lifetimes stop being scary syntax and
become the borrow checker simply asking you to fill in a blank.)*

## 6. What surprised you?

A sentence or two: did "an annotation describes a relationship but changes nothing" land —
or did you expect `'a` to somehow *extend* a value's life? Did elision explain why you'd
never typed a lifetime before this lesson, despite returning references since Lesson 17?
Did the Lesson 18 `&str`-field mystery finally close? Tell me, and I'll fold it into the
Phase-6 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.10.3** "Validating References with
  Lifetimes": the dangling-reference motivation (`E0597`), the generic `longest` function
  and its `<'a>` annotation, lifetime annotations in **struct definitions**
  (`ImportantExcerpt`, adapted here as `Excerpt`), the returned-reference-to-local failure
  (`E0515`), and the **three lifetime elision rules** (kept in BOOK's precise wording —
  rule 3 is "one of the parameters is `&self` or `&mut self`").
- **CR** — *Comprehensive Rust* (Google), §24 Lifetimes: the "borrow both inputs / borrow
  one input" framing that motivates *why* a shared `'a` is the right relationship for
  `longest`.
- **BLOG** — not used here; it mentions "lifetime" only colloquially and never teaches `'a`,
  the borrow checker as a lifetime analyzer, or elision.
- This lesson cashes the `E0106` forward-reference from **Lesson 18** (the borrowed `&str`
  struct field) and builds on the borrow checker introduced in **Lesson 16**.
- Every snippet compiled and run, and every error captured live, on **rustc 1.95.0**,
  edition 2024 (`rustc --edition 2024 FILE.rs`). This completes the generics trio of
  Phase 6 (L24 generics · L25 traits · L26 lifetimes).

---

<!-- lesson-nav -->
[← Lesson 25 — Traits: Defining Shared Behavior](25-traits.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 27 — Closures: Anonymous Functions That Capture →](27-closures.md)
