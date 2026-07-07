# Lesson 19d — Concise matching: `if let`, `while let`, `let…else`

*(Phase 5, part 5. A full `match` is exhaustive by construction. Sometimes you
genuinely care about ONE case — Rust has three short forms for exactly that.)*

## 1. Why it exists

Writing a whole `match` to handle `Some` and do nothing for `None` is ceremony.
The concise forms trade the exhaustiveness *checking* away **on purpose** — you're
saying "this one case, and I know what I'm ignoring." Used well they keep the
happy path flat and readable; used carelessly they hide cases — which is why they
come *after* you've learned the full `match`.

## 2. The idea

- `if let Some(v) = opt { … }` — handle just one case (you trade away
  exhaustiveness checking; add a plain `else { … }` if you want the other side).
- `while let Some(x) = stack.pop() { … }` — loop for as long as the pattern keeps
  matching.
- `let Some(v) = opt else { return; };` — bind on success and keep the **happy
  path** flat; the `else` must **diverge** (`return` / `break` / `panic!` — it
  can't just fall through, because `v` wouldn't exist).

## 3. A tiny example to read

Predict all six lines:

```rust
fn main() {
    let config: Option<i32> = Some(42);
    if let Some(v) = config { println!("configured: {v}"); }

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() { println!("popped {top}"); }

    let maybe: Option<i32> = Some(10);
    let Some(n) = maybe else { return; };
    println!("got {n}");
}
```

```
configured: 42
popped 3
popped 2
popped 1
got 10
```

## 4. Common pitfalls / real compiler errors — a plain `let` can't take a maybe

`let` binds **irrefutably** — the pattern must always fit. Write
`let Some(v) = maybe;` with no `else`, and:

```
error[E0005]: refutable pattern in local binding
 --> main.rs:3:9
  |
3 |     let Some(v) = maybe;
  |         ^^^^^^^ pattern `None` not covered
  …
help: you might want to use `let else` to handle the variant that isn't matched
```

The pattern *could* fail (`None`), and a plain `let` has no plan for that — so
the compiler points you at exactly this lesson's tools. The matching exercise
below hands you this wall — **predict the code**, then read the compiler's two
suggested ways out.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new concise` works too.)* **Predict on paper
before each run.**

1. **`if let` and `let…else`.** Take an `Option`, print its value with `if let` if
   it's `Some`. Then rewrite using `let…else` (binding on success, `return`ing in
   the `else`). **Predict** both behaviours.
2. **Drop the `else`.** Turn the `let…else` into a plain `let` with a `Some(…)`
   pattern. **Predict** the error code and what the compiler suggests.

*(You write every line here — I won't. The predictions are your answer key.
Structs + enums + `Option` + `match` are how you make a type that can only ever
hold *valid* states. Next: error handling — `Result`, `?`, `panic!`.)*

## 6. What surprised you?

A sentence or two: does "trade exhaustiveness away on purpose" make `if let` feel
like a tool or a trap? Tell me, and I'll pitch Lesson 20 to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §6.3 (`if let`, `let…else`; the
  diverging-`else` rule in full).
- **CR** — *Comprehensive Rust* (Google), §12.5 (`while let`).
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 19c — match patterns in depth](19c-match-in-depth.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 20 — Error Handling: `Result`, `?`, `panic!` →](20-error-handling.md)
