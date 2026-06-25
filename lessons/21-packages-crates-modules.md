# Lesson 21 — Packages, Crates & Modules

*(Phase 6 — Organizing & generics, part 1. Every program so far has been one file
with one growing `main`. Real programs split into named groups you can navigate, hide
the internals of, and reuse. This lesson is the **map** of Rust's organizing tools; the
next two cover **finding** things across modules (paths, L22) and **hiding** things
(`pub`, L22).)*

## 1. Why it exists

As a program grows, "one big file" stops working — you lose track of what's where, and
everything can touch everything. Rust gives you a nested vocabulary for structure:

- a **package** is what `cargo new` makes — a project bundle;
- a **crate** is one unit the compiler builds at a time;
- a **module** is a named group *inside* a crate, for organization and privacy.

You don't need this for a 20-line program. You need it the moment a program has more than
one idea in it — which is soon. Knowing the words now means the compiler's messages (and
every Rust project you read) make sense later.

> **How the sources frame it:** the **BOOK** Ch.7 is the backbone — it's the only source
> with a real `cargo new` filesystem walk, the module tree drawn out, and the
> file-loading rule. **CR** Ch.27 owns the crispest one-screen module artifact and the
> clean three-level hierarchy. (No metaphor is invented — the **filesystem** comparison is
> the BOOK's own, and it's a good one.)

## 2. The idea

**The three words, smallest-out.**

- A **module** (`mod`) is a named group of items (functions, types, other modules) inside
  a crate. It does two jobs: **organization** (a name to group things under) and
  **privacy** (its contents are hidden unless you say otherwise).
- A **crate** is the smallest amount of code the compiler looks at as a unit — really a
  *tree of modules*. There are two kinds:
  - a **binary crate** — has a `fn main`, compiles to a program you can run. Its
    **crate root** (the file the compiler starts from) is `src/main.rs`.
  - a **library crate** — no `main`, just code meant to be shared/reused. Its crate root
    is `src/lib.rs`.
- A **package** is what `cargo new` produces: a `Cargo.toml` plus one or more crates. The
  rule: a package has **at least one** crate, **at most one** library crate, and any number
  of binary crates (extra binaries live in `src/bin/`). Cargo itself is a package — a
  binary crate (the `cargo` command) plus a library crate it depends on.

**Modules and the module tree.** You write a module inline with `mod name { … }`, and
modules **nest**:

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

That nesting forms a **tree**, rooted at an implicit module named `crate`:

```
crate
 └── front_of_house
      ├── hosting
      │    └── add_to_waitlist
      └── serving
           └── take_order
```

The family words are exactly the tree's: `hosting` and `serving` are **siblings** (same
parent); `hosting` is a **child** of `front_of_house`, which is its **parent**. (BOOK calls
this "a very apt comparison" to a **directory tree** — and that's the running picture for
the rest of the phase.) One thing to flag now and explain fully next lesson: a module's
contents are **private by default**, so to *call* a module's function from outside it you
mark the function `pub` — you'll see that in the examples below.

**Modules in files.** You don't keep everything inline forever. When a module gets big,
move it to its own file. A `mod` declaration **with a semicolon and no body** tells the
compiler to load the body from another file:

```
mod garden;   // load this module's body from a file
```

The compiler looks for it at `src/garden.rs` (idiomatic today) **or** `src/garden/mod.rs`
(the older style — both work, don't mix them for one module). A submodule follows the
directory: `garden::vegetables` lives at `src/garden/vegetables.rs`. The keystone rule:

> **`mod` is not an `include`.** `mod garden;` *declares* the module — it loads that file
> into the tree **once**. You don't re-`mod` it elsewhere; everywhere else *refers* to it.

## 3. Tiny examples to read

**Two modules, same function name — no clash** (the module *is* the namespace):

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

Both modules define `do_something`, yet there's no conflict — `foo::do_something` and
`bar::do_something` are different paths. The `::` reaches into a module; `pub` is what lets
`main` (outside the module) call in. That's the whole job of a module: a namespace with a
privacy fence.

**A nested tree to read** (this one is library-shaped — no `main`, just structure; it's the
`front_of_house` snippet above). Read it as the tree picture, not as something that *does*
anything yet — the bodies are empty on purpose, "to concentrate on organization rather than
implementation" (BOOK). Building it as a library crate compiles cleanly (you'll just get
`dead_code` warnings, since nothing calls these yet — expected for a bare scaffold).

**The same idea, split into files** (a structural diagram — this is what `mod garden;` sets
up):

```
src/
 ├── lib.rs              // contains:  mod garden;
 ├── garden.rs           // the `garden` module's body;  contains:  mod vegetables;
 └── garden/
      └── vegetables.rs  // the `garden::vegetables` module's body
```

The directory layout *mirrors* the module tree — that's the filesystem metaphor paying off.

## 4. Common pitfalls / real compiler errors

**Declaring a file-module but not creating the file — `E0583`.** Write `mod garden;` with
no `src/garden.rs` next to it and the compiler stops:

```rust
mod garden;

fn main() {
    println!("hi");
}
```

```
error[E0583]: file not found for module `garden`
 --> main.rs:1:1
  |
1 | mod garden;
  | ^^^^^^^^^^^
  |
  = help: to create the module `garden`, create file "garden.rs" or "garden/mod.rs"
  = note: if there is a `mod garden` elsewhere in the crate already, import it with `use crate::...` instead
```

This is the best proof that **`mod` loads a file**: the compiler even tells you the two
filenames it will accept. (And the note restates the keystone rule — if the module already
exists, you *refer* to it with a path, you don't `mod` it a second time.)

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new modules_practice`. **Predict on paper before each run.**

1. **Two modules, one call each.** Write modules `english` and `spanish`, each with a
   `pub fn greet()` that prints a different greeting. Call both from `main`. **Predict** the
   two output lines, then run.

2. **Nest a module.** Put a module `inner` *inside* a module `outer`, give `inner` a
   `pub fn ping()` that prints something, and call it from `main` (you'll need `pub` on
   `inner` too — **predict the path** you write before the `::ping()`). Then draw the module
   tree (`crate` → … → `ping`) and label which items are siblings and which are
   parent/child.

3. **Make the compiler ask for a file.** In a `cargo` project, add `mod notes;` to
   `src/main.rs` but *don't* create the file yet. **Predict the error code**, then run
   `cargo build` to check. Now create `src/notes.rs` with a `pub fn show()` inside, call it,
   and watch the error go away.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. Once you can split a program into modules and files, you can organize any size of
project — the next lesson is how to *reach* those items by path and control what's `pub`.)*

## 6. What surprised you?

A sentence or two: did "a crate is a *tree of modules*" change how you picture a Rust
project? Did the filesystem comparison (`crate` is the root directory, modules are
folders) help — or did "`mod` is not `include`" trip you up? Tell me, and I'll tune L22
(paths & `pub`) to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, Ch.7: §7.1 (packages & crates — the
  `cargo new` walk, binary-vs-library crate roots, the cardinality rule), §7.2 (defining
  modules; the module tree and its `crate`/parent/child/sibling vocabulary), §7.5 (separating
  modules into files; "`mod` is not an `include`"; the `name.rs` / `name/mod.rs` file rule).
- **CR** — *Comprehensive Rust* (Google), Ch.27 "Modules": the `foo`/`bar` two-module
  artifact (run output above) and the package → crate → module hierarchy; "`impl` namespaces
  functions to a type; `mod` namespaces types and functions" ties straight back to Lesson 18.
- **BLOG** — absent for this slice (out of scope; the concepts are sourced from BOOK/CR).
- Every snippet compiled (and the `main`-bearing one run) on **rustc 1.95.0**, edition 2024;
  `E0583` captured live. `pub`, paths, and privacy get their full treatment in **Lesson 22**.

---

<!-- lesson-nav -->
[← Lesson 20 — Error Handling: `Result`, `?`, `panic!`](20-error-handling.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 22 — Paths & Visibility (`pub`) →](22-paths-and-visibility.md)
