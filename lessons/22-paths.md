# Lesson 22 — Paths: naming items across modules

*(Phase 6 — Organizing & generics, part 4. Lessons 21b–21c built the module
**tree**. First of the two questions that make it usable: how do you **name**
an item that lives in another module? A *path* — and the filesystem picture
keeps paying off, right down to having a `/` and a `..`.)*

## 1. Why it exists

A module tree is only useful if you can refer to something across it. Rust's
answer is the **path**: `a::b::c` points at an item exactly like a file path
points at a file. You met `::` briefly in Lesson 21b; this lesson gives you
the full addressing system — including how to point *up*.

## 2. The idea

A path is `::`-separated and comes in two flavours:

- **Absolute** — starts at the crate root with the literal `crate::` (think
  `/`, the root): `crate::front_of_house::hosting::add_to_waitlist()`.
- **Relative** — starts from the *current* module: a sibling's name, or
  `self::` (this module, ≈ `.`), or **`super::`** (the parent module, ≈ `..`).

When in doubt, prefer **absolute** paths — they keep working if you move the
*calling* code. (The fuller move-code-vs-move-definition reasoning is
**Book §7.3**.)

One thing to know before the examples: they carry `pub` markers. Why they're
needed — and the trap hiding in them — is **Lesson 22b's whole story**; today,
just read the paths.

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

Both calls hit the same function. The absolute one starts at `crate`; the
relative one starts from the sibling name `front_of_house`.

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

Notice `deliver_order` isn't even `pub`, yet `back_of_house` can call it —
a child module may use its **ancestors'** items. (That one-way rule is
Lesson 22b's opening move.)

## 4. Common pitfalls / real compiler errors

**Forgetting to point up — `E0425`.** Inside a module, a bare name means
"something in *this* module." The parent's items aren't in scope by name:

```rust
fn deliver_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        deliver_order();   // the parent's function — but no super::
    }
}

fn main() {
    back_of_house::fix_incorrect_order();
}
```

**Before you scroll — the function exists one level up. Does the child see
it by name?**

```
error[E0425]: cannot find function `deliver_order` in this scope
 --> main.rs:5:9
  |
5 |         deliver_order();   // the parent's function — but no super::
  |         ^^^^^^^^^^^^^ not found in this scope
  |
help: consider importing this function
  |
4 +     use crate::deliver_order;
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

"Not found in this scope" — names don't drift down into child modules; you
must *point*. Two fixes, one per flavour: `super::deliver_order()` (relative,
one step up) or the compiler's own suggestion `crate::deliver_order` (absolute,
from the root). Either mends it — pick the one that reads best. The matching
exercise below is this wall.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new paths_practice` works too.)* **Predict on
paper before each run.**

1. **One function, two paths.** Build `mod garden { pub mod flowers { pub fn
   plant() { println!("planted"); } } }`. From `main`, call `plant()` **once
   by an absolute path and once by a relative path**. **Predict** both lines
   you'll write and the output, then run.
2. **Point up.** Put a `fn announce()` at the crate root and a module `crew`
   with a `pub fn work()` that calls it **with `super::`**. Call
   `crew::work()` from `main`. **Predict**: does `announce` need `pub`?
   Run and see.
3. **Break the pointer.** Delete the `super::` from task 2. **Predict the
   error code** and the four-word phrase about the name — then compare the
   compiler's suggested fix with yours: absolute or relative?

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next, Lesson 22b: the other half — what is *allowed* to be
named from where, and the two-strike trap inside `pub`.)*

## 6. What surprised you?

A sentence or two: did the filesystem mapping (`crate::` ≈ `/`, `super::` ≈
`..`) make paths feel familiar? Did it surprise you that the child could call
the parent's *private* function — or that a bare name simply isn't found?
Tell me, and I'll tune Lesson 22b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§7.3**: absolute-vs-relative
  paths (Listing 7-3), `super` (Listing 7-8), and the prefer-absolute
  reasoning this lesson points at rather than recites.
- **CR** — *Comprehensive Rust* (Google), §27.3–27.5: the
  `self`/`super`/`crate` resolution rule in one block.
- Every snippet compiled (and the `main`-bearing ones run) on **rustc
  1.95.0**, edition 2024; `E0425` captured live (temp paths normalized to
  `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 21c — Splitting modules into files](21c-modules-in-files.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 22b — Privacy & pub: private by default →](22b-privacy-and-pub.md)
