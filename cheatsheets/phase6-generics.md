# Phase 6 Cheatsheet — Generics: `<T>`, Traits & Lifetimes

Quick reference (pairs with the Phase-6 Generics lessons — L24 generics `<T>` · L25 traits ·
L26 lifetimes). The arc: **generics** write one definition for many types, **traits** name
the behavior a type must have (and let you demand it), **lifetimes** let a borrowed value
cross a function/struct boundary safely. Verified on rustc 1.95.0, edition 2024. *(This is the
"later slice" the Organizing cheatsheet pointed to.)*

## Generics — `<T>`, write once for many types
- A **type parameter** (`T`, by convention a single capital, declared in `<…>`) is a hole for a real type you fill in later — once per call. You've used it already: `Vec<T>`, `Option<T>`.
- **Four placements**, same idea each time — *declare* the parameter, then *use* it as a type:
  - **Function** — `fn largest<T>(list: &[T]) -> &T` (declare `<T>` after the name).
  - **Struct** — `struct Point<T> { x: T, y: T }`.
  - **Enum** — `enum MyOption<T> { Some(T), None }` (this *is* how `Option<T>` is built).
  - **Method** — `impl<T> Point<T> { … }` — `<T>` appears **twice**: once to declare, once in the type's name.
- **The body must be valid for *every* `T`.** You can move/store/return `T`, but you can't `>`/`==`/`+` a bare `T` — not all types support that. To do more, add a **trait bound** (next section).
- **Monomorphization:** the compiler stamps out a specialized copy per concrete type used, so `T` is gone at runtime. **Runtime cost: none** (as fast as hand-written copies). **Trade-off: bigger binary + slower compile** (matters for embedded / WebAssembly).
- *Display gotcha:* a float like `4.0` prints as **`4`** under `{}` — the trailing `.0` is dropped.

## Traits — naming shared behavior
- `trait Summary { … }` declares a set of method signatures a type promises to provide. `impl Summary for SocialPost { … }` keeps the promise; then you can call `.summarize()` on its values.
- **Required vs default method:** a signature ending in `;` (no body) is **required** — every implementer must write it. A signature **with** a body is a **default** — implementers get it free and may override it.
- **Three ways to demand a trait** (same meaning):
  - `fn notify(item: &impl Summary)` — `impl Trait` parameter: "anything that is `Summary`."
  - `fn notify<T: Summary>(item: &T)` — the **trait bound** (`impl Trait` is shorthand for this).
  - `fn notify<T>(item: &T) where T: Summary { … }` — a **`where` clause** (moves long/multiple bounds off the signature line). `<T: Summary>` and `where T: Summary` compile to the same thing.
- **`impl Trait` return** — `fn make() -> impl Summary` returns "some type that is `Summary`" without the caller naming which.
- **Orphan rule, one line:** you may write `impl Trait for Type` only if the trait **or** the type is defined in your own crate.

## Lifetimes — `'a`, how long a reference stays valid
- A **lifetime** is the span a reference is valid for; every reference has one (usually inferred). A **lifetime annotation** `'a` (an apostrophe + a name, "tick-a") *names* one so you can state how two references relate.
- **Key idea — an annotation changes nothing.** It can't make a value live longer or a reference die sooner; it only **describes a relationship the compiler then checks**. A promise *about* lifetimes, not a lever *on* them.
- **Syntax mirrors generics:** `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` — declare `<'a>`, attach it. Reads: "the result lives as long as the **shorter** of `x` and `y`."
- **Lifetime elision** — three rules; if they cover every reference you write nothing:
  1. Each reference **parameter** gets its own lifetime.
  2. **Exactly one** input lifetime → it's given to **all outputs**. (Why `fn first_word(s: &str) -> &str` needs no annotation.)
  3. A parameter is **`&self`/`&mut self`** (a method) → the output gets `self`'s lifetime.
  - `longest` defeats rule 2 (*two* inputs), so you must write `'a`; `first_word` (one input) doesn't.
- **A struct that holds a reference needs a lifetime:** `struct Excerpt<'a> { part: &'a str }` — the `Excerpt` may not outlive the `&str` it borrows. (Or just **own** it with `String` — simpler when you want to keep the data freely.)

## Error codes — what the compiler says, and the fix
- **`E0425`** "cannot find type `T` in this scope" — used `T` in `impl Point<T>` without declaring it. Fix: **`impl<T> Point<T>`**.
- **`E0369`** "binary operation `>` cannot be applied to type `&T`" — used `>`/`==` on a bare generic `T`. Fix: add a bound, e.g. `<T: PartialOrd>`.
- **`E0277`** "the trait bound `…: Summary` is not satisfied" — passed a type that doesn't implement the required trait (`notify(&5)`). Fix: pass a type that implements it.
- **`E0046`** "not all trait items implemented, missing: `…`" — an `impl` block left out a **required** method (an empty `impl Summary for Photo {}`). Defaults are free; required ones aren't. Fix: add the method.
- **`E0597`** "`x` does not live long enough" — a reference outlived the data it points to (borrow stored in an outer binding from an inner scope). The borrow checker's core dangling-reference catch.
- **`E0106`** "missing lifetime specifier" — a returned/stored reference whose source is ambiguous: `fn longest(x: &str, y: &str) -> &str` (two inputs) or `struct Holder { text: &str }` (a borrowed field). Fix: add `'a`.
- **`E0515`** "cannot return value referencing local variable `…`" — returned a reference to data **owned by the function** (it's dropped at the closing brace). No annotation fixes this; return the owned value (`String`) instead.

— *Sources:* BOOK Ch.10.1 (generics) · Ch.10.2 (traits) · Ch.10.3 (lifetimes); CR Ch.13
(generics/traits) & §24 (lifetimes). The dropped cross-language framings ("like an interface,"
"like C++ templates") stay dropped. Snippets verified on rustc 1.95.0, edition 2024.
