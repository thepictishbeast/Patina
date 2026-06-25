# Lesson 33 — Advanced Patterns & Matching

*(Phase 9 — Advanced, part 1. Lesson 19 taught you `match`: bind the inner data,
cover every case. This lesson goes deeper into the **pattern** itself — the little
shape on the left of `=>` (and the left of `=` in a `let`). You'll add an extra
condition to an arm, capture a value **and** range-test it in one move, reach into
nested data, and finally learn the rule that explains why `let Some(x) = …` is
rejected but `let (a, b) = …` is fine.)*

## 1. Why it exists

A pattern isn't just for `match`. Every time you write `let (a, b) = pair;` or
`for (k, v) in map`, the left side is a **pattern** — it **describes the shape of the
data** and pulls pieces out. Lesson 19 used patterns to pick an enum variant. But
real data is layered: an `Option` holding a struct holding a tuple. And sometimes
"which arm" depends on more than shape — it depends on a *value* ("is this id even?",
"is this number in range 3 to 7?"). Plain shape-matching can't ask that.

This lesson adds the missing pieces — extra conditions, value capture, reaching into
nested shapes — and names the one rule underneath all pattern positions:
**refutability** (can this pattern *fail* to match?). That rule is what makes `let`
and `for` behave differently from `if let` and `match`, and once it clicks the rest
falls into place.

> **How the sources frame it:** the **BOOK** Ch.19 "Patterns and Matching" is the
> backbone — it's the only source that lays out the full pattern catalog *and* names
> refutability as a concept tied to a real compiler error (`E0005`). **CR** supplies
> the pattern vocabulary (`|`, ranges, guards) you already met in L19 but treats `@`
> and refutability lightly. The framing "patterns describe the shape of data" is the
> BOOK's.

## 2. The idea

**The spine: refutable vs irrefutable.** A pattern is **irrefutable** if it *always*
matches — `(a, b)` matches every pair, `x` matches anything. It's **refutable** if it
*can fail* — `Some(x)` doesn't match `None`; `3` doesn't match `4`.

Now the rule that ties everything together — it's about whether there's an **else path**:

| Position | Needs | Why |
|---|---|---|
| `let PAT = …;` | **irrefutable** | no else-branch — if it failed, what would run? |
| `for PAT in …` | **irrefutable** | each item must bind; nowhere to send a miss |
| function params | **irrefutable** | same — every call must bind |
| `match` arms | **refutable** OK | other arms catch the misses |
| `if let` / `while let` | **refutable** OK | the `else` / loop-exit is the miss path |

So `let Some(x) = opt;` is **rejected** — `opt` might be `None`, and a plain `let`
has nowhere to go on a miss. `let (a, b) = pair;` is **fine** — a pair is *always* a
pair. You'll see the exact error in part 4. The fix for the refutable case is
`if let` or `let … else` (L19) — both *provide* the missing else path.

**Three new tools that hang off `match` (and `if let`):**

- **Match guards** — an `if` condition tacked onto an arm: `Some(x) if x < 0 => …`.
  The arm matches only if the pattern fits **and** the guard is true. It's how you
  test a *value*, not just a shape. With a `|` pattern, the guard applies to the
  **whole** alternation: `(4 | 5 | 6, _) if active` means "(matched one of those) AND
  active."
- **`@` bindings** — `name @ pattern` captures the value into `name` **while** testing
  it against `pattern`. Use it when you need both: range-test a number *and* keep it.
  `id @ 3..=7` means "is `id` in 3..=7? if so, bind it as `id`."
- **Nested destructuring** — patterns nest as deep as the data: a struct inside an
  `Option` inside a `match`. `Some(Point { x: 0, y })` matches *only* a `Some` holding
  a `Point` whose `x` is `0`, binding `y`. You match and unpack in one shape.

**Recap from L19 (no new rules here):** `|` is "or" (`1 | 2 | 3`), and `..=` is an
**inclusive** range (`4..=9` is 4 through 9). We lean on both below; they're not new.

## 3. Tiny examples to read

**Irrefutable destructure — always matches, so `let` is happy.** A tuple is always
a tuple, even nested:

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

No `else` anywhere — none is needed, because these patterns *cannot* fail. Hold that
thought against part 4.

**The flagship: `@` + inclusive range + guard + struct-enum destructure, one `match`.**
Read each arm, then predict the three lines (adapted from BOOK Listing 19-29):

```rust
enum Message {
    Hello { id: i32 },
}

fn classify(msg: &Message) {
    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}");
        }
        Message::Hello { id } if id % 2 == 0 => {
            println!("Even id: {id}");
        }
        Message::Hello { id } => {
            println!("Found some other id: {id}");
        }
    }
}

fn main() {
    classify(&Message::Hello { id: 5 });
    classify(&Message::Hello { id: 8 });
    classify(&Message::Hello { id: 11 });
}
```

```
Found an id in range: 5
Even id: 8
Found some other id: 11
```

Walk it: `5` hits the first arm (`@ 3..=7` captures it *and* confirms the range). `8`
fails the range, but the guard `id % 2 == 0` is true → "Even id". `11` fails both, so
the catch-all arm binds it. **Arm order matters:** `id @ 3..=7` is written *first*, so
an in-range *even* value like `6` is claimed by the range arm and never reaches the
even-guard. The range arm goes first on purpose.

**`@` on its own — capture while you range-test:**

```rust
fn main() {
    let n = 6;
    match n {
        small @ 1..=5 => println!("small: {small}"),
        big @ 6..=10 => println!("big: {big}"),
        other => println!("out of range: {other}"),
    }
}
```

```
big: 6
```

Without the `@`, a `3..=7` arm tells you *that* it matched but throws the value away.
`@` keeps it.

**Nested destructuring — reach into a struct inside an `Option`:**

```rust
struct Point { x: i32, y: i32 }

fn main() {
    let shape = Some(Point { x: 3, y: -4 });
    match shape {
        Some(Point { x: 0, y }) => println!("on the y-axis at {y}"),
        Some(Point { x, y: 0 }) => println!("on the x-axis at {x}"),
        Some(Point { x, y }) => println!("at ({x}, {y})"),
        None => println!("no point"),
    }
}
```

```
at (3, -4)
```

The pattern mirrors the data's shape exactly: `Some(...)` peels the `Option`, then
`Point { x: 0, y }` matches *only if* `x` is literally `0`. Here `x` is `3`, so the
first two arms miss and the third binds both fields.

**A guard over a `|` pattern — the guard covers the whole "or":**

```rust
fn main() {
    let pair = (5, true);
    match pair {
        (4 | 5 | 6, active) if active => println!("matched and active"),
        (4 | 5 | 6, _) => println!("matched but not active"),
        _ => println!("no match"),
    }
}
```

```
matched and active
```

The first arm needs *both*: the number is 4, 5, or 6 **and** `active` is true. `5`
qualifies and `active` is `true`, so it wins.

## 4. Common pitfalls / real compiler errors

**Refutable pattern in a plain `let` — `E0005`.** `Some(x)` can fail (the value might
be `None`), and a `let` has no else-branch to take when it does:

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

The message *names the rule* — "`let` bindings require an irrefutable pattern" — and
points at the gap: "pattern `None` not covered." The fix is to **add an else path**,
exactly as L19 showed: `if let` (do something only on `Some`) or `let … else` (bind on
`Some`, diverge on `None`). Both compile and run:

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

**The inverse — an irrefutable pattern in `if let` — is a *warning*, not an error.**
`(a, b)` always matches, so the `if let` is pointless; the compiler says so but still
builds:

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

It runs and prints `1 2`. The compiler's advice is the mirror image of the rule:
when a pattern *can't* fail, drop the `if` and use a plain `let`. **Match the tool to
the refutability:** irrefutable → `let`/`for`; refutable → `if let`/`while let`/`match`.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new patterns`. **Predict on paper before each run.**

1. **Trigger `E0005` yourself.** Write a `let` that binds with a refutable pattern —
   e.g. destructure an `Option<i32>` you set to `Some(...)` using `let Some(x) = …;`.
   **Predict the error code** before compiling. Then fix it **two** ways: once with
   `if let`, once with `let … else` (the `else` must diverge — `return` or `panic!`).
   Notice the compiler's note *names* which kind of pattern `let` requires.

2. **`@` to capture a range.** Write a `match` on an `i32` with three arms using `@`:
   one for `1..=3`, one for `4..=6`, and a catch-all — each printing the **captured**
   value with a label (e.g. `"low: 2"`). **Predict** which arm fires, and what it
   prints, for the values `2`, `5`, and `9`. Then remove the `@` from one arm and see
   what you lose.

3. **A guard that decides between two same-shape arms.** Write a `match` on an
   `i32` `score` with two arms whose *patterns are identical* (both just bind `n`) but
   whose **guards** differ — e.g. `n if n >= 50` prints "pass" and the next `n` prints
   "fail". **Predict** the output for `72` and for `30`, and **predict what happens to
   arm order**: which arm must come first, and why does swapping them change nothing
   here but *would* matter if the first pattern were narrower?

4. **Nested destructure.** Make a small `struct` with two fields, wrap one in `Some`,
   and `match` it so that one arm only fires when a specific field equals a literal
   (like `Some(Thing { kind: 0, value })`) and another binds both fields generally.
   **Predict** which arm matches your value before you run it. Then add the `None` arm
   and confirm the `match` now compiles without a catch-all `_`.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. With guards, `@`, nested patterns, and the refutability rule, you can now read —
and write — the dense `match` blocks real Rust is full of, and you'll know at a glance
why a pattern belongs in a `let` or only in an `if let`.)*

## 6. What surprised you?

A sentence or two: did **refutability** click as the reason `let Some(x)` fails but
`let (a, b)` works — "is there an else path?" Did `@` make sense as "test *and* keep"?
Did it surprise you that the *inverse* mistake (an irrefutable `if let`) is only a
**warning**, not an error? Tell me, and I'll fold it into the Phase-9 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.19** "Patterns and Matching":
  §19.2 "Refutability: Whether a Pattern Might Fail to Match" (irrefutable vs
  refutable, the `E0005` "refutable pattern in local binding" error, and the `let …
  else` remedy); §19.3 "Pattern Syntax" — match guards, `@` bindings (adapted from
  **Listing 19-29**, the `id @ 3..=7` example, extended here with a guard arm and a
  catch-all), `|` and `..=` ranges, and nested struct/enum/tuple destructuring. The
  framing "patterns describe the shape of data" is the BOOK's.
- **CR** — *Comprehensive Rust* (Google): the pattern-vocabulary slides (`|`, ranges,
  guards) that L19 drew on; `@` and refutability are covered only lightly there.
- **BLOG** — not used here; this topic is sourced from BOOK/CR.
- Every snippet compiled and run, and every error/warning captured live, on **rustc
  1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`; temp paths normalized to
  `main.rs`). One footnote: in older code you may see **`ref`** in patterns
  (`match s { ref r => … }`) to borrow instead of move. It still compiles on 1.95, but
  edition-2024 **default binding modes** (match ergonomics) handle this automatically
  in nearly every case — treat `ref` as something to *recognize*, not a skill to learn.
  This opens the Phase-9 (Advanced) lessons.

---

<!-- lesson-nav -->
[← Lesson 32 — Trait Objects & OOP in Rust: `dyn Trait`, Encapsulation, States as Types](32-trait-objects-and-oop.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 34 — Advanced Features: `unsafe`, Traits & Macros →](34-advanced-features.md)
