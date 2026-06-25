# Lesson 9 — `if` / `else if` / `else` (as an expression)

*(Phase 2 — Control flow begins here. Phase 1 built the pieces; now you start
making the program **choose**.)*

## 1. Why it exists

A program that always does the same thing isn't much use. The moment you want it
to do one thing *when* something is true and another thing otherwise, you need a
way to branch. `if` is that fork in the road: hand it a yes/no question, and it
runs one block of code or another.

Rust adds a twist worth learning early: an `if` doesn't just *do* something, it
**produces a value** — the whole `if … else …` can sit on the right of a `let`
and hand back a result. That one fact (an `if` is an *expression*, building on the
semicolon idea from Lesson 6) is what this lesson is really about.

> **How the three sources frame it (one line):** the official **BOOK** *earns*
> "`if` is an expression" step by step and then *proves* it by breaking it with
> two real compiler errors; **CR** reinforces it with a one-line value-selector;
> **BLOG** only shows the basic shape and never mentions the value rule. We follow
> BOOK — you'll see the failures yourself.

## 2. The idea

The shape is a question, then a block to run if the answer is yes:

```
if temperature > 25 {
    // runs only when the question is true
}
```

`temperature > 25` is the **condition**. It has to be a real yes/no value — a
`bool`, which is either `true` or `false`. Rust is strict here: there is **no
"truthiness."** A number is not a stand-in for "yes," an empty string is not a
stand-in for "no." If you want a `bool`, you write something that produces one — a
comparison like `>`, `==`, `!=` does exactly that.

Add `else` for the "otherwise" path, and chain `else if` to ask further questions
in order — the first one that's `true` wins, and the rest are skipped:

```
if temperature > 25 {
    // hot
} else if temperature > 10 {
    // mild
} else {
    // cold
}
```

Now the leap. Remember from Lesson 6 that a block `{ … }` ends in an expression
and *produces that value*. An `if` is built from blocks, so an `if` produces a
value too: whichever block runs, **its** last value becomes the value of the whole
`if`. That means an `if` can go on the right-hand side of a `let`:

```
let number = if condition { 5 } else { 6 };
```

If `condition` is `true`, the whole right side *is* `5`, so `number` is `5`. One
catch falls straight out of this: every arm has to produce the **same kind** of
value. `number` is one binding with one type — Rust can't have it be a whole
number on one path and a piece of text on another. Both arms must agree. You'll
watch the compiler enforce exactly that in part 4.

## 3. Tiny examples to read

**Branching with `else if`** — read it, then predict its output:

```rust
fn main() {
    let temperature = 18;
    if temperature > 25 {
        println!("warm");
    } else if temperature > 10 {
        println!("mild");
    } else {
        println!("cold");
    }
}
```

`18 > 25` is `false`, so the first block is skipped; `18 > 10` is `true`, so the
second one wins and the rest are skipped. It prints:

```
mild
```

**Now a 30-second rep — you type this one.** This is the value form: the `if`
sits on the right of a `let` (BOOK's Listing 3-2). Type it yourself and **predict
before you run** — what is `number`?

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

Notice there's no `;` inside `{ 5 }` — that bare `5` is the block's value (Lesson
6). Put a semicolon there and the block would produce nothing, which breaks the
whole thing — the same `;`-turns-a-value-into-nothing rule you already met.
*(That was your first small write-rep; part 5 is where you write the most.)*

## 4. Common pitfalls / real compiler errors

Two failures are worth feeling in your hands, because each one teaches a rule.

**Pitfall 1 — handing `if` something that isn't a `bool`.** It's tempting to think
a number like `3` should count as "yes." Rust doesn't work that way — it wants a
real `bool`:

```rust
fn main() {
    let number = 3;
    if number {
        println!("nonzero");
    }
}
```

**Before you scroll — will this compile?** It won't. Real `rustc` (1.95.0),
unedited:

```
error[E0308]: mismatched types
 --> main.rs:3:8
  |
3 |     if number {
  |        ^^^^^^ expected `bool`, found integer

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

The fix is to ask a real question: `if number != 0 {`. The comparison produces a
`bool`, which is what `if` demands.

**Pitfall 2 — arms that disagree on type.** Here the two arms try to hand back
different kinds of value:

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

Read the carets: it points at the `5` and says **"expected because of this,"** then
at `"six"` with **"expected integer, found `&str`"** (`&str` is a piece of text).
The first arm set the type; the second has to match. `number` is a single binding
with a single type — the compiler is holding both arms to it.

> A compiler error here isn't pickiness — it's Rust refusing to let one name mean
> two different kinds of thing depending on which way the program branched.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new branching`. **Predict on paper before you
run each part.**

1. Write `main` with a binding `score` set to any whole number you choose. Use an
   `if` / `else if` / `else` to print `"high"`, `"mid"`, or `"low"` based on it
   (you pick the cutoffs). **Predict** which line prints for your number, then run.

2. Now the value form. Write `let label = if score >= 50 { "pass" } else { "fail" };`
   and print `label`. **Predict** what it prints for your `score`, then run.

3. Break it on purpose: change one arm so the two arms produce *different* kinds of
   value (e.g. one returns a number, the other a piece of text). **Predict** —
   will it compile? Which **error code**, and what will the carets point at? Run it
   and check, then change it back so both arms agree.

*(You write every line here — I won't. The predictions are your answer key; the
code is yours.)*

## 6. What surprised you?

In a sentence or two: did "an `if` produces a value" feel natural after the
semicolon lesson, or strange? Did the "no truthiness — hand it a real `bool`" rule
trip you up? Tell me, and I'll tune the next lesson (loops) to where you actually
are.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.5 "Control Flow → `if`
  Expressions." Backbone: the stepwise "`if` is an expression" build, the
  `let number = if condition { 5 } else { 6 };` centerpiece (Listing 3-2), and
  both failing demos (non-`bool` condition, mismatched arms) repurposed as part 4.
- **CR** — *Comprehensive Rust* (Google), §6.2. Cited for contrast (the one-line
  value-selector form). Its "like other languages" framing was dropped per the
  no-analogy rule.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Conditional control." Cited
  for contrast — it shows the basic `if/else if/else` shape but not the
  value-producing rule or the arms-must-agree constraint.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 8 — Comments & printing](08-comments-and-printing.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 10 — Loops: `loop`, `while`, and `for` →](10-loops.md)
