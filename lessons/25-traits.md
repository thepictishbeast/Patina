# Lesson 25 — Traits: Defining Shared Behavior

*(Phase 6 — Generics, part 2. Lesson 24 gave you generics: one function, like
`largest<T>`, that works over many types. But it left a gap — a generic body
has to be valid for **every** `T`, so the compiler won't let you do anything to
a bare `T` that not all types can do (that was the `>` error, `E0369`). A
**trait** is how you say what a type must be **able to do**. Bolt a trait onto a
generic — `<T: Summary>` — and the gap closes: now the function knows `T` has
the methods it needs.)*

## 1. Why it exists

Different types often share behavior. A social post and a news article are
unrelated structs, but both can produce a one-line *summary* of themselves. You
want to write `notify(item)` **once** and have it work for either — without
copying the function per type, and without it silently accepting a type that
*can't* be summarized.

A **trait** names that shared behavior: a set of methods a type promises to
provide. A type **implements** the trait to keep the promise. And the key part
— the compiler **enforces** the promise at compile time. If a function asks for
"something that implements `Summary`," you cannot hand it a plain number; the
build fails before the program ever runs (part 4).

> **How the sources frame it:** the **BOOK** Ch.10.2 is the whole backbone — it
> declares a `trait`, implements it for a type, adds a **default method** that
> calls a required one, then uses the trait as a function parameter and as a
> bound. **CR** reinforces the tiny default-method idea. Both sources reach for
> an "it's like an interface in other languages" line — **dropped here** as dead
> weight for a no-background learner. A trait is just *a named set of methods a
> type promises to provide*; that carries it.

## 2. The idea

You **declare** a trait with the `trait` keyword and a list of method
signatures:

```
trait Summary {
    fn summarize(&self) -> String;   // a REQUIRED method — just a signature, no body
}
```

A method with **no body** (just a signature ending in `;`) is **required**: any
type implementing `Summary` must write that method itself. A method **with** a
body is a **default**: the trait provides it for free, and a type can either
take the default or override it.

You **implement** a trait for a type with `impl Trait for Type`:

```
impl Summary for SocialPost {
    fn summarize(&self) -> String { /* ... */ }
}
```

Once a type implements `Summary`, you can call `.summarize()` on its values.

**Using a trait to constrain code** — three shapes, same idea:

- **`impl Trait` parameter** — `fn notify(item: &impl Summary)` reads as "give
  me a reference to *anything* that implements `Summary`." Inside, you may call
  the trait's methods on `item`.
- **A trait bound** — `fn notify<T: Summary>(item: &T)` says the same thing in
  the generic form you met last lesson: "`T` can be any type, **as long as** it
  implements `Summary`." `impl Trait` is just shorthand for this.
- **`impl Trait` return** — `fn make() -> impl Summary` means "I return *some*
  type that implements `Summary`; you don't need to know which one."

For longer or multiple bounds, a **`where` clause** moves them off the signature
line so it stays readable:

```
fn notify<T>(item: &T) where T: Summary { /* ... */ }
```

**The orphan rule, in one line:** you may write `impl Trait for Type` only if
the trait **or** the type is defined in your own crate — you can't implement
someone else's trait for someone else's type.

## 3. Tiny examples to read

**A trait, two implementers, a required + a default method.** `SocialPost`
implements only the required `summarize_author`, so it inherits the default
`summarize`. `NewsArticle` overrides `summarize` with its own version:

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

fn notify(item: &impl Summary) {                   // accepts anything that is Summary
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Cup"),
        author: String::from("Iceburgh"),
    };
    notify(&post);
    notify(&article);
}
```

```
Breaking news! (Read more from @horse_ebooks...)
Breaking news! Penguins win the Cup, by Iceburgh
```

`post` used the **default** `summarize` (which called *its* `summarize_author`);
`article` used its **own** `summarize`. One `notify` served both — that's the
trait doing the work.

**The bound, written three ways, plus an `impl Trait` return.** All four
functions below say "this works for any type that is `Summary`":

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    who: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("tweet from {}", self.who)
    }
}

fn announce<T: Summary>(item: &T) {        // the generic bound
    println!("Now: {}", item.summarize());
}

fn announce_where<T>(item: &T)             // same bound, moved to a where clause
where
    T: Summary,
{
    println!("Later: {}", item.summarize());
}

fn make_tweet() -> impl Summary {          // returns "some type that is Summary"
    Tweet { who: String::from("ferris") }
}

fn main() {
    let t = make_tweet();
    announce(&t);
    announce_where(&t);
}
```

```
Now: tweet from ferris
Later: tweet from ferris
```

`<T: Summary>` and `where T: Summary` compile to exactly the same thing — pick
whichever reads better. `make_tweet`'s caller never names `Tweet`; it just gets
back "something summarizable."

## 4. Common pitfalls / real compiler errors

**Passing a type that doesn't implement the trait — `E0277`.** `notify` demands
`Summary`, and a plain integer doesn't have it:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

fn notify(item: &impl Summary) {
    println!("News! {}", item.summarize());
}

fn main() {
    notify(&5);
}
```

```
error[E0277]: the trait bound `{integer}: Summary` is not satisfied
  --> main.rs:10:12
   |
10 |     notify(&5);
   |     ------ ^^ the trait `Summary` is not implemented for `{integer}`
   |     |
   |     required by a bound introduced by this call
   |
help: this trait has no implementations, consider adding one
  --> main.rs:1:1
   |
 1 | trait Summary {
   | ^^^^^^^^^^^^^
note: required by a bound in `notify`
  --> main.rs:5:23
   |
 5 | fn notify(item: &impl Summary) {
   |                       ^^^^^^^ required by this bound in `notify`
```

This is the bound doing its job. The compiler names exactly what's missing —
*`Summary` is not implemented for `{integer}`* — and points at the bound that
required it. The fix is to pass a type that *does* implement `Summary`.

**Forgetting a required method — `E0046`.** A default method is free, but a
required one is not. Here the `impl` block is empty, so `summarize_author` is
never supplied:

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

fn main() {
    let p = Photo { caption: String::from("hi") };
    println!("{}", p.summarize());
}
```

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

The compiler will take the default `summarize` off your hands, but it won't
invent the **required** `summarize_author` — that one is the promise *you* must
keep. Add the method to the `impl` block and it compiles.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new traits`. **Predict on paper before each
run.**

1. **A `Describe` trait with a default.** Declare `trait Describe` with one
   required method `fn name(&self) -> String` and one default method
   `fn describe(&self) -> String` whose body returns something like
   `"This is <name>"` by calling `self.name()`. Make a struct `Dog` and a struct
   `Cat`; implement `Describe` for both, supplying only `name`. Call
   `.describe()` on one of each in `main`. **Predict** both lines of output
   before running — especially: where does the word in each `describe` come
   from?

2. **Override the default.** Add an `impl Describe for Cat` that *overrides*
   `describe` to return something different (e.g. `"A cat named <name>"`).
   Leave `Dog` using the default. **Predict** which animal's output changed and
   which stayed the same, then run to check.

3. **A function over the trait.** Write `fn introduce(item: &impl Describe)`
   that prints `item.describe()`, and call it with both your `Dog` and your
   `Cat`. Then rewrite the signature two more ways that mean the same thing:
   once as `fn introduce<T: Describe>(item: &T)`, and once using a `where`
   clause. **Predict** whether the output changes between the three versions.
   (It shouldn't — convince yourself *why*.)

4. **Trigger `E0277` on purpose.** Call `introduce(&5)` (or pass any type you
   never implemented `Describe` for). **Predict the error code** before you
   compile, then read the message — does it name the type and the missing
   trait? Fix it by passing a `Dog` instead.

5. **Trigger `E0046` on purpose.** Add a struct `Fish` with `impl Describe for
   Fish {}` — an empty block. **Predict the error code**, then read what the
   compiler says is missing. Fix it by adding the one required method.

*(You write every line here — I won't. The predictions are your answer key; the
code is yours. With traits you can now name a behavior, demand it with a bound,
and let the compiler reject anything that can't keep the promise.)*

## 6. What surprised you?

A sentence or two: did "a trait is a set of methods a type promises to provide"
land cleanly? Did the **required-vs-default** split make sense — that the
compiler gives you defaults for free but enforces the required ones (`E0046`)?
Did seeing `notify(&5)` get rejected at *compile* time (`E0277`) feel like the
same safety `Option`/`Result` gave you, now for behavior? Tell me, and I'll fold
it into the Phase-6 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.10.2** ("Traits: Defining
  Shared Behavior"): declaring a trait, `impl Trait for Type`, default methods
  that call required ones, traits as parameters (`impl Trait`), trait bounds
  (`<T: Summary>` and `where`), `impl Trait` return types, and the orphan rule.
  The `Summary` / `SocialPost` / `NewsArticle` / `notify` example is adapted
  from BOOK Listings 10-12 through 10-14, collapsed into one runnable file.
- **CR** — *Comprehensive Rust* (Google), §13.2: reinforces the default-method
  idea (a default body calling a required method). Its supertraits / associated
  types / `dyn Trait` material is out of beginner scope and left out.
- **BLOG** — not used here; it has no trait teaching.
- Every snippet compiled and run, and every error captured live, on **rustc
  1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`). Builds on Lesson 24
  (generics); the Phase-6 "& generics" arc continues with lifetimes next.

---

<!-- lesson-nav -->
[← Lesson 24 — Generics: `<T>` type parameters](24-generics.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 26 — Lifetimes: annotations (`'a`), the borrow checker, elision →](26-lifetimes.md)
