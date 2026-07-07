# Lesson 9b — `if` is an expression: branching that produces a value

*(Phase 2 — Control flow, part 2. Lesson 9 made the program choose. Rust's
twist, worth learning this early: the choice **produces a value** — a whole
`if … else …` can sit on the right of a `let`. That one fact builds straight
on the semicolon idea from Lesson 6, and it changes how you'll write Rust.)*

## 1. Why it exists

You'll constantly want to *pick a value* based on a condition: the label is
`"pass"` or `"fail"`, the price is discounted or not. In many languages that
means declaring a variable, then mutating it inside an `if`. Rust has a
cleaner move — because `if` is an **expression**, the branch itself *is* the
value, and one `let` captures it.

Remember from Lesson 6 that a block `{ … }` ends in an expression and
*produces that value*. An `if` is built from blocks, so an `if` produces a
value too: whichever block runs, **its** last value becomes the value of the
whole `if`.

## 2. The idea

The whole move in one line:

```
let number = if condition { 5 } else { 6 };
```

If `condition` is `true`, the whole right side *is* `5`, so `number` is `5`.

One catch falls straight out of this: every arm has to produce the **same
kind** of value. `number` is one binding with one type — Rust can't have it be
a whole number on one path and a piece of text on another. **Both arms must
agree.** You'll watch the compiler enforce exactly that in part 4.

And Lesson 6's semicolon rule rides along: it's the *bare* `5` (no `;`) that
makes the block produce a value. Put a semicolon there and the arm produces
nothing — you'll test that yourself in part 5.

## 3. A tiny example to read — then type

**A 30-second rep — you type this one.** The `if` sits on the right of a `let`
(BOOK's Listing 3-2). Type it yourself and **predict before you run** — what
is `number`?

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is: {number}");
}
```

`condition` is `true`, so the `if` arm wins and the whole expression is `5`:

```
The number is: 5
```

Notice there's no `;` inside `{ 5 }` — that bare `5` is the block's value
(Lesson 6). *(That was your first small write-rep; part 5 is where you write
the most.)*

## 4. Common pitfalls / real compiler errors

**Arms that disagree on type.** Here the two arms try to hand back different
kinds of value:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { "six" };
    println!("The number is: {number}");
}
```

**Before you scroll — what stops this?**

```
error[E0308]: `if` and `else` have incompatible types
 --> main.rs:3:44
  |
3 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Read the carets: it points at the `5` and says **"expected because of this,"**
then at `"six"` with **"expected integer, found `&str`"**. (`&str` is
Lesson 12's topic — for now, just read it as "a piece of text"; the compiler
had to name the type, you don't have to know it yet.) The first arm set the
type; the second has to match. `number` is a single binding with a single
type — the compiler is holding both arms to it.

> A compiler error here isn't pickiness — it's Rust refusing to let one name
> mean two different kinds of thing depending on which way the program
> branched.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new branching` works too.)* **Predict on paper
before you run each part.**

1. **The value form.** With a `score` binding of your choice, write
   `let label = if score >= 50 { "pass" } else { "fail" };` and print `label`.
   **Predict** what it prints for your `score`, then run.
2. **Break it on purpose.** Change one arm so the two arms produce *different*
   kinds of value (e.g. one a number, the other a piece of text). **Predict** —
   will it compile? Which **error code**, and what will the carets point at?
   Run it and check, then change it back so both arms agree.
3. **The semicolon trap, revisited.** Restore the working version, then put a
   `;` after the value inside the first arm (`{ "pass"; }`). **Predict**: does
   it still compile? If not — which of this lesson's rules did you just break,
   and what does Lesson 6 say a `;` does to a block's value? Undo it.

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next: doing things more than once — loops.)*

## 6. What surprised you?

In a sentence or two: did "an `if` produces a value" feel natural after the
semicolon lesson, or strange? Did the arms-must-agree rule click once you saw
*which* arm the compiler blamed? Tell me, and I'll tune the loops lesson to
where you actually are.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.5 "Control Flow → `if`
  Expressions": the stepwise "`if` is an expression" build, the
  `let number = if condition { 5 } else { 6 };` centerpiece (Listing 3-2), and
  the mismatched-arms failure repurposed as part 4.
- **CR** — *Comprehensive Rust* (Google), §6.2: the one-line value-selector
  form.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 9 — if / else if / else: making the program choose](09-if-else.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 10 — loop: repeat until you break →](10-loop-and-break.md)
