# Lesson 22c — `pub` on structs & enums: an asymmetry

*(Phase 6 — Organizing & generics, part 6. Lesson 22b's rules were about
modules and functions. Put `pub` on a TYPE and something odd happens: a
`pub struct` keeps its fields sealed, while a `pub enum` throws every variant
open. This lesson is that asymmetry — and the design pattern it forces.)*

## 1. Why it exists

Sooner or later a module exports a *type*, not just functions — and then the
question isn't only "can outsiders name it?" but "can they see *inside* it?"
Rust answers differently for the two kinds of type, and the difference isn't
arbitrary: it falls straight out of what each kind is *for*.

## 2. The idea

- A **`pub struct`** makes the *type* public, but **each field stays private
  unless you add `pub` to it**. A struct with any private field therefore
  needs a **public constructor** (an associated function like
  `Breakfast::summer`), because outside code can't fill a private field in.
- A **`pub enum`** makes **all of its variants public** at once — no
  per-variant `pub`.

Why the difference? An enum is almost useless if you can't name its variants;
a struct is often *most* useful with its fields hidden. So Rust defaults each
to the useful posture.

And the deeper rule underneath (CR): **privacy is bounded by the *module*,
not the type.** Code *in the same module* reads a struct's private fields
freely; only *outside* code is blocked. The fence is 22b's fence — types
don't get their own.

## 3. A tiny example to read

**A public struct with a hidden field, and a public enum:**

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // public field
        seasonal_fruit: String, // private — only this module can set it
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {   // the public constructor
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }

    pub enum Appetizer { Soup, Salad }   // BOTH variants public — no per-variant `pub`
}

pub fn eat() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");        // ok — `toast` is pub
    println!("I'd like {} toast", meal.toast);
    let _order = back_of_house::Appetizer::Soup;
}
```

You can change `toast`, but you couldn't set `seasonal_fruit` from here — the
chef picks the fruit. That's why `summer` exists: it's the only way to build
a `Breakfast` from outside. And `Appetizer::Soup` needed no ceremony at all —
`pub enum` opened every variant.

## 4. Common pitfalls / real compiler errors

**Touching a private field from outside — `E0616`.** Add one line to `eat`,
aiming at the private field:

```rust
    meal.seasonal_fruit = String::from("blueberries");
```

**Before you scroll — the struct is `pub`, the binding is `mut`. Enough?**

```
error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
  --> main.rs:14:10
   |
14 |     meal.seasonal_fruit = String::from("blueberries");
   |          ^^^^^^^^^^^^^^ private field
```

A `pub struct` does **not** make its fields public — `seasonal_fruit` is
sealed, so the only way to influence it is through the module's own code (the
`summer` constructor). Trying to *build* a `Breakfast` with a struct literal
from outside fails the same way under a different code — `E0451`, in **Book
§7.3**'s Breakfast listing — same lesson either way: **the constructor is the
door.**

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new visibility` works too.)* **Predict on paper before
each run.**

1. **A guarded struct.** Write a `pub struct Profile` inside a module, with a
   `pub` field `name` and a *private* field `id`. From `main`, try to build
   it with a struct literal directly — **predict the error code** (part 4
   names its cousin). Then add a `pub fn new(name: &str) -> Profile`
   constructor that fills `id` in, and use that instead.
2. **Read but don't write.** With your constructor working, try to *print*
   `profile.id` from `main`. **Predict**: does reading a private field get
   further than writing one did? What single word does the compiler use for
   the field either way?
3. **The enum side.** In the same module, add `pub enum Status { Active,
   Retired }` and build a `Status::Active` from `main`. **Predict**: any
   `pub` needed on the variants? Then answer in one sentence: why does the
   asymmetry point this way — what would a per-variant-private enum even be
   *for*?

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Paths, privacy, and type-shaped `pub` are the whole
access-control story — next, Lesson 23: `use`, so you can stop typing the
long paths you've earned.)*

## 6. What surprised you?

A sentence or two: did the struct/enum asymmetry feel arbitrary before the
"what is each kind *for*" framing — and after? Did "privacy is module-based,
not type-based" change where you'd put a helper function that needs the
private fields? Tell me, and I'll tune Lesson 23 to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§7.3**: `pub` on structs vs
  enums — the `Breakfast`/`Appetizer` examples (Listings 7-9/7-10, reproduced
  here) and the struct-literal `E0451` listing this lesson points at.
- **CR** — *Comprehensive Rust* (Google), §27.4: the marquee "privacy is
  module-based, not type-based" framing.
- Every snippet compiled (and the `main`-bearing ones run) on **rustc
  1.95.0**, edition 2024; `E0616` captured live (temp paths normalized to
  `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 22b — Privacy & pub: private by default](22b-privacy-and-pub.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 23 — The `use` Keyword →](23-the-use-keyword.md)
