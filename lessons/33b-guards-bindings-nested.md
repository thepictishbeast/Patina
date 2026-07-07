# Lesson 33b — Guards, `@` bindings & nested patterns

*(Phase 9 — Advanced, part 5. Lesson 33 gave you the rule about WHERE patterns
may sit; this lesson upgrades the patterns themselves. Three tools: an extra
condition on an arm, capturing a value while range-testing it, and reaching
into layered data — plus one honest wall about what guards cost you.)*

## 1. Why it exists

Lesson 19c matched on *shape*: which variant, which literal. But real questions
are often about **values**, not shapes — "is this id even?", "is this number in
range 3 to 7?" Plain shape-matching can't ask that. And real data is
**layered** — an `Option` holding a struct holding fields — while your patterns
so far peel one layer at a time.

Three tools close the gap:

- **Match guards** — an `if` condition tacked onto an arm:
  `Some(x) if x < 0 => …`. The arm matches only if the pattern fits **and**
  the guard is true. With a `|` alternation, the guard covers the **whole**
  thing: `(4 | 5 | 6, _) if active` means "(matched one of those) AND active."
- **`@` bindings** — `name @ pattern` captures the value into `name` **while**
  testing it against `pattern`. `id @ 3..=7` means "is `id` in 3..=7? if so,
  bind it as `id`." Without the `@`, a range arm knows *that* it matched but
  throws the value away.
- **Nested destructuring** — patterns nest as deep as the data:
  `Some(Point { x: 0, y })` matches *only* a `Some` holding a `Point` whose
  `x` is `0`, binding `y`. Match and unpack in one shape.

(Recap from L19, nothing new: `|` is "or", `..=` is an **inclusive** range.)

## 2. The idea — read one `match` that uses everything

The Book's flagship example (Listing 19-29, adapted) puts `@`, a range, a
guard, and struct-enum destructuring in one `match`. Read each arm, then
predict the three lines:

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

Walk it: `5` hits the first arm (`@ 3..=7` captures *and* confirms the range).
`8` fails the range but passes the guard → "Even id". `11` fails both → the
catch-all binds it. **Arm order matters:** the range arm is written *first*,
so an in-range *even* value like `6` is claimed by it and never reaches the
even-guard. (The full pattern catalog behind this one example is **Book
Ch.19.3** — this lesson keeps the three tools you'll actually reach for.)

## 3. Two more tiny examples to read

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

The pattern mirrors the data's shape exactly: `Some(...)` peels the `Option`,
then `Point { x: 0, y }` matches *only if* `x` is literally `0`. Here `x` is
`3`, so the first two arms miss and the third binds both fields.

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

The first arm needs *both*: the number is 4, 5, or 6 **and** `active` is true.

## 4. Common pitfalls / real compiler errors — guards are invisible to exhaustiveness

Guards let arms slice a type by *value* — but the compiler doesn't read your
guard logic. Cover all of `i32` with two guards that are obviously complete:

```rust
fn main() {
    let n = 5;
    match n {
        x if x < 0 => println!("negative: {x}"),
        x if x >= 0 => println!("non-negative: {x}"),
    }
}
```

**Before you scroll — you and I can see every `i32` is covered. Can rustc?**

```
error[E0004]: non-exhaustive patterns: `i32::MIN..=i32::MAX` not covered
 --> main.rs:3:11
  |
3 |     match n {
  |           ^ pattern `i32::MIN..=i32::MAX` not covered
  |
  = note: the matched value is of type `i32`
  = note: match arms with guards don't count towards exhaustivity
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
```

The note states the rule outright: **"match arms with guards don't count
towards exhaustivity."** A guard is arbitrary code; the compiler won't try to
prove that `x < 0` and `x >= 0` together cover everything, so it demands a
guard-free arm anyway. The habit: when guards slice a type, end with an
unguarded arm (a plain binding or `_`) — it's the arm the checker can *see*.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new patterns` works too.)* **Predict on paper before each
run.**

1. **`@` to capture a range.** Write a `match` on an `i32` with three arms
   using `@`: one for `1..=3`, one for `4..=6`, and a catch-all — each
   printing the **captured** value with a label (e.g. `"low: 2"`). **Predict**
   which arm fires, and what it prints, for the values `2`, `5`, and `9`.
   Then remove the `@` from one arm and see what you lose.
2. **A guard that decides between two same-shape arms.** Write a `match` on an
   `i32` `score` with two arms whose *patterns are identical* (both just bind
   `n`) but whose **guards** differ… almost: `n if n >= 50` prints "pass", and
   a final **unguarded** `n` prints "fail". **Predict** the output for `72`
   and for `30`. Then put a guard on the *last* arm too (`n if n < 50`) —
   **predict the error code** before compiling, and find the note in the
   message that names the rule from part 4.
3. **Nested destructure.** Make a small `struct` with two fields, wrap one in
   `Some`, and `match` it so that one arm only fires when a specific field
   equals a literal (like `Some(Thing { kind: 0, value })`) and another binds
   both fields generally. **Predict** which arm matches your value before you
   run it. Then add the `None` arm and confirm the `match` compiles without a
   catch-all `_` — no guards here, so the checker can see everything.

*(You write every line here — I won't. The predictions are your answer key.
With guards, `@`, and nested patterns you can now read — and write — the dense
`match` blocks real Rust is full of. Next: the advanced-features tour —
`unsafe`, advanced traits, and macros.)*

## 6. What surprised you?

A sentence or two: did `@` make sense as "test *and* keep"? Did the arm-order
consequence in part 2 (the range arm claiming `6` before the even-guard could)
change how you'll order arms? And were you surprised the compiler refuses to
reason about two guards that *obviously* cover everything? Tell me, and I'll
fold it into the Phase-9 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.19 §19.3** "Pattern
  Syntax": match guards, `@` bindings (adapted from **Listing 19-29**, the
  `id @ 3..=7` example, extended here with a guard arm and a catch-all),
  guard-over-`|` semantics, and nested struct/enum/tuple destructuring — plus
  the fuller pattern catalog this lesson points at rather than reproduces.
- **CR** — *Comprehensive Rust* (Google): the pattern-vocabulary slides
  (`|`, ranges, guards); `@` is covered only lightly there.
- One footnote: in older code you may see **`ref`** in patterns to borrow
  instead of move. It still compiles, but edition-2024 default binding modes
  (match ergonomics) handle this automatically in nearly every case — treat
  `ref` as something to *recognize*, and see Book Ch.19 if you meet it.
- Every snippet compiled and run, and every error captured live, on **rustc
  1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`; temp paths
  normalized to `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 33 — Refutability: when a pattern can fail](33-refutability.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 34 — unsafe: a small, audited escape hatch →](34-unsafe.md)
