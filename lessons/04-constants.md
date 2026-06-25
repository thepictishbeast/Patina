# Lesson 4 — Constants (`const`)

## 1. Why it exists

You now have three ways a name can behave: locked (`let`), changeable (`let mut`),
and re-namable (shadowing). `const` covers a fourth need: a value that is **fixed
forever and known before the program even runs** — a true constant like "a week has
7 days" or "max lives is 3." Giving it a name (instead of scattering the number `3`
through your code) means one obvious place to read it, and one place to change it.

## 2. The idea

A constant is declared with `const`, and unlike `let` it has **two required
parts**:

```
const MAX_LIVES: u32 = 3;
```

1. **A name in `SCREAMING_SNAKE_CASE`** — all caps, words joined by underscores.
   This is the convention so constants stand out from ordinary names.
2. **A written-out type** — here `: u32` (an *unsigned* — never-negative — whole
   number). With `const` the type is **not optional**; you always spell it.

A `const` can't be `mut`, and its value must be computable before the program runs
(a literal like `3`, or simple arithmetic on other constants — not something typed
in at the keyboard). You can also declare one **outside** `main`, so the whole file
can see it.

## 3. A tiny example to read

```rust
const MAX_LIVES: u32 = 3;

fn main() {
    println!("You get {MAX_LIVES} lives.");
}
```

**Predict the line:**

```
You get 3 lives.
```

Notice `MAX_LIVES` sits **above** `main`, not inside it — constants often live at
the top of a file so every function can reach them.

## 4. Common pitfalls / real compiler errors

The easy slip: forgetting the required type. `let` lets you skip the type (Rust
infers it); `const` does **not**.

```rust
const MAX_LIVES = 3;

fn main() {
    println!("{MAX_LIVES}");
}
```

**Before you scroll — does this compile?**

No. Real output from `rustc` (1.95.0), unedited:

```
error: missing type for `const` item
 --> b.rs:1:16
  |
1 | const MAX_LIVES = 3;
  |                ^ help: provide a type for the constant: `: i32`

error: aborting due to 1 previous error
```

The compiler is precise: it points just after the name and even drafts the fix
(`: i32`). Add the type — `const MAX_LIVES: u32 = 3;` (or `: i32`) — and it builds.
This is the one rule to carry away: **a `const` always needs its type written out.**

## 5. Predict-then-run practice (your turn — write this yourself)

`cargo new constants`. Predict before each run:

1. Declare a `const` of your own **above** `main` (SCREAMING_SNAKE_CASE, with a
   type — pick `u32` or `i32`). Print it. Predict the line first.
2. Delete the `: type` from your const and run. Before you do, predict the
   compiler's words — what does it say is missing, and what fix does it draft?
3. Put the type back, then add a line that tries to reassign it
   (`YOUR_CONST = something;`). Predict whether that's even allowed before running.
   (Think back to "locked by default" — is a `const` more or less locked than a
   plain `let`?)

*(All yours to type. Predictions are the answer key.)*

## 6. What surprised you?

Did you expect `const` to require the type when `let` doesn't? Did anything about
SCREAMING_SNAKE_CASE or the top-of-file placement feel odd? Tell me — next we leave
the "naming values" group behind and start on the **kinds** of values themselves
(Lesson 5: number types and a famous overflow surprise).

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.1 "Variables and Mutability →
  Constants" (required type, SCREAMING_SNAKE_CASE, compile-time value).
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Constants." Cited for contrast.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 3 — Shadowing](03-shadowing.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 5 — Number types (and a famous overflow surprise) →](05-number-types-and-overflow.md)
