# Lesson 21 — Packages & crates: what you've been building all along

*(Phase 6 — Organizing & generics, part 1. Every program so far has been one
file with one growing `main`. The next three lessons give you Rust's
organizing tools, smallest-words-first: this one names the units you've
already been using — then 21b groups code INSIDE one, and 21c spreads it
across files.)*

## 1. Why it exists

As a program grows, "one big file" stops working — you lose track of what's
where, and everything can touch everything. Before you can organize anything,
you need the vocabulary for what a Rust project *is*. Two words do most of the
work:

- a **crate** is one unit the compiler builds at a time;
- a **package** is what `cargo new` makes — a `Cargo.toml` plus one or more
  crates.

Here's the reveal this lesson turns on: **you've been writing crates since
Lesson 0.** Every Sandbox program you've ever run was a complete crate. The
words are new; the thing isn't.

## 2. The idea

**A crate is the smallest amount of code the compiler looks at as a unit.**
There are two kinds:

- a **binary crate** — has a `fn main`, compiles to a program you can run.
  Its **crate root** (the file the compiler starts from) is `src/main.rs`.
  Every program you've written so far is one of these.
- a **library crate** — no `main`, just code meant to be shared and reused.
  Its crate root is `src/lib.rs`. You *used* dozens of these the moment you
  called `println!` or `Vec` — the standard library is exactly this kind.

**A package bundles crates.** `cargo new hello` produces a `Cargo.toml` (the
package's manifest — name, version, dependencies) and a `src/main.rs` — that
is, one package containing one binary crate. A package always has **at least
one** crate; the finer counting rules (how many of each kind, the `src/bin/`
trick for extra binaries, and the fact that Cargo itself is a package) are
**Book §7.1** when you need them.

**One binding fact ties the two kinds together:** what makes a binary crate
*binary* is `fn main` — the agreed starting point the runner looks for.
Part 4 shows the compiler insisting on it, by name.

## 3. A tiny example to read — the package on disk

What `cargo new hello` actually puts on your disk:

```
hello/
 ├── Cargo.toml     // the package manifest
 └── src/
      └── main.rs   // the binary crate's ROOT — fn main lives here
```

That's a whole package. Two files. When Lesson 21c adds more files, they'll
all hang off this root — the compiler always *starts* at `src/main.rs` and
discovers everything else from there.

And your very first program was already a complete binary crate:

```rust
fn main() {
    println!("Hello, world!");
}
```

```
Hello, world!
```

Nothing new ran here — the point is the *reading*: that file is a crate root,
the `fn main` is what marks the crate as binary, and the Sandbox has been
building and running a fresh crate for you on every ▶ Run since Lesson 0.

## 4. Common pitfalls / real compiler errors

**A binary crate without `main` — `E0601`.** Delete `fn main` (or misspell
it) and the compiler tells you exactly what a binary crate demands — naming
the crate as it does:

```rust
fn greet() {
    println!("hello");
}
```

**Before you scroll — this is valid Rust. Why won't it build as a program?**

```
error[E0601]: `main` function not found in crate `main`
 --> main.rs:3:2
  |
3 | }
  |  ^ consider adding a `main` function to `main.rs`

error: aborting due to 1 previous error
```

Read the words: not "in this file" — **"in crate `main`"**. The compiler
thinks in crates, and a *binary* crate must have a `main` to be runnable.
(Code with no `main` isn't wrong — it's just *library-shaped*; as a library
crate this exact code builds fine. The kind of crate decides the rule.)

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine,
`cargo new` makes this concrete.)* **Predict on paper before each run.**

1. **Name what you have.** Take any program you wrote in an earlier lesson.
   Answer on paper: what kind of crate is it, what marks it as that kind, and
   what would its crate root be called in a cargo project?
2. **Break the contract.** Write a file with only a `greet()` function — no
   `main`. **Predict the error code** and the two places the message will
   name (a crate, and a file). Then add `fn main` calling `greet()` and
   confirm it runs.
3. **On your own machine (or Termux): the package walk.** Run
   `cargo new hello`, then `ls hello` and `ls hello/src`. **Predict** the two
   files you'll find before you look — then open `Cargo.toml` and read the
   manifest. That's a package.

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next, Lesson 21b: naming groups INSIDE a crate — modules,
and the tree they form.)*

## 6. What surprised you?

A sentence or two: did "you've been writing crates since Lesson 0" reframe
anything? Did the compiler saying **"in crate `main`"** make the unit feel
real? Tell me, and I'll tune the modules lesson to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§7.1** "Packages and Crates":
  the `cargo new` walk, binary-vs-library crates and their roots, and the
  package counting rules this lesson points at rather than recites.
- **CR** — *Comprehensive Rust* (Google), Ch.27: the package → crate → module
  hierarchy (modules are Lessons 21b–21c here).
- Compiler output captured live on **rustc 1.95.0**, edition 2024
  (`rustc --edition 2024 FILE.rs`; temp paths normalized to `main.rs`, which
  also names the crate in E0601's message).

---

<!-- lesson-nav -->
[← Lesson 20c — The ? operator](20c-question-mark.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 21b — Modules & the module tree →](21b-modules.md)
