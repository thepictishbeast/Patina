# Lesson 25 — Traits: declare and implement

*(Phase 6 — Generics, part 3. A **trait** names a behaviour a type can promise to
provide. This lesson is about making and keeping that promise; the next one is
about *demanding* it — which is what finally closes Lesson 24's `E0369` wall.)*

## 1. Why it exists

Different types often share behaviour. A social post and a news article are
unrelated structs, but both can produce a one-line *summary* of themselves. A
**trait** names that shared behaviour: a set of methods a type promises to
provide. A type **implements** the trait to keep the promise — and the compiler
**enforces** it: leave a required method out and the build fails (part 4).

## 2. The idea

You **declare** a trait with the `trait` keyword and a list of method signatures:

```
trait Summary {
    fn summarize(&self) -> String;   // a REQUIRED method — just a signature, no body
}
```

A method with **no body** (a signature ending in `;`) is **required**: any
implementing type must write it. A method **with** a body is a **default**: the
trait provides it for free, and a type can take it or override it.

You **implement** a trait for a type with `impl Trait for Type`:

```
impl Summary for SocialPost {
    fn summarize(&self) -> String { /* ... */ }
}
```

Once a type implements `Summary`, you can call `.summarize()` on its values.
This works for the standard library's traits too — `impl std::ops::Add for
YourType` is the same move, and it's what makes `+` work on your own types (one
of the exercises below has you do exactly that).

Two ground rules to file away:
- **A trait's methods are only callable where the trait is in scope** — using a
  trait someone else defined usually starts with a `use` line. (An exercise
  hands you the error that teaches this.)
- **The orphan rule:** you may write `impl Trait for Type` only if the trait
  *or* the type is your own — the story, and the newtype way around it, is in
  the Book, Ch. 10.2 (and one of the exercises walks you into it).

## 3. A tiny example to read

**A trait, two implementers, a required + a default method.** `SocialPost`
implements only the required `summarize_author`, so it inherits the default
`summarize`. `NewsArticle` overrides `summarize`:

```rust
trait Summary {
    fn summarize_author(&self) -> String;          // required
    fn summarize(&self) -> String {                // default — calls the required one
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct SocialPost {
    username: String,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    fn summarize(&self) -> String {                // overrides the default
        format!("{}, by {}", self.headline, self.summarize_author())
    }
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Cup"),
        author: String::from("Iceburgh"),
    };
    println!("{}", post.summarize());
    println!("{}", article.summarize());
}
```

```
(Read more from @horse_ebooks...)
Penguins win the Cup, by Iceburgh
```

`post` used the **default** `summarize` (which called *its* `summarize_author`);
`article` used its **own**. Same trait, two types, two promises kept.

## 4. Common pitfalls / real compiler errors — forgetting a required method

A default method is free, but a required one is not. Here the `impl` block is
empty, so `summarize_author` is never supplied:

```rust
trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Photo {
    caption: String,
}

impl Summary for Photo {}
```

**Before you scroll — what does the compiler say?**

```
error[E0046]: not all trait items implemented, missing: `summarize_author`
  --> main.rs:12:1
   |
 2 |     fn summarize_author(&self) -> String;
   |     ------------------------------------- `summarize_author` from trait
...
12 | impl Summary for Photo {}
   | ^^^^^^^^^^^^^^^^^^^^^^ missing `summarize_author` in implementation
```

The compiler will take the default off your hands, but it won't invent the
**required** method — that promise is *yours* to keep. Add it and it compiles.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then work through the matching
exercises via the **Practice this lesson** links at the bottom — implementing,
scope, `Add`, and the orphan rule are all in there. *(On your own machine, a
playground or `cargo new traits` works too.)* **Predict on paper before each
run.**

1. **A `Describe` trait with a default.** Declare `trait Describe` with one
   required `fn name(&self) -> String` and one default `fn describe(&self) ->
   String` that calls `self.name()`. Implement it for a `Dog` and a `Cat`,
   supplying only `name`. **Predict** both `.describe()` outputs.
2. **Override the default** for `Cat` only. **Predict** which output changed.
3. **Trigger `E0046` on purpose.** Add a `Fish` with an empty `impl Describe for
   Fish {}`. **Predict the error code**, read what's missing, fix it.

*(You write every line here — I won't. The predictions are your answer key.
Next: DEMANDING a promise from a generic `T` — the trait bound, and the close of
Lesson 24's cliffhanger.)*

## 6. What surprised you?

A sentence or two: did "a trait is a set of methods a type promises to provide"
land cleanly? Did required-vs-default make sense once `E0046` enforced it? Tell
me, and I'll pitch Lesson 25b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.10.2**: declaring a trait,
  `impl Trait for Type`, default methods that call required ones, and the orphan
  rule. The `Summary` example is adapted from Listings 10-12 through 10-14.
- **CR** — *Comprehensive Rust* (Google), §13.2: the default-method idea. Both
  sources' "like an interface" line is dropped per the no-analogy rule — the
  plain definition carries it.
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 24b — Generic structs, enums & methods](24b-generic-types.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 25b — Trait bounds: demanding behaviour →](25b-trait-bounds.md)
