# Lesson 8 — Comments & printing

## 1. Why it exists

Two everyday tools you'll reach for in every program: **comments** (notes for
humans that the compiler ignores) and **printing** (how your program talks back to
you). You've used `println!` since Lesson 1 — now let's see what it can actually do.
This closes out the foundations.

## 2. The idea

**Comments** — the compiler skips them entirely:

- `// ...` — a line comment, runs to the end of the line.
- `/* ... */` — a block comment, can span several lines.
- `/// ...` — a *doc* comment (documents the thing just below it). You'll use these
  properly much later; just recognise them for now.

**Printing:**

- `println!("...")` prints one line. The `!` marks it a **macro** (not a plain
  function — that's what lets it do flexible formatting). For now: `!` means macro,
  go with it.
- `{name}` drops a variable straight into the text. A bare `{}` is a **placeholder**
  filled by the arguments listed after the string, left to right.
- `format!` works exactly like `println!` but instead of printing, it **returns**
  the finished text — an owned `String` value you can keep in a binding. (Text
  types get their full lesson in Lesson 12; for now "a `String`" = "text you own.")

## 3. A tiny example to read

```rust
fn main() {
    // a line comment — ignored by the compiler
    let name = "Rust";
    let version = 2024;
    println!("Hello, {name}!");
    println!("{} edition {}", name, version);
    let sentence = format!("{name} {version}");
    println!("{sentence}");
}
```

**Predict the three printed lines, then check:**

```
Hello, Rust!
Rust edition 2024
Rust 2024
```

Line 1 uses an inline `{name}`; line 2 uses positional `{}` placeholders filled by
`name` then `version`; line 3 was *built* by `format!` into `sentence` and then
printed.

## 4. Common pitfalls / real compiler errors — `{}` vs `{:?}`

`{}` works for things with one obvious text form (numbers, text, bools). Point it
at a whole *array* — `[1, 2, 3]` is a fixed list of values, arriving properly in
Lesson 13 — and:

```rust
fn main() {
    let nums = [1, 2, 3];
    println!("{nums}");
}
```

**Before you scroll — will this compile?**

No. Real output from `rustc` (1.95.0), unedited:

```
error[E0277]: `[{integer}; 3]` doesn't implement `std::fmt::Display`
 --> b.rs:3:15
  |
3 |     println!("{nums}");
  |               ^^^^^^ `[{integer}; 3]` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for `[{integer}; 3]`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

A **new** code, `E0277` — and it's not about a *wrong* type, it's about a *missing
ability*: an array has no single obvious way to show itself with `{}` ("doesn't
implement `Display`"). The note hands you the fix: use `{:?}`, the **debug** format
for programmer-facing output. `println!("{nums:?}")` prints `[1, 2, 3]`. Rule of
thumb: **`{}` for simple values, `{:?}` for collections and things you're
inspecting.**

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine,
`cargo new printing` works too.)* Predict before each run:

1. Print one line that mixes an inline `{var}` and a positional `{}`. Predict the
   exact line first.
2. Use `format!` to build a `String` out of two of your variables, then print it.
   Predict the line first.
3. Make an array of three numbers and print it with plain `{}`. Predict the
   **error code** before running. Then switch to `{:?}` and predict the output.

*(Every line is yours. Predictions are your answer key.)*

## 6. What surprised you?

Did the `{}`-vs-`{:?}` split make sense? Anything about macros (`!`) or `format!`
you want unpacked? Tell me.

> **Phase 1 complete.** You've now got the foundations: naming values (`let`,
> `mut`, shadowing, `const`), the kinds of values (numbers, `bool`, `char`),
> expressions and the semicolon, functions, and output. **Next phase is your stated
> gap — control flow** (`if`, `loop`, `while`, `for`, `match`), starting from the
> very basics again.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.4 "Comments" and §3.1/§1.2
  (printing & `println!`); `{:?}`/`Display` vs `Debug` from §5.2.
- **CR** — *Comprehensive Rust* (Google), "Formatting." Cited for contrast.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Printing." Cited for contrast.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 7 — Functions](07-functions.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 9 — if / else if / else: making the program choose →](09-if-else.md)
