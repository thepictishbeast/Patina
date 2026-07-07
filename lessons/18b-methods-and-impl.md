# Lesson 18b — Methods & `impl` blocks

*(Phase 5, part 2. Lesson 18 bundled the data. Now the behaviour moves in with it.)*

## 1. Why it exists

A rectangle's area is a fact *about a rectangle* — computing it belongs with the
type, not in a loose function somewhere. An `impl` block attaches **methods** to
your struct so the behaviour lives with the data — and the way a method takes its
instance turns out to be the borrow rules you already know, wearing new clothes.

## 2. The idea

**Methods and `impl`.** Behaviour attaches to a type in an `impl` block. The first
parameter is the **receiver**, and which one you pick mirrors the borrow intent
from Phase 4:

- `&self` — borrow to **read** (most methods).
- `&mut self` — borrow to **change** the instance.
- `self` — **consume** it (the instance is moved into the method; using it
  afterward is the Phase-4 `E0382`).

A function in `impl` with **no** receiver is an **associated function** — called
with `::`, often used to build instances (`Rectangle::square(3)`). `new` is just a
*convention* for this, not a keyword. `Self` is shorthand for the type's own name.

## 3. A tiny example to read (30-second rep — you type this)

Predict both lines:

```rust
struct Rectangle { width: u32, height: u32 }
impl Rectangle {
    fn area(&self) -> u32 {                 // &self: reads the instance
        self.width * self.height
    }
    fn square(size: u32) -> Self {          // associated fn: builds one
        Self { width: size, height: size }
    }
}
fn main() {
    let r = Rectangle { width: 30, height: 50 };
    println!("area = {}", r.area());
    let sq = Rectangle::square(10);         // called with ::
    println!("square area = {}", sq.area());
}
```

```
area = 1500
square area = 100
```

## 4. Common pitfalls / real compiler errors

**Calling a method the type doesn't have — `E0599`.** Ask `r` for a method that
isn't in any `impl` for its type (a typo, or a method you haven't written yet),
and the compiler names the type, the missing method, and — when something close
exists — suggests it:

```
error[E0599]: no method named `are` found for struct `Rectangle` in the current scope
  |
  |     println!("{}", r.are());
  |                      ^^^ help: there is a method with a similar name: `area`
```

The matching exercise below hands you this wall on purpose — read *whose* impl the
compiler searched.

> A quick Phase-4 callback: a method that takes `self` (not `&self`) **consumes**
> the instance — using it after that call is the `E0382` move error you already
> know.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new methods` works too.)* **Predict on paper
before each run.**

1. **A method + an associated function.** Take your `Book` from Lesson 18: add an
   `impl Book` with a `&self` method that returns whether the book is "long"
   (say, over 300 pages), and an associated function that builds a default
   `Book`. Call both. **Predict** the results.
2. **Misspell the method** at the call site. **Predict**: the error code, and
   whether the compiler guesses what you meant.

*(You write every line here — I won't. The predictions are your answer key. Next:
printing your own types — the one derive every Rust programmer types daily.)*

## 6. What surprised you?

A sentence or two: did "the receiver (`&self` / `&mut self` / `self`) is just the
borrow rules again" connect to Phase 4? Tell me, and I'll pitch Lesson 18c to
match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §5.3 "Method Syntax" —
  `&self`/`&mut self`/`self`, associated functions, `Self`. (Multiple `impl`
  blocks are legal too — Book §5.3.)
- **CR** — *Comprehensive Rust* (Google), §10.2 — the method example showing every
  kind of receiver.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 18 — Structs: bundle your data](18-defining-structs.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 18c — Printing your own types →](18c-derive-debug.md)
