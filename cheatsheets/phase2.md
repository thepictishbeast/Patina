# Phase 2 Cheatsheet — Control Flow

Quick reference (pairs with the Phase-2 control-flow lessons — if / loop / while / for / match). Control flow is where Rust expects you to think in **expressions**. Verified on rustc 1.95.0, edition 2024.

## `if` is an EXPRESSION
- It produces a value: `let n = if big { 100 } else { 1 };`
- Both arms must be the **same type** → mismatched arms = **`error[E0308]`** "`if` and `else` have incompatible types".
- The condition must be a real `bool` — **no truthiness**: `if x { … }` with `x: i32` is an error.
- Chain with `else if`.

## `loop` — runs until you `break`
- `loop { … break; }` repeats forever until a `break`.
- **`break` can return a value:** `let v = loop { c += 1; if c == 3 { break c * 2; } };` → `v == 6`.
- **Labels** for nested loops: `'outer: loop { loop { break 'outer; } }` (also `continue 'outer`).

## `while` — loop while a condition holds
- `while n != 0 { n -= 1; }` — the readable shorthand for a hand-built `loop` + `if` + `break`.

## `for` — iterate a collection or a range
- `for x in &v { … }` — the safe, idiomatic loop over a collection.
- Ranges: `0..5` (exclusive → 0,1,2,3,4) · `0..=5` (inclusive → …,5).
- `for i in (1..4).rev() { … }` counts down. Prefer `for` over manual indexing — no off-by-one, no per-step bounds check.

## `match` — the multi-way branch (intro)
- `match value { 1 => …, 2 | 3 => …, _ => … }` — arms are tried top-to-bottom; the value drops into the first arm it fits (a **coin-sorting machine**).
- **Exhaustive:** every possible value must be covered → a missing case is **`error[E0004]`** "non-exhaustive patterns". `_` is the catch-all.
- `match` is an expression too — every arm yields a value of the same type.
- (Binding inner data + matching structs/enums is Phase 5.)

— *Sources:* BOOK §3.5 + Ch.6 · CR "Control Flow Basics" · BLOG "Conditional control / Loops". Snippets verified on rustc 1.95.0 (re-verified, edition 2024).
