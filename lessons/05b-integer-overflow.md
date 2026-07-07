# Lesson 5b — Integer overflow: your first *runtime* panic

## 1. Why it exists

In Lesson 5 you picked integer widths — a `u8` holds `0` to `255`. So far, every
mistake you've made was caught **before the program ran**: the compiler refused,
printed an error code, and nothing executed. This lesson shows the other kind of
failure — the first one in this course that happens **while the program is
running**. What does Rust do when a value tries to outgrow its type?

## 2. The idea

Two different moments a program can fail:

- **Compile time** — the compiler rejects the code. You've met these: `E0384`,
  `E0308`. The program never runs at all.
- **Run time** — the code compiles fine, starts running, and then hits something
  impossible. Rust's response is a **panic**: stop immediately, say exactly
  where and why, rather than continue with a wrong value.

Arithmetic that overflows an integer's width is the classic runtime failure —
`255 + 1` simply does not fit in a `u8`. Rust checks for that while you're
learning and testing, and treats it as a bug worth halting for.

## 3. Watch it happen

```rust
fn main() {
    let mut small: u8 = 255;
    println!("small = {small}");
    small = small + 1;
    println!("small = {small}");
}
```

**Before you scroll — does this compile? Does it run? What happens?**

It compiles fine. Then, running the normal (debug) build, it prints the first line
and **stops**:

```
small = 255

thread 'main' (…) panicked at b.rs:4:13:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

*(The number in parentheses is a process id — it'll differ on your run.)*

Read the panic like you read a compile error: **where** (`b.rs:4:13` — line 4,
the addition) and **why** ("attempt to add with overflow"). Rust didn't let
`small` silently become a wrong number; it halted at the exact moment of
overflow. The fix is a roomier type from Lesson 5's table (`u32`).

> One thing to file away: this overflow guard is on while you're learning and
> testing. A speed-optimized build handles overflow differently — the full story
> is in the Book, §3.2 "Integer Overflow" (read more there when you're curious;
> build modes get taught in Lesson 37).

## 4. Common pitfalls — trusting "it compiled"

The trap this lesson exists to break: *"it compiled, so it works."* Compiling
proves the **kinds** all line up; it cannot prove your **values** stay in range.
That's why the predict-then-run habit has two questions from now on: *will it
compile?* and — if yes — *will it run cleanly?*

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, `cargo new overflow` works too.)* Predict before each run:

1. Make a `u8` set near its limit, then add enough to push it over `255`. Predict:
   will it compile? will it run? what happens? Then run it.
2. Change that `u8` to `u32` and run again. Predict the difference first.
3. Read your panic line out loud: which line number does it point at, and what
   operation does it name?

*(All yours to type. Predictions are the answer key.)*

## 6. What surprised you?

Did you expect overflow to *crash* rather than quietly wrap? Does "compiles ≠
correct" change how much you trust a green build? Tell me — Lesson 6 is the last
foundations idea before functions: what the **semicolon** really does.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.2 "Data Types" (integer overflow
  panicking in debug, wrapping in release).
- **CR** — *Comprehensive Rust* (Google), "Types and Values." Tiny-example style.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 5 — The scalar types](05-scalar-types.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 6 — Expressions, statements & the semicolon →](06-expressions-statements-semicolon.md)
