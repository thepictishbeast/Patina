# Lesson 11 — `match` (intro)

*(Phase 2 — Control flow, part 3 and last. After this comes the Phase 2 review and
the `likes` kata.)*

## 1. Why it exists

`if` / `else if` is great for one or two forks, but it gets clumsy fast when a value
has *many* possible cases each needing a different result. `match` is built for
exactly that: you hand it one value, list the cases as **patterns**, and it runs the
first arm whose pattern fits.

It has one more property `if` doesn't: `match` is **exhaustive**. You must cover
every possible case, and the compiler checks it — so you physically cannot forget
one. That's a safety feature, not red tape; a forgotten case is a classic bug, and
Rust turns it into a build error you fix in seconds.

> **How the sources frame it:** **CR** gives the cleanest first taste (literal arms +
> the `_` catch-all, returning a value); **BLOG** owns the failing "you forgot a
> case" demo; the **BOOK** gives the one metaphor worth keeping (the coin sorter) and
> the `match`-vs-`if` contrast. Deep pattern matching (pulling data *out* of values)
> waits for Phase 5 — this is the shallow intro.

## 2. The idea

The shape is a value, then a list of `pattern => result` arms:

```
match value {
    pattern_a => result_a,
    pattern_b => result_b,
    _ => fallback,
}
```

> **Metaphor (BOOK):** picture a coin-sorting machine. A coin slides along and drops
> through the **first hole it fits**. `match` works the same way: the value falls
> through the arms top to bottom and takes the **first pattern it matches** — the
> rest are skipped.

`_` is the **catch-all** — a pattern that matches anything, used as the "everything
else" arm. Two more things to know up front:

1. **`match` is an expression**, like `if` (Lesson 9): the matched arm's value becomes
   the value of the whole `match`, so a `match` can sit on the right of a `let`.
2. **It's exhaustive.** Every possible value of the type must be covered by some arm.
   For a type with a huge range (like a number), `_` is how you cover "all the rest"
   in one line. Leave a gap and it won't compile — you'll see that in part 4.

And a contrast with Lesson 9: an `if` *condition* must be a `bool`, but `match`
compares a value against patterns, so it works on any type — numbers, `bool`,
characters, and (in Phase 5) your own types.

## 3. Tiny examples to read

**A number to a word, with a catch-all.** Read it, then predict what prints:

```rust
fn main() {
    let roll = 3;
    let label = match roll {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "many",
    };
    println!("you rolled {label}");
}
```

`roll` is `3`; it falls past the `1` and `2` arms into `3 => "three"`, and that
becomes `label` (the whole `match` is an expression). Anything not listed — `0`, `4`,
`99` — would hit `_ => "many"`:

```
you rolled three
```

**Now a 30-second rep — you type this one.** `match` on a `bool`, predict before you
run:

```rust
fn main() {
    let raining = true;
    let bring = match raining {
        true => "umbrella",
        false => "sunglasses",
    };
    println!("bring your {bring}");
}
```

`raining` is `true`, so the first arm wins:

```
bring your umbrella
```

Notice there's no `_` here — and it still compiles. A `bool` has only two possible
values, and both arms (`true`, `false`) are present, so the `match` is already
exhaustive. *(That was your write-rep for this part; part 5 is the rest.)*

## 4. Common pitfalls / real compiler errors

**Forgetting a case.** Here we `match` a `u8` (a number from 0 to 255) but only
handle two values:

```rust
fn main() {
    let n: u8 = 7;
    let label = match n {
        0 => "zero",
        1 => "one",
    };
    println!("{label}");
}
```

**Before you scroll — will this compile?** It won't — and the message is unusually
helpful. Real `rustc` (1.95.0), unedited:

```
error[E0004]: non-exhaustive patterns: `2_u8..=u8::MAX` not covered
 --> main.rs:3:23
  |
3 |     let label = match n {
  |                       ^ pattern `2_u8..=u8::MAX` not covered
  |
  = note: the matched value is of type `u8`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
5 ~         1 => "one",
6 ~         2_u8..=u8::MAX => todo!(),
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
```

Read what it tells you: it names *exactly* the cases you left out —
`2_u8..=u8::MAX`, meaning "everything from 2 up to the largest `u8`" — and even shows
where to add an arm. This is the whole reason `_` exists: one catch-all arm covers
that entire leftover range.

The fix — add a `_` arm for everything else:

```rust
fn main() {
    let n: u8 = 7;
    let label = match n {
        0 => "zero",
        1 => "one",
        _ => "lots",
    };
    println!("{label}");
}
```

```
lots
```

> The compiler refusing a non-exhaustive `match` is the same spirit as the rest of
> Rust: it would rather stop you at build time than let a value silently fall through
> with no result at run time.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new matching` works too.)* **Predict on paper before each run.**

1. **Match a number to words.** Bind a number `0`–`5` of your choosing. `match` it so
   `0`–`3` each map to their own word and everything else maps to `"big"`. Store the
   result in a `let` and print it. **Predict** which arm yours hits, then run.

2. **Match on a `bool`.** Bind a `bool` and `match` it to two different strings — *no*
   `_` arm. **Predict**: will it compile without the catch-all? Run and confirm why.

3. **Cause and fix non-exhaustive.** Take your number `match` from step 1 and delete
   the catch-all arm. **Predict** the error: compile or runtime? which **code**, and
   what range will it say is "not covered"? Run it, read the help, then put the arm
   back.

*(You write every line here — I won't. The predictions are your answer key; the code
is yours. The deeper powers of `match` — pulling values apart, matching your own
types — arrive in Phase 5.)*

## 6. What surprised you?

A sentence or two: did the coin-sorter picture help? Was "you must cover every case"
a surprise — annoying, or reassuring? Tell me, and I'll pitch the Phase 2 review and
the `likes` kata at the right level.

## 7. Sources

- **CR** — *Comprehensive Rust* (Google), §6.3. Backbone of the shallow intro: literal
  arms, the `_` wildcard, `match` as a value-returning expression, exhaustiveness shown
  on `bool` (enums/patterns deliberately deferred — our Phase 5).
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Conditional control → Match." The
  **E0004 non-exhaustive** failing demo (compiler names the uncovered range and offers
  a fix) repurposed as part 4. Its comparisons to other languages' switch/case
  constructs were dropped per the no-analogy rule.
- **BOOK** — *The Rust Programming Language*, §6.2 "The `match` Control Flow Operator"
  (and §3.5's `if`-vs-`match` note). Source of the coin-sorting-machine metaphor and
  the "`if` needs a `bool`, `match` takes any type" contrast.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 10 — Loops: `loop`, `while`, and `for`](10-loops.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 12 — `String` vs `&str` →](12-string-vs-str.md)
