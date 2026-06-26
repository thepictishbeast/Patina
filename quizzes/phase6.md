# Phase 6 Quiz — Organizing Code (modules, paths, `use`)

A self-check for the Phase-6 **Organizing** lessons (Lessons 21–23: packages & crates,
modules & the module tree, modules-in-files, paths, `pub`/privacy, and the `use` keyword).
Same rule as before: **predict each answer before** you look at the **Answers** section.
Don't run the code first; predict, then verify. Fourteen questions.

> Tip: cover the Answers section until you've committed to an answer for every question.
> (Generics, traits, and lifetimes are a *later* part of Phase 6 — not covered here.)

---

## Questions

**Q1 — concept.** A package can hold two kinds of crate. Which **file** is the crate root
of a **binary** crate, and which is the crate root of a **library** crate?

**Q2 — predict the output.**
```rust
mod foo {
    pub fn do_something() { println!("In the foo module"); }
}
mod bar {
    pub fn do_something() { println!("In the bar module"); }
}
fn main() {
    foo::do_something();
    bar::do_something();
}
```

**Q3 — does this compile? If not, what's the error code?** (There is no `src/garden.rs`.)
```rust
mod garden;

fn main() {
    println!("hi");
}
```

**Q4 — concept.** In this tree, name the relationship: is `hosting` a **sibling**,
**child**, or **parent** of `serving`? And of `front_of_house`?
```rust
mod front_of_house {
    mod hosting {}
    mod serving {}
}
```

**Q5 — predict the output.**
```rust
mod greetings {
    pub fn hello() { println!("hello"); }
}
fn main() {
    crate::greetings::hello();   // absolute path
    greetings::hello();          // relative path
}
```

**Q6 — does this compile? If not, what's the error code?**
```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
}
```

**Q7 — predict the output.**
```rust
fn at_root() { println!("at the root"); }
mod child {
    pub fn call_up() { super::at_root(); }
}
fn main() { child::call_up(); }
```

**Q8 — does this compile? If not, what's the error code?**
```rust
mod back_of_house {
    pub struct Breakfast { pub toast: String, seasonal_fruit: String }
    impl Breakfast {
        pub fn summer(t: &str) -> Breakfast {
            Breakfast { toast: String::from(t), seasonal_fruit: String::from("peaches") }
        }
    }
}
fn main() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.seasonal_fruit = String::from("blueberries");
}
```

**Q9 — does this compile?** (Note: no `pub` on the individual variants.)
```rust
mod m {
    pub enum Color { Red, Blue }
}
fn main() {
    let _c = m::Color::Red;
    let _d = m::Color::Blue;
}
```

**Q10 — does this compile? If not, what's the error code?**
```rust
mod front_of_house {
    pub mod hosting { pub fn add_to_waitlist() {} }
}
use crate::front_of_house::hosting;
mod customer {
    pub fn eat() { hosting::add_to_waitlist(); }
}
```

**Q11 — predict the output, and explain.**
```rust
use std::fmt::Result;
use std::io::Result as IoResult;
fn f1() -> Result { Ok(()) }
fn f2() -> IoResult<()> { Ok(()) }
fn main() {
    let _ = f1();
    let _ = f2();
    println!("two Results, no clash");
}
```
Why doesn't bringing two things named `Result` into scope cause a conflict here?

**Q12 — concept.** What does **`pub use`** do that a plain `use` does not, and what's it
*for*?

**Q13 — concept.** After `use std::io::{self, Write};`, which **two** names are in scope?

**Q14 — fill in the blanks (concept).** (a) Putting `pub` on a module `____` make its
contents public. (b) A `use` shortcut only applies inside its `____`. (c) The glob import
`use some_module::*;` should be used `____`.

---

## Answers

*(Verified on rustc 1.95.0, edition 2024.)*

**A1 — `src/main.rs` is the binary crate root; `src/lib.rs` is the library crate root.**
The crate root is the file the compiler starts from. (Lesson 21.)

**A2 — `In the foo module` / `In the bar module`.** Both modules define `do_something`, but
`foo::do_something` and `bar::do_something` are different paths — the module *is* the
namespace, so there's no clash. (Lesson 21.)

**A3 — No: `error[E0583]`** ("file not found for module `garden`"). `mod garden;` (semicolon,
no body) tells the compiler to *load* the module from a file — `src/garden.rs` or
`src/garden/mod.rs` — and there isn't one. (Lesson 21.)

**A4 — `hosting` and `serving` are siblings** (same parent, `front_of_house`); **`hosting`
is a child of `front_of_house`** (and `front_of_house` is its parent). (Lesson 21.)

**A5 — `hello` / `hello`** (twice). The absolute path (`crate::greetings::hello`) and the
relative path (`greetings::hello`) reach the *same* function. (Lesson 22.)

**A6 — No: `error[E0603]`** ("function `add_to_waitlist` is private"). `pub mod hosting`
makes the *module* reachable, but the function inside still needs its own `pub` — **`pub` on
a module doesn't publish its contents.** (Lesson 22.)

**A7 — `at the root`.** `at_root` is private at the crate root, but `child` is its
descendant, and a child can call its ancestors' private items; `super::` reaches up to the
parent. (Lesson 22.)

**A8 — No: `error[E0616]`** ("field `seasonal_fruit` of struct `Breakfast` is private"). A
`pub struct` does **not** make its fields public — `seasonal_fruit` has no `pub`, so outside
code can't set it. (That's why the `summer` constructor exists.) (Lesson 22.)

**A9 — Yes, it compiles.** `pub enum` makes **all** of its variants public at once — you
don't (and can't) put `pub` on individual variants. (Lesson 22.)

**A10 — No: `error[E0433]`** ("cannot find module or crate `hosting` in this scope"). A `use`
is **scope-local**: the one at the crate root doesn't reach into the `customer` module (which
would need its own `use`, or `super::hosting`). The root `use` also warns that it's now an
unused import. (Lesson 23.)

**A11 — `two Results, no clash`.** Both `std::fmt::Result` and `std::io::Result` are brought
in, but `as` **renamed** the second to `IoResult`, so the two names are distinct. (Without
the `as`, the two `Result`s *would* collide.) (Lesson 23.)

**A12 — `pub use` re-exports** a name: it brings the item into scope **and** makes that
shorter path public to outside code, whereas a plain `use` is private to your module. It
lets you organize code one way internally but expose a **simpler public API**. (Lesson 23.)

**A13 — `io` and `io::Write`.** The `self` in `{self, Write}` brings in the module `io`
itself, alongside the `Write` item from inside it. (Lesson 23.)

**A14 — (a) does *not*** (a public module still has private contents until each item is
`pub`); **(b) scope** (the block/module where it's written); **(c) sparingly** (the glob
hides where names come from). (Lessons 22–23.)

---

*How did you do?* Anything you missed points at the lesson to reread. You can now split a
program into a package of crates and modules, reach items by path, control what's `pub`, and
tidy it all with `use` — the whole "organizing" toolkit. Next in Phase 6: generics and
traits (a later slice).

— *Sources:* questions written for this corpus from Lessons 21–23 (BOOK Ch.7, CR Ch.27);
every code snippet compiled (and the `main`-bearing ones run) on **rustc 1.95.0**, edition 2024.
