# Lesson 26 — Lifetime annotations: the `longest` function

*(Phase 6 — Generics, part 5 and the last pillar. Lesson 24 gave you generics
(`<T>`) — a stand-in for any **type**. Lifetimes are generics too, but the thing
they stand in for is **how long a reference stays valid**. They're the part of
the borrow checker (Lessons 16/16b) you've been using all along without naming.)*

## 1. Why it exists

Back in Lesson 16b the borrow checker stopped you from using a reference after
the data it pointed to was gone. It did that silently, by tracking how long each
value lives. Lifetimes are the *names* for those spans, and they exist for the
one job the checker can't do alone: when a function takes *more than one*
reference and returns one, the compiler can't guess **which input the output
borrows from** — and it stops and asks you to say so. That annotation is the
whole topic.

## 2. The idea

A **lifetime** is the span during which a reference is valid. Every reference
has one; usually the compiler infers it and you never see it. A **lifetime
annotation** — written `'a` (an apostrophe and a name, said "tick-a") — lets you
*name* a lifetime so you can say how two references' lifetimes **relate**.

Here is the single idea beginners trip on, so read it twice:

> **An annotation does not change how long anything lives.** Writing `'a` can't
> make a value live longer or a reference die sooner. It only **describes a
> relationship the compiler then checks** — "these references share a lifetime;
> the result lasts as long as the *shorter* of the inputs." If the real code
> violates the relationship, you get an error. The annotation is a *promise
> about* lifetimes, not a *lever on* them.

The syntax mirrors generics. Declare the lifetime in angle brackets after the
function name, then attach it to the references:

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

Reading each `'a` in that signature:

- `<'a>` — declares a lifetime, named `'a`.
- `x: &'a str` — x lives at least as long as `'a`.
- `y: &'a str` — and so does y.
- `-> &'a str` — the result borrows from one of them, so it also lives at
  least `'a`.

That signature tells the compiler: *the returned reference is valid for as long
as both `x` and `y` are valid* — so at every call site it can stop you keeping
the result around after either input is gone.

## 3. A tiny example to read

Two `&str` in, one `&str` out — the compiler can't guess which, so you name a
shared lifetime:

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

Here both inputs live the whole `main`, so `result` is fine. Make one of them
die early and the compiler rejects the call — that's exactly the protection the
annotation buys, and you'll provoke it in part 5.

## 4. Common pitfalls / real compiler errors — two inputs, no annotation

Drop the `'a` from `longest` and the compiler can't tell whether the result
borrows from `x` or from `y`:

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

Read the `help` line — it states the problem in plain words ("does not say
whether it is borrowed from `x` or `y`") and the second block hands you the
exact fix. (And if the result *outlives* an input at a call site, you get the
`E0597` "does not live long enough" you already met in Lesson 16b — same
checker, now reasoning across a function boundary.)

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new lifetimes` works too.)* **Predict on paper
before each run** — the error codes are the answer key.

1. **Build a `shorter` function — and watch it demand a lifetime.** Write a
   function that takes two `&str` and returns whichever is **shorter**. First
   write it with **no** annotations and **predict the error code**. Then add the
   annotation that fixes it and call it from `main`. (Use `longest` only as a
   *shape* reference — write `shorter` yourself.)
2. **Make the borrow checker bite.** With your working `shorter`, assign its
   result from inside an inner scope where one input is created, then print the
   result *after* the scope closes. **Predict which error code** and **which
   variable** the message names as "does not live long enough."

*(You write every line here — I won't. The predictions are your answer key.
Next: why you almost never actually write `'a` — the elision rules.)*

## 6. What surprised you?

A sentence or two: did "an annotation describes a relationship but changes
nothing" land — or did you expect `'a` to somehow *extend* a value's life? Tell
me, and I'll pitch Lesson 26b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.10.3** "Validating References
  with Lifetimes": the ambiguous `longest` and its `<'a>` annotation (Listings
  10-20 through 10-22).
- **CR** — *Comprehensive Rust* (Google), §24: the "borrow both inputs / borrow
  one input" framing.
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 25b — Trait bounds: demanding behaviour](25b-trait-bounds.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 26b — Lifetime elision →](26b-lifetime-elision.md)
