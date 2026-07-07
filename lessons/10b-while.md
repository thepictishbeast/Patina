# Lesson 10b — `while`: repeat while a condition holds

*(Phase 2 — Control flow, part 4. Lesson 10's `loop` repeats until YOU break
out. Most repetition has a natural stopping question built in — "is the
countdown at zero yet?" — and `while` is the loop that asks it every time
around.)*

## 1. Why it exists

Plenty of loops aren't "forever, until I say so" — they're "keep going *while*
something is true." You could build that from Lesson 10's parts:

```
loop {
    if !condition {
        break;
    }
    // the work
}
```

**`while` is exactly that pattern with the plumbing built in.** Give it a
`bool` condition; it checks, runs the block if `true`, checks again, and stops
the moment the answer is `false`.

## 2. The idea

```
while number != 0 {
    // runs while the condition holds
}
```

Two rules, both of which you already know from other corners:

- **The condition must be a real `bool`** — Lesson 9's no-truthiness rule
  applies to `while` exactly as it did to `if`. A comparison like `!=`
  produces one.
- **Something in the body must move the condition toward `false`** — usually a
  counter stepping down (`number -= 1`, the `-=` sibling of Lesson 10's `+=`).
  A `while` whose condition never changes is just `loop` without the honesty.

Because a `while` can end *on its own* (the condition turns false), it can't
carry a value out through `break` — that's Lesson 10's `E0571` rule from the
other side.

## 3. A tiny example — you type this one (30-second rep)

**The countdown.** Predict the output before you run it:

```rust
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
```

It prints each number while `number != 0` holds, then falls out and prints
liftoff:

```
3!
2!
1!
LIFTOFF!!!
```

Trace one lap by hand: check (`3 != 0`, true) → print → step down → check
again. The `LIFTOFF` line is *outside* the loop — it runs exactly once, after
the condition finally fails.

## 4. Common pitfalls / real compiler errors

**A condition that isn't a `bool` — `E0308`.** Lesson 9's rule, enforced in
the new spot. It's tempting to write "while number is still nonzero" as just
`while number`:

```rust
fn main() {
    let mut number = 3;
    while number {
        number -= 1;
    }
    println!("done");
}
```

**Before you scroll — you've met this exact error once before. Where?**

```
error[E0308]: mismatched types
 --> main.rs:3:11
  |
3 |     while number {
  |           ^^^^^^ expected `bool`, found integer

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

Same message, same fix as Lesson 9's `if number`: ask a real question —
`while number != 0`. One rule, every condition in the language: `if`, `while`,
and (later) match guards all demand an actual `bool`.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new loops` works too.)* **Predict on paper before each
run.**

1. **`while` accumulator.** Start `let mut total = 0;` and a counter at `1`.
   Use a `while` to add the counter into `total` and step the counter up,
   stopping after it passes `5`. Print `total`. **Predict** the final number
   before you run.
2. **Break the condition.** Change your `while` to test the bare counter (no
   comparison). **Predict the error code** and the two types the message
   names — then say which lesson you first met it in.
3. **The forgotten step.** Comment out the line that steps your counter.
   **Predict** what happens when you run it. (You can stop a runaway program
   in the Sandbox; on your own machine it's Ctrl-C.) Put the line back and
   say in one sentence why `while` puts that responsibility on *you*.

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next: Lesson 10c — the loop you'll actually use most.)*

## 6. What surprised you?

A sentence or two: did `while` feel like Lesson 10's `loop`-plus-`if` pattern
with the plumbing hidden — and did the runaway loop in task 3 change how you
think about who keeps a `while` honest? Tell me, and I'll tune Lesson 10c to
where you actually are.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.5 "Control Flow → Repetition
  with Loops": the `while` form and the 3-2-1-LIFTOFF countdown (Listing 3-3,
  reproduced here).
- **CR** — *Comprehensive Rust* (Google), §6.4: the `while` shape.
- Compiler output captured live on **rustc 1.95.0** (edition 2024;
  `rustc --edition 2024 FILE.rs`, temp paths normalized to `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 10 — loop: repeat until you break](10-loop-and-break.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 10c — for & ranges: walk each item →](10c-for-and-ranges.md)
