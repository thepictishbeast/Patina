# Lesson 33 — Refutability: when a pattern can fail

*(Phase 9 — Advanced, part 4. Every `let`, every `for`, every `match` arm has a
**pattern** in it — the little shape that describes data and pulls pieces out.
This lesson names the one rule that governs where each pattern is allowed:
can it **fail** to match? That single question explains why `let Some(x) = …`
is rejected while `let (a, b) = …` is fine — and which tool fixes it.)*

## 1. Why it exists

A pattern isn't just for `match`. Every time you write `let (a, b) = pair;` or
`for (k, v) in map`, the left side is a **pattern** — it **describes the shape
of the data** and pulls pieces out. Lesson 19 used patterns to pick an enum
variant; Lesson 19d gave you `if let` and `let … else`.

But you may have noticed something odd: `let (a, b) = pair;` works, yet the
almost-identical `let Some(x) = opt;` is a compile error. Same keyword, same
shape-on-the-left idea — different verdict. The difference isn't syntax. It's
whether the pattern can **fail**, and whether the position it sits in has
anywhere to go when it does. Rust names this **refutability**, ties it to a
real error (`E0005`), and once you hold the rule, every pattern position in
the language sorts itself out.

## 2. The idea

**A pattern is *irrefutable* if it always matches** — `(a, b)` matches every
pair, a plain `x` matches anything. **It's *refutable* if it can fail** —
`Some(x)` doesn't match `None`; the literal `3` doesn't match `4`.

Now the rule that ties everything together — it's about whether there's an
**else path**:

| Position | Needs | Why |
|---|---|---|
| `let PAT = …;` | **irrefutable** | no else-branch — if it failed, what would run? |
| `for PAT in …` | **irrefutable** | each item must bind; nowhere to send a miss |
| function params | **irrefutable** | same — every call must bind |
| `match` arms | **refutable** OK | other arms catch the misses |
| `if let` / `while let` | **refutable** OK | the `else` / loop-exit is the miss path |

So `let Some(x) = opt;` is **rejected** — `opt` might be `None`, and a plain
`let` has nowhere to go on a miss. `let (a, b) = pair;` is **fine** — a pair is
*always* a pair. You'll see the exact error in part 4. The fix for the
refutable case is `if let` or `let … else` (Lesson 19d) — both *provide* the
missing else path.

> **How the sources frame it:** the **BOOK** Ch.19 §19.2 "Refutability:
> Whether a Pattern Might Fail to Match" is this lesson — it's the only source
> that names the concept and ties it to `E0005`. The pattern *tools* that ride
> on this rule (guards, `@`, nesting) are Lesson 33b.

## 3. A tiny example to read

**Irrefutable destructure — always matches, so `let` is happy.** A tuple is
always a tuple, even nested:

```rust
fn main() {
    let (a, b) = (1, 2);              // tuple — always matches
    let ((c, d), e) = ((10, 20), 30); // nested tuple
    println!("{a} {b} {c} {d} {e}");
}
```

```
1 2 10 20 30
```

No `else` anywhere — none is needed, because these patterns *cannot* fail.
Hold that thought against part 4.

## 4. Common pitfalls / real compiler errors

**Refutable pattern in a plain `let` — `E0005`.** `Some(x)` can fail (the value
might be `None`), and a `let` has no else-branch to take when it does:

```rust
fn main() {
    let opt: Option<i32> = Some(5);
    let Some(x) = opt;
    println!("{x}");
}
```

```
error[E0005]: refutable pattern in local binding
 --> main.rs:3:9
  |
3 |     let Some(x) = opt;
  |         ^^^^^^^ pattern `None` not covered
  |
  = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
  = note: for more information, visit https://doc.rust-lang.org/book/ch19-02-refutability.html
  = note: the matched value is of type `Option<i32>`
help: you might want to use `let...else` to handle the variant that isn't matched
  |
3 |     let Some(x) = opt else { todo!() };
  |                       ++++++++++++++++
```

The message *names the rule* — "`let` bindings require an irrefutable pattern" —
and points at the gap: "pattern `None` not covered." The fix is to **add an
else path**, exactly as Lesson 19d showed: `if let` (do something only on
`Some`) or `let … else` (bind on `Some`, diverge on `None`). Both compile and
run:

```rust
fn main() {
    let opt: Option<i32> = Some(5);

    if let Some(x) = opt {
        println!("if let got {x}");
    }

    let Some(y) = opt else {
        println!("was None");
        return;
    };
    println!("let else got {y}");
}
```

```
if let got 5
let else got 5
```

**The inverse — an irrefutable pattern in `if let` — is a *warning*, not an
error.** `(a, b)` always matches, so the `if let` is pointless; the compiler
says so but still builds:

```rust
fn main() {
    if let (a, b) = (1, 2) {
        println!("{a} {b}");
    }
}
```

```
warning: irrefutable `if let` pattern
 --> main.rs:2:8
  |
2 |     if let (a, b) = (1, 2) {
  |        ^^^^^^^^^^^^^^^^^^^
  |
  = note: this pattern will always match, so the `if let` is useless
  = help: consider replacing the `if let` with a `let`
  = note: `#[warn(irrefutable_let_patterns)]` on by default
```

It runs and prints `1 2`. The compiler's advice is the mirror image of the
rule: when a pattern *can't* fail, drop the `if` and use a plain `let`.
**Match the tool to the refutability:** irrefutable → `let`/`for`; refutable →
`if let`/`while let`/`match`.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new patterns` works too.)* **Predict on paper before each
run.**

1. **Classify first, on paper.** For each pattern, say *refutable or
   irrefutable* — `x` · `(a, b)` · `Some(x)` · `3` · `(x, 0)` — and for each,
   name one position where it's therefore allowed. Then check any two of your
   answers in the Sandbox by putting the pattern in a plain `let`.
2. **Trigger `E0005` yourself.** Write a `let` that binds with a refutable
   pattern — e.g. destructure an `Option<i32>` you set to `Some(...)` using
   `let Some(x) = …;`. **Predict the error code** before compiling. Then fix
   it **two** ways: once with `if let`, once with `let … else` (the `else`
   must diverge — `return` or `panic!`). Notice the compiler's note *names*
   which kind of pattern `let` requires.
3. **The mirror image.** Write an `if let` whose pattern is a plain tuple
   destructure. **Predict**: error, warning, or clean compile? What does the
   compiler tell you to use instead — and why does that advice follow from
   the table in part 2?

*(You write every line here — I won't. The predictions are your answer key.
One question sorts every pattern position in Rust: "if this pattern missed,
where would the program go?" Next, Lesson 33b arms `match` itself with three
power tools: guards, `@` bindings, and nested patterns.)*

## 6. What surprised you?

A sentence or two: did **refutability** click as the reason `let Some(x)`
fails but `let (a, b)` works — "is there an else path?" Did it surprise you
that the *inverse* mistake (an irrefutable `if let`) is only a **warning**,
not an error? Tell me, and I'll fold it into the Phase-9 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.19 §19.2** "Refutability:
  Whether a Pattern Might Fail to Match": irrefutable vs refutable, the
  `E0005` "refutable pattern in local binding" error, and the `let … else`
  remedy. The framing "patterns describe the shape of data" is the BOOK's.
- **CR** — *Comprehensive Rust* (Google): covers refutability only lightly;
  this rule is the BOOK's territory.
- Every snippet compiled and run, and every error/warning captured live, on
  **rustc 1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`; temp paths
  normalized to `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 32c — States as types: broken states won't compile](32c-states-as-types.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 33b — Guards, `@` bindings & nested patterns →](33b-guards-bindings-nested.md)
