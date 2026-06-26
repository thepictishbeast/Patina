# Phase 1 Cheatsheet — Foundations

Quick reference for Phase-1 concepts (pairs with Lessons 1–8). Forms verified on rustc 1.95.0, edition 2024.

## Bindings & mutability
- `let x = 5;` — bind a name to a value; **immutable by default**, type **inferred**.
- `let x: i32 = 5;` — same, with the type written (annotation). Same program.
- Reassigning an immutable binding → **`error[E0384]`** "cannot assign twice to immutable variable".
- `let mut x = 5; x = 6;` — `mut` opts in to changing the value (**same type only**).

## Shadowing (≠ `mut`)
- `let x = 5; let x = x + 1;` — re-`let` makes a **new** binding; still immutable.
- The type **may change**: `let s = "  "; let s = s.len();` (→ `usize`).
- `mut` **cannot** change the type: `let mut s = "  "; s = s.len();` → **`error[E0308]`** mismatched types.

## Constants
- `const MAX_POINTS: u32 = 100_000;` — `SCREAMING_SNAKE`, **type required**, compile-time, any scope (incl. global).
- A global `let` is rejected → use `const` (or `static`).

## Scalar types
- Integers: `i8…i128` / `u8…u128` + `isize`/`usize`; **default `i32`**. (Signed shows the sign only when needed.)
- Float: `f32`/`f64` (**default `f64`**). Bool: `bool`. Char: `char` (one Unicode scalar, `'a'`).
- **Overflow:** **panics** in a normal (debug) build; **silently wraps around** in an optimized (release) build.[^overflow]

## Literals & casting
- Separators `1_000_000`; bases `0xff` `0o77` `0b1010` `b'A'`; suffix `57u8`.
- Cast with `as`: `3.7_f64 as i32` → `3` (**truncates, never rounds**). No implicit numeric conversion.

## Operators
- Arithmetic `+ - * / %` — integer `/` truncates toward zero (`-5 / 3 == -1`); power `5_i32.pow(2)` (exponent is `u32`).
- Bitwise `& | ^ << >>`; logical `&& ||` (**not** `&`/`|`). Comparison `== != < <= > >=` (used in Phase 2).

## Expressions vs statements (the hinge)
- An **expression** produces a value; a **statement** does not.
- A block is an expression: `let y = { let a = 3; a + 1 };` → `y == 4` (last line, **no `;`**).
- Add `;` to the last line → it returns `()`. In a fn promising `-> i32` that's **`error[E0308]`** "expected `i32`, found `()` … remove this semicolon".

## Functions
- `fn add(a: i32, b: i32) -> i32 { a + b }` — params typed; return type after `->`; **last expression is the return** (no `;`). `return x;` also works.

## Comments & printing
- `// line` · `/* block */` · `/// doc` (covered later).
- `println!("{x}")` (inline) / `println!("{}", x)` (Display) / `println!("{:?}", v)` (Debug) / `let s = format!(…)` (returns a `String`).

## Reading errors *by hand* (the skill we build)
- Every error has a code: `error[E0384]` → `rustc --explain E0384` gives the full explanation.
- The compiler shows the cause (`first assignment to x`, `^^^ cannot assign twice`) and usually the fix (`help: … let mut x`). Read it top-to-bottom.

— *Sources:* BOOK §3.1–3.4 · CR §5–6 · BLOG (variables/types/operators/printing). Snippets verified on rustc 1.95.0.

[^overflow]: The wrap is two's-complement, and a release build with `overflow-checks = on` panics instead — detail you don't need until much later.
