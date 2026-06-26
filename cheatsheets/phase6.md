# Phase 6 Cheatsheet — Organizing Code (modules, paths, `use`)

Quick reference (pairs with the Phase-6 Organizing lessons — L21 packages/crates/modules ·
L22 paths & visibility · L23 `use`). The whole toolkit: a **package** holds **crates**, a
crate is a **tree of modules**, **paths** name items across the tree, **`pub`** controls who
can see them, and **`use`** makes the paths short. Verified on rustc 1.95.0, edition 2024.
*(Generics, traits, and lifetimes are a later Phase-6 slice.)*

## Packages & crates
- A **package** = what `cargo new` makes — a `Cargo.toml` plus crate(s). Rule: **≥1 crate**, **≤1 library crate**, any number of binaries (extra ones in `src/bin/`).
- A **crate** = the unit the compiler builds at once; really a *tree of modules*. **Binary crate** root = `src/main.rs` (has `fn main`, runs); **library crate** root = `src/lib.rs` (shared code). The **crate root** is the file the compiler starts from.

## Modules & the module tree
- `mod name { … }` defines a module — a **namespace** + a **privacy fence**. Modules nest; the whole tree is rooted at the implicit module **`crate`**.
- Family words: **sibling** (same parent) · **child**/**parent**. Picture it as a **filesystem directory tree**.
- "`impl` namespaces functions to a type; `mod` namespaces types and functions" (ties back to L18).

## Modules in files
- `mod garden;` (semicolon, **no body**) **loads** the module body from a file: `src/garden.rs` (idiomatic) **or** `src/garden/mod.rs` (older — don't mix for one module). A submodule `garden::vegetables` → `src/garden/vegetables.rs` (the directory mirrors the tree).
- **`mod` is not `include`** — it declares/loads the module **once**; everywhere else *refers* to it by path. Declare a file-module with no file → **`error[E0583]`** "file not found for module" (the compiler even names both filenames).

## Paths — naming items (like a filesystem)
- **Absolute:** starts at `crate::` (≈ `/`, the root). **Relative:** a sibling name, `self::` (this module, ≈ `.`), or **`super::`** (parent module, ≈ `..`).
- `::`-separated. **Prefer absolute** paths — they survive moving the *calling* code.

## `pub` & privacy-by-default
- Everything is **private to code outside its module** by default. A **child** can use its **ancestors'** private items; a **parent** *cannot* reach into a child. `pub` opens a door; **`pub(crate)`** = public within this crate, not beyond.
- **`pub` on a module ≠ `pub` on its contents** — `pub mod hosting` makes the module reachable, but each function still needs its own `pub`. Calling a private item → **`error[E0603]`** "… is private".
- **`pub struct` keeps fields private** (opt in per field with `pub`), so a struct with a private field needs a **public constructor**. Touch a private field from outside → **`error[E0616]`** "field … is private". **`pub enum` publishes ALL variants** at once (no per-variant `pub`). Privacy is **module-based, not type-based** — same-module code reads private fields freely.

## `use` — shortcuts for paths
- `use crate::front_of_house::hosting;` then call `hosting::add_to_waitlist()`. **Idiom:** bring the **parent module** into scope for *functions*, the **full path** for *types* (`use std::collections::HashMap;`).
- **Scope-local:** a `use` only applies where it's written; a crate-root `use` does **not** reach into a child module → **`error[E0433]`** (the child needs its own `use`, or `super::`).
- **Name clash:** parent-qualify (`fmt::Result` vs `io::Result`) **or** rename with **`as`** (`use std::io::Result as IoResult;`).
- **`pub use`** re-exports — exposes a name at a shorter **public** path (decouple internal layout from your public API).
- **Tidy imports:** nested `use std::{cmp::Ordering, io};` · `self`-in-nested `use std::io::{self, Write};` (brings both `io` and `io::Write`) · glob `use some_module::*;` (**sparingly** — it hides where names come from). Think of `use` as a **symbolic link**.

— *Sources:* BOOK Ch.7 · CR Ch.27. Snippets verified on rustc 1.95.0, edition 2024. The metaphors (the module tree as a **filesystem directory tree**, `use` as a **symbolic link**) are the BOOK's own, carried across Lessons 21–23.
