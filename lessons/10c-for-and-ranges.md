# Lesson 10c — `for` & ranges: walk each item

*(Phase 2 — Control flow, part 5 — the loop you'll reach for most. `loop`
repeats until you break; `while` watches a condition; **`for` walks each item
of something** — no counter to manage, no condition to keep honest, nothing to
get off by one.)*

## 1. Why it exists

Most repetition in real programs is "do this once per item": once per number
in a range, once per line, once per element. With `while` you'd manage a
counter by hand — start it, test it, step it — and every one of those is a
place to slip. **`for` deletes the bookkeeping**: point it at a sequence and
it hands you each item in turn, stopping by itself when the items run out.
It *cannot* run off the end, because you never name a position at all.

## 2. The idea

```
for item in collection {
    // runs once per item
}
```

The `item` name is fresh **each lap** — `for` binds it to the next value,
runs the block, and moves on. The commonest "collection" this early is a
**range**:

- **`1..4` is exclusive** — 1, 2, 3, stopping *before* 4.
- **`1..=4` is inclusive** — 1, 2, 3, 4. The `=` is the tell.

(Ranges have more tricks — walking backwards with `.rev()`, stepping — that
Book §3.5 shows; exclusive-vs-inclusive is the part to own now.)

One printing tool joins the kit here, because item-walking is where you want
it: **`print!` is `println!` without the newline** — it stays on the same
line. That's the whole fact; you'll use it below to print a range as one row.

## 3. A tiny example to read

**A range, two ways.** Predict both lines before you run:

```rust
fn main() {
    for n in 1..4 { print!("{n} "); }   // exclusive
    println!();
    for n in 1..=4 { print!("{n} "); }  // inclusive
    println!();
}
```

```
1 2 3 
1 2 3 4 
```

The bare `println!()` after each loop just ends the row `print!` was building.
Count the numbers: three for `1..4`, four for `1..=4` — the `=` is one extra
item, and knowing *which* end it adds is what keeps your fences un-posted.

## 4. Common pitfalls / real compiler errors

**The loop variable lives only inside the loop — `E0425`.** `for` creates
`n` fresh each lap, and when the loop ends, `n` is gone. Reach for it after
and the name simply doesn't exist:

```rust
fn main() {
    for n in 1..4 {
        println!("{n}");
    }
    println!("last was {n}");
}
```

**Before you scroll — is this a type error, a logic bug, or something
blunter?**

```
error[E0425]: cannot find value `n` in this scope
 --> main.rs:5:25
  |
5 |     println!("last was {n}");
  |                         ^ not found in this scope

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0425`.
```

"Cannot find value" — not "wrong type," not "moved": the name is simply not
*there*. `n` belongs to the loop's block — Lesson 3's rule that names live
only inside their `{ }` block, now applied to a loop. If you need the last
value after the loop, bind it to a `let mut` declared *outside* and assign
inside. The matching exercise below is this wall.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new loops` works too.)* **Predict on paper
before each run.**

1. **`for` + range.** Print the numbers `1` through `5`, each on its own
   line, using a `for` and a range. **Predict**: do you need `1..5` or
   `1..=5`? Run and check — then make the *other* one work by changing the
   endpoint.
2. **One row instead.** Change your loop to build one row with `print!`, and
   end the row after the loop. **Predict** the exact row, spaces and all.
3. **The scope wall.** After your loop, try to print the loop variable one
   more time. **Predict the error code** and the four-word phrase the
   compiler will use about the name. Then fix it the outside-`let` way and
   confirm it prints the last value.

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. That's all three loops: `loop` breaks, `while` watches,
`for` walks. Next: `match` — choosing between many shapes at once.)*

## 6. What surprised you?

A sentence or two: did "you never name a position, so you can't run off the
end" land as the reason `for` is the safe default? Did the loop variable
vanishing after the loop surprise you, or did Lesson 3's block scoping predict
it? Tell me, and I'll tune the `match` lesson to where you actually are.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.5 "Control Flow → Repetition
  with Loops": the `for` form as the safe walk, ranges with `..` / `..=`, and
  the further range tricks (`.rev()` and friends) this lesson points at rather
  than teaches.
- **CR** — *Comprehensive Rust* (Google), §6.5: `..` vs `..=`.
- Compiler output captured live on **rustc 1.95.0** (edition 2024;
  `rustc --edition 2024 FILE.rs`, temp paths normalized to `main.rs`). The
  two-row range example was compiled and run as shown (trailing spaces in the
  output are real — `print!` writes them).

---

<!-- lesson-nav -->
[← Lesson 10b — while: repeat while a condition holds](10b-while.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 11 — `match` (intro) →](11-match-intro.md)
