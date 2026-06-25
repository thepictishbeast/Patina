# Lesson 22 — Paths & Visibility (`pub`)

*(Phase 6 — Organizing & generics, part 2. Lesson 21 built the module **tree**. Now the
two questions that make it usable: how do you **name** an item that lives in another module
(a *path*), and what is **allowed** to be named from where (*visibility*, controlled by
`pub`)? Paths reach; `pub` permits. They're two halves of one idea.)*

## 1. Why it exists

A module tree is only useful if you can (a) refer to something across it and (b) decide
what outsiders are allowed to touch. Rust keeps those separate and explicit:

- a **path** points at an item — `a::b::c` — exactly like a file path points at a file;
- **visibility** decides whether that path is *allowed*. Everything is **private by
  default**; `pub` opens a specific door.

Private-by-default is a feature, not a nuisance: anything you didn't mark `pub` is yours to
rename or rework freely, because no outside code could have depended on it.

> **How the sources frame it:** the **BOOK** §7.3 is the backbone — absolute-vs-relative
> paths side by side, the `super` example, and the signature **two-strike** `pub` walkthrough
> (a public module whose function is still private). **CR** owns the crisp rules — "modules
> are a **privacy boundary**", and the marquee fact that **privacy is module-based, not
> type-based**. The **filesystem** metaphor from Lesson 21 keeps paying off.

## 2. The idea

**Paths — like filesystem paths.** A path is `::`-separated and comes in two flavours:

- **Absolute** — starts at the crate root with the literal `crate::` (think `/`, the root):
  `crate::front_of_house::hosting::add_to_waitlist()`.
- **Relative** — starts from the *current* module: a sibling's name, or `self::` (this
  module, ≈ `.`), or **`super::`** (the parent module, ≈ `..`).

When in doubt, prefer **absolute** paths — they keep working if you move the *calling* code,
since they don't depend on where the call sits.

**Privacy by default, and `pub`.** Inside a crate:

- Every item is **private to code outside its module** by default.
- A **child** module can use its **ancestors'** private items; a **parent** *cannot* reach
  into a child's private items. (Privacy points one way: outward-in is blocked, inward-out is
  fine.)
- `pub` opens an item up. `pub(crate)` is a middle setting: public *within this crate*, but
  not to code that depends on your crate as a library.

The surprise worth burning in: **`pub` on a module does not make its contents public.**
Marking `pub mod hosting` only lets outside code *refer to* `hosting`; each function inside
still needs its own `pub`. You'll meet that as an error in part 4.

**`pub` on structs vs enums — an asymmetry.** This is the module rule with a twist:

- A **`pub struct`** makes the *type* public, but **each field stays private unless you add
  `pub` to it**. A struct with any private field therefore needs a **public constructor**
  (an associated function like `Breakfast::summer`), because outside code can't fill in a
  private field directly.
- A **`pub enum`** makes **all of its variants public** at once — no per-variant `pub`.

Why the difference? An enum is almost useless if you can't name its variants, so Rust makes
them all public; a struct is often useful with its fields hidden, so it makes you opt in
field by field. And note the deeper rule (CR): **privacy is bounded by the *module*, not the
type** — code *in the same module* can read a struct's private fields freely; only *outside*
code is blocked.

## 3. Tiny examples to read

**One function, reached two ways** (library-shaped — no `main`):

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();   // absolute
    front_of_house::hosting::add_to_waitlist();          // relative
}
```

Both calls hit the same function. The absolute one starts at `crate`; the relative one
starts from the sibling name `front_of_house`. (Both need `pub` on the module *and* the
function — see part 4 for what happens without it.)

**`super::` reaches the parent** (≈ `..`):

```rust
fn deliver_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();              // a sibling in this module
        super::deliver_order();    // up to the parent (crate root)
    }
    fn cook_order() {}
}
```

`deliver_order` is private at the crate root, yet `back_of_house` (its child) can call it —
children see their ancestors' private items.

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

You can change `toast`, but you couldn't set `seasonal_fruit` from here — the chef picks the
fruit. That's why `summer` exists: it's the only way to build a `Breakfast` from outside.

## 4. Common pitfalls / real compiler errors

**A public module with a private function — `E0603`.** Making the *module* `pub` is not
enough:

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}   // NOT pub
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
}
```

```
error[E0603]: function `add_to_waitlist` is private
 --> main.rs:7:37
  |
7 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^ private function
  |
note: the function `add_to_waitlist` is defined here
 --> main.rs:3:9
  |
3 |         fn add_to_waitlist() {}
  |         ^^^^^^^^^^^^^^^^^^^^
```

The module is reachable, but the function inside it isn't — exactly the "`pub` on a module ≠
`pub` on its contents" surprise. The fix is to add `pub` to the function too (`pub fn
add_to_waitlist`). The compiler even points at the definition so you know where to add it.

**Touching a private field from outside — `E0616`.** Swap the `meal.toast` line for the
private field:

```rust
    meal.seasonal_fruit = String::from("blueberries");
```

```
error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
  --> main.rs:14:10
   |
14 |     meal.seasonal_fruit = String::from("blueberries");
   |          ^^^^^^^^^^^^^^ private field
```

A `pub struct` does **not** make its fields public — `seasonal_fruit` is sealed off, so the
only way to influence it is through the module's own code (the `summer` constructor). (Try
to *build* a `Breakfast` directly with a struct literal from outside and you'll get a close
cousin, `E0451`, naming the private field — same lesson: the constructor is the door.)

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new visibility`. **Predict on paper before each run.**

1. **One function, two paths.** Build `mod garden { pub mod flowers { pub fn plant() {
   println!("planted"); } } }`. From `main`, call `plant()` **once by an absolute path and
   once by a relative path**. **Predict** both lines you'll write and the output, then run.

2. **The two-strike.** Remove `pub` from `plant` (keep `pub mod`). **Predict the error
   code**, run to check, then add `pub fn` back and confirm it builds. Which word did the
   compiler use for the function?

3. **A guarded struct.** Write a `pub struct Profile` with a `pub` field `name` and a
   *private* field `id`. From `main`, try to build it with a struct literal directly —
   **predict the error code**. Then add a `pub fn new(name: &str) -> Profile` constructor
   that fills `id` in, and use that instead.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. Paths + `pub` are the whole access-control story; next lesson, `use` lets you stop
typing long paths.)*

## 6. What surprised you?

A sentence or two: did "`pub mod` doesn't publish its contents" catch you out? Did the
one-way privacy (a child sees its parent's privates, never the reverse) feel natural — and
does the absolute-path preference make sense yet? Tell me, and I'll tune L23 (`use`) to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §7.3: absolute-vs-relative paths (Listing 7-3),
  `super` (Listing 7-8), exposing paths with `pub` and the two-strike `pub mod`/`pub fn`
  walkthrough (Listings 7-5 to 7-7), and `pub` on structs vs enums (the `Breakfast`/`Appetizer`
  examples, Listings 7-9/7-10).
- **CR** — *Comprehensive Rust* (Google), §27.3–27.5: "modules are a privacy boundary",
  `pub`/`pub(crate)`, and the marquee "privacy is module-based, not type-based" framing; the
  `self`/`super`/`crate` resolution rule in one block.
- **BLOG** — absent for this slice (out of scope; sourced from BOOK/CR).
- Every snippet compiled (and the `main`-bearing ones run) on **rustc 1.95.0**, edition 2024;
  `E0603` and `E0616` captured live. Next: **Lesson 23** — the `use` keyword (shortcuts for
  these paths, re-exporting, and `as`).

---

<!-- lesson-nav -->
[← Lesson 21 — Packages, Crates & Modules](21-packages-crates-modules.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 23 — The `use` Keyword →](23-the-use-keyword.md)
