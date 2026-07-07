# Lesson 21b — Modules & the module tree

*(Phase 6 — Organizing & generics, part 2. A crate (Lesson 21) is one build
unit — but inside it, everything can still touch everything. A **module** is
a named group inside a crate, and nested modules form a TREE: the picture
every Rust project, and half the compiler's messages, are drawn on.)*

## 1. Why it exists

One file with thirty functions is a junk drawer. You want named groups —
"these five functions are about parsing, those three are about printing" — so
you can navigate by name and, later, hide internals. A **module** (`mod`) is
that named group. It does two jobs: **organization** (a name to group things
under) and **privacy** (its contents are hidden unless you say otherwise).

You don't need this for a 20-line program. You need it the moment a program
has more than one idea in it — which is soon.

## 2. The idea

**You write a module inline with `mod name { … }`, and modules nest:**

```
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
    mod serving {
        fn take_order() {}
    }
}
```

**That nesting forms a tree**, rooted at an implicit module named `crate`:

```
crate
 └── front_of_house
      ├── hosting
      │    └── add_to_waitlist
      └── serving
           └── take_order
```

The family words are exactly the tree's: `hosting` and `serving` are
**siblings** (same parent); `hosting` is a **child** of `front_of_house`,
which is its **parent**. The BOOK calls this "a very apt comparison" to a
**directory tree** — and that's the running picture for the rest of the
phase.

**Reaching in: `::`.** From outside a module, you name its items by path —
`foo::do_something()` means "the `do_something` inside `foo`." The module is
a **namespace**: two modules can each have a `do_something` with no clash,
because the paths differ.

**One thing to copy now, understand next lesson:** a module's contents are
**private by default**. To *call* a module's function from outside it, the
function is marked **`pub`** — you'll see it in every example below. What
`pub` really does, and all the ways to aim it, is **Lesson 22's whole
topic**; until then, write it where shown and move on.

## 3. Tiny examples to read

**Two modules, same function name — no clash** (the module *is* the
namespace):

```rust
mod foo {
    pub fn do_something() {
        println!("In the foo module");
    }
}

mod bar {
    pub fn do_something() {
        println!("In the bar module");
    }
}

fn main() {
    foo::do_something();
    bar::do_something();
}
```

```
In the foo module
In the bar module
```

Both modules define `do_something`, yet there's no conflict —
`foo::do_something` and `bar::do_something` are different paths. That's the
whole job of a module: a namespace with a privacy fence.

**A nested tree to read** (library-shaped — no `main`, just structure; it's
the `front_of_house` snippet above). Read it as the tree picture, not as
something that *does* anything yet — the bodies are empty on purpose, "to
concentrate on organization rather than implementation" (BOOK). Building it
as a library crate compiles cleanly (you'll just get `dead_code` warnings,
since nothing calls these yet — expected for a bare scaffold).

## 4. Common pitfalls / real compiler errors

**Calling a module's function without its path — `E0425`.** The namespace is
real: names do *not* leak out of their module. Define `do_something` inside
`foo`, then call it bare:

```rust
mod foo {
    pub fn do_something() {
        println!("In the foo module");
    }
}

fn main() {
    do_something();
}
```

**Before you scroll — the function exists, it's even `pub`. What's missing?**

```
error[E0425]: cannot find function `do_something` in this scope
 --> main.rs:8:5
  |
8 |     do_something();
  |     ^^^^^^^^^^^^ not found in this scope
  |
help: consider importing this function
  |
1 + use crate::foo::do_something;
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

"Not found in this scope" — `main`'s scope doesn't contain `do_something`;
`foo`'s does. The immediate fix is the path: `foo::do_something();`. And look
at the compiler's own suggestion: `use crate::foo::do_something` — a path
starting at the tree's root, brought into scope with `use`. That one help
line is a preview of the next two lessons (paths in 22, `use` in 23).

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new modules_practice` works too.)* **Predict on paper
before each run.**

1. **Two modules, one call each.** Write modules `english` and `spanish`,
   each with a `pub fn greet()` that prints a different greeting. Call both
   from `main`. **Predict** the two output lines, then run.
2. **Nest a module.** Put a module `inner` *inside* a module `outer`, give
   `inner` a `pub fn ping()` that prints something, and call it from `main`
   (you'll need `pub` on `inner` too — **predict the path** you write before
   the `::ping()`). Then draw the module tree (`crate` → … → `ping`) and
   label which items are siblings and which are parent/child.
3. **Drop the path.** From task 1, change one call to a bare `greet()`.
   **Predict the error code** — and read the compiler's `help:` line
   carefully: which TWO upcoming lessons is it quietly previewing?

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next, Lesson 21c: when a module outgrows the file it lives
in — `mod name;` and the files that mirror the tree.)*

## 6. What surprised you?

A sentence or two: did "a crate is a *tree of modules*" change how you
picture a Rust project? Did the filesystem comparison (`crate` is the root
directory, modules are folders) help? Tell me, and I'll tune Lesson 21c to
match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§7.2** "Defining Modules to
  Control Scope and Privacy": the module tree and its
  `crate`/parent/child/sibling vocabulary, the directory-tree comparison, and
  the `front_of_house` scaffold (reproduced here).
- **CR** — *Comprehensive Rust* (Google), Ch.27 "Modules": the `foo`/`bar`
  two-module artifact (run output above); "`impl` namespaces functions to a
  type; `mod` namespaces types and functions" ties straight back to
  Lesson 18b.
- Every snippet compiled (and the `main`-bearing ones run) on **rustc
  1.95.0**, edition 2024; `E0425` captured live (temp paths normalized to
  `main.rs`). `pub`, paths, and privacy get their full treatment in
  **Lesson 22**.

---

<!-- lesson-nav -->
[← Lesson 21 — Packages & crates](21-packages-and-crates.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 21c — Splitting modules into files →](21c-modules-in-files.md)
