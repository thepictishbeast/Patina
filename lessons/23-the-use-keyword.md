# Lesson 23 — The `use` Keyword

*(Phase 6 — Organizing & generics, part 7, and the finale of the "Organizing" set. Lesson
22 had you writing full paths like `crate::front_of_house::hosting::add_to_waitlist()`.
Saying that on every call is noise. `use` brings a name into scope **once** so you can use
the short form — think of it as a **symbolic link** in the filesystem: a shortcut to
something that lives elsewhere.)*

## 1. Why it exists

Full paths are precise but tiring to repeat. `use` creates a shortcut for the rest of the
current scope, so your code reads by the *short* name while the path is stated once at the
top. It also gives you the tools to handle two real problems: two items with the **same
name**, and presenting a **tidy public API** that doesn't expose your internal folder
structure.

> **How the sources frame it:** the **BOOK** §7.4 is the backbone — the shortcut, the
> scope-locality trap, the function-vs-type idiom, name clashes with `as`, `pub use`
> re-exporting, and nested/glob imports. **CR** §27.5 reinforces `pub use` and is firmly
> anti-glob. `as` is **BOOK-only** here — CR's modules chapter never shows it. The
> **symbolic-link** picture is the BOOK's own.

## 2. The idea

**`use` brings a path into scope.** Write it once, then use the short name:

```
use crate::front_of_house::hosting;
hosting::add_to_waitlist();   // not the full crate::front_of_house::... every time
```

The **idiom** (BOOK, honestly flagged as just convention): bring the **parent module** into
scope for *functions* — `hosting::add_to_waitlist()` still shows where the function comes
from — but bring the **full path** for *types*: `use std::collections::HashMap;` then
`HashMap::new()`.

**`use` is scope-local.** A `use` only applies inside the block or module where you wrote
it. A `use` at the crate root does **not** reach into a child module — that child needs its
own `use` (or a relative `super::`). This is the one beginners trip on; it's the error in
part 4.

**Name clashes — two fixes.** If two items share a name — `std::fmt::Result` and
`std::io::Result` — either bring the **parent modules** into scope and qualify (`fmt::Result`
vs `io::Result`), or rename one with **`as`**:

```
use std::io::Result as IoResult;   // now `Result` and `IoResult` are distinct
```

**Two more tools to recognize, not master yet.** **`pub use`** re-exports a
name so outside code can reach it through your shorter path — the
public-API-façade trick, in **Book §7.4 Listing 7-17** and CR §27.5 when you
need it. And imports can be tidied: a shared prefix collapses
(`use std::{cmp::Ordering, io};`), and the **glob** `use std::collections::*;`
brings in everything — handy in tests, **used sparingly** elsewhere because it
hides where names came from (**Book §7.4 Listings 7-18 to 7-20** has the full
menu).

## 3. Tiny examples to read

**The shortcut** (library-shaped):

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();   // short, but still says "hosting::"
}
```

**`as` to dodge a name clash** (two different `Result` types in one file):

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result { Ok(()) }
fn function2() -> IoResult<()> { Ok(()) }

fn main() {
    let _ = function1();
    let _ = function2();
    println!("two Results, no clash");
}
```

```
two Results, no clash
```

## 4. Common pitfalls / real compiler errors

**A `use` doesn't reach into a child module — `E0433`.** Here the `use` sits at the crate
root, but a child module `customer` tries to use the short name:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

```
error[E0433]: cannot find module or crate `hosting` in this scope
  --> main.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of unresolved module or unlinked crate `hosting`
   |
   = help: you might be missing a crate named `hosting`
help: consider importing this module through its public re-export
```

And — a second clue — the root `use` now reports `warning: unused import:
crate::front_of_house::hosting`, because nothing in *its* scope uses it. Two signals, one
cause: **the shortcut only exists where it was written.** The fix is to put a `use` *inside*
`customer` (or call it relatively with `super::hosting::add_to_waitlist()`).

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new use_practice` works too.)* **Predict on paper before each run.**

1. **Shorten a path.** Reuse your L22 tree (`mod garden { pub mod flowers { pub fn plant()
   {…} } }`). Add a `use` so you can call `flowers::plant()` instead of the full path.
   **Predict** the `use` line you write, then run.

2. **Make a clash, fix it twice.** In one file bring in *two* things named `Result`
   (`std::fmt::Result` and `std::io::Result`). First resolve it by importing the **parent
   modules** and qualifying; then redo it with **`as`**. **Predict** which names are in scope
   in each version.

3. **The scope trap.** Put a `use` at the crate root, then try to use the short name inside a
   child `mod`. **Predict the error code** *and* what warning the root `use` will get. Run to
   check, then fix it by moving the `use` into the child module.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. That completes the "organizing" toolkit: modules build the tree, paths + `pub` reach
and guard it, `use` makes it pleasant to write. Next is the Phase-6 review over these three
lessons.)*

## 6. What surprised you?

A sentence or two: did the symbolic-link picture make `use` click? Did scope-locality (a
`use` only works where it's written) surprise you — and does the function-vs-type idiom feel
arbitrary or sensible? Tell me, and I'll shape the Phase-6 review around it.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §7.4: the `use` shortcut and scope-locality
  (Listings 7-11/7-12, the `E0433`), the function-vs-type idiom (7-14/7-15), name clashes and
  `as` (7-16); `pub use` (7-17) and nested paths/`self`/glob (7-18 to 7-20) are pointed at
  rather than taught here.
- **CR** — *Comprehensive Rust* (Google), §27.5: `use`, the `pub use` re-export (the `lib.rs`
  façade), and the explicit "glob is **discouraged**" guidance.
- **BLOG** — absent for this slice (out of scope; sourced from BOOK/CR).
- Every snippet compiled (and the `main`-bearing ones run) on **rustc 1.95.0**, edition 2024;
  `E0433` captured live. Next: the **Phase-6 review** (quiz + cheatsheet) over Lessons 21–23.

---

<!-- lesson-nav -->
[← Lesson 22c — pub on structs & enums](22c-pub-structs-enums.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 24 — Generic functions: the `<T>` placeholder →](24-generic-functions.md)
