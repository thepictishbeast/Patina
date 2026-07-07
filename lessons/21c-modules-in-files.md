# Lesson 21c — Splitting modules into files

*(Phase 6 — Organizing & generics, part 3. Lesson 21b's modules were inline —
`mod name { … }` with the body right there. Real modules outgrow that. One
semicolon changes everything: `mod name;` says "the body lives in a FILE" —
and the directory layout starts mirroring the module tree.)*

## 1. Why it exists

Inline modules keep everything in one file — which is exactly the junk-drawer
problem modules were meant to solve, one level up. When a module gets big,
you move its body to its own file, and the project's *folders* start looking
like the module *tree*: the filesystem comparison from Lesson 21b stops being
a metaphor and becomes the literal layout.

## 2. The idea

**A `mod` declaration with a semicolon and no body tells the compiler to load
the body from another file:**

```
mod garden;   // load this module's body from a file
```

The compiler looks for it at `src/garden.rs`. A **submodule** follows the
directory: `garden::vegetables` lives at `src/garden/vegetables.rs`. (An
older style, `src/garden/mod.rs`, also works — you'll meet it in existing
code; the trade-offs are **Book §7.5**. Pick the modern one for your own
projects.)

The keystone rule:

> **`mod` is not an `include`.** `mod garden;` *declares* the module — it
> loads that file into the tree **once**, at that spot. You don't re-`mod` it
> elsewhere; everywhere else *refers* to it by path.

## 3. A structural diagram to read

The same tree from Lesson 21b, split into files — this is what `mod garden;`
sets up:

```
src/
 ├── lib.rs              // contains:  mod garden;
 ├── garden.rs           // the `garden` module's body;  contains:  mod vegetables;
 └── garden/
      └── vegetables.rs  // the `garden::vegetables` module's body
```

The directory layout *mirrors* the module tree — that's the filesystem
comparison paying off. Note what did **not** change: paths. Code elsewhere
still says `garden::vegetables::…` exactly as if everything were inline. The
files are storage; the tree is the truth.

## 4. Common pitfalls / real compiler errors

**Declaring a file-module but not creating the file — `E0583`.** Write
`mod garden;` with no `src/garden.rs` next to it and the compiler stops:

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

This is the best proof that **`mod` loads a file**: the compiler even tells
you the two filenames it will accept. And the note restates the keystone
rule — if the module already exists, you *refer* to it with a path, you don't
`mod` it a second time.

## 5. Predict-then-run practice (your turn — write this yourself)

This lesson needs real files, so it's a **project** exercise: `cargo new
modules_practice` on your own machine (or in Termux). **Predict on paper
before each build.**

1. **Make the compiler ask for a file.** Add `mod notes;` to `src/main.rs`
   but *don't* create the file yet. **Predict the error code — and the two
   filenames the help line will offer.** Run `cargo build` to check.
2. **Give it the file.** Create `src/notes.rs` with a `pub fn show()` that
   prints something; call `notes::show()` from `main`. **Predict** the
   output, and answer: did the *path* you call change because the module
   moved to a file?
3. **One level deeper.** Inside `notes.rs`, add `mod drafts;` and create
   `src/notes/drafts.rs` with a `pub fn count()`. Call it from `main` —
   **predict the full path** you'll need (three segments). Then draw the
   tree and the directory side by side: they should be the same shape.

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. You can now split a program across files without losing
the tree. Next: Lesson 22 — reaching along that tree with paths, and what
`pub` has really been doing.)*

## 6. What surprised you?

A sentence or two: did "`mod` is not an `include`" trip you up, or did the
E0583 help line settle it? Did task 3's side-by-side drawing (tree vs
directory) come out identical? Tell me, and I'll tune Lesson 22 to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§7.5** "Separating Modules
  into Different Files": the `mod name;` file rule, `name.rs` vs
  `name/mod.rs` (this lesson points at the style discussion rather than
  reciting it), and "`mod` is not an `include`."
- **CR** — *Comprehensive Rust* (Google), Ch.27: the filesystem-hierarchy
  slide.
- `E0583` captured live on **rustc 1.95.0**, edition 2024 (temp paths
  normalized to `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 21b — Modules & the module tree](21b-modules.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 22 — Paths: naming items across modules →](22-paths.md)
