# Phase 2 Quiz — Control Flow

A self-check for Phase 2 (Lessons 9–11: `if`, loops, `match`). It's all
**predict-then-check**: read each question and decide your answer *before* you look
at the **Answers** section at the bottom — that prediction is the point, the same
active-recall move as the lessons. Don't run the code first; predict, then verify
(by running it, or by reading the answer key). Ten questions.

> Tip: cover the Answers section (or scroll so you can't see it) until you've
> committed to an answer for every question.

---

## Questions

**Q1 — predict the output.**
```rust
let hour = 14;
let part = if hour < 12 { "morning" } else { "afternoon" };
println!("{part}");
```

**Q2 — does this compile? If not, what's the error?**
```rust
let count = 5;
if count {
    println!("yes");
}
```

**Q3 — predict the output.**
```rust
let mut x = 0;
let y = loop {
    x += 2;
    if x == 8 { break x + 1; }
};
println!("{y}");
```

**Q4 — predict the output.**
```rust
for n in 2..5 { print!("{n} "); }
```

**Q5 — predict the output.**
```rust
for n in (1..=3).rev() { print!("{n} "); }
```

**Q6 — does this compile? Does it run cleanly?**
```rust
let items = [10, 20, 30];
let mut i = 0;
while i <= items.len() {
    println!("{}", items[i]);
    i += 1;
}
```

**Q7 — does this compile? If not, what's the error?**
```rust
let n: u8 = 4;
let s = match n {
    0 => "none",
    1 => "one",
    2 => "two",
};
```

**Q8 — predict the output.**
```rust
let n = 2;
let word = match n {
    1 => "one",
    2 => "two",
    _ => "other",
};
println!("{word}");
```

**Q9 — fill in the blanks (concept).** An `if` condition must be a `____`, but a
`match` can compare a value of `____` type.

**Q10 — short answer (concept).** Of the three loops, only one can hand a value back
out with `break value;`. Which one, and why can't the other two?

---

## Answers

*(Verified on rustc 1.95.0, edition 2024.)*

**A1 — `afternoon`.** `14 < 12` is `false`, so the `else` arm is chosen and the whole
`if` expression is `"afternoon"`. (Lesson 9: `if` is an expression.)

**A2 — No, it doesn't compile.** `error[E0308]: mismatched types … expected ``bool``,
found integer`. There is no "truthiness" in Rust — `if` needs a real `bool`. The fix
is to ask a question that produces one, e.g. `if count != 0 {`. (Lesson 9.)

**A3 — `9`.** `x` climbs 2, 4, 6, 8; when it hits `8`, `break x + 1` exits the loop
*and* hands `9` back as the loop's value, so `y` is `9`. (Lesson 10: only `loop`
carries a value out.)

**A4 — `2 3 4 `.** `2..5` is **exclusive** — it stops before `5`. (Lesson 10: ranges.)

**A5 — `3 2 1 `.** `1..=3` is **inclusive** (1, 2, 3); `.rev()` walks it backwards.

**A6 — It compiles, but it crashes at run time.** It prints `10`, `20`, `30`, then
panics: `index out of bounds: the len is 3 but the index is 3`. The bound should be
`<` (or better, use `for value in items`, which can't run off the end). A panic is a
*runtime* failure — the build was fine. (Lesson 10.)

**A7 — No, it doesn't compile.** `error[E0004]: non-exhaustive patterns:
``3_u8..=u8::MAX`` not covered`. A `u8` ranges 0–255 and you only handled 0, 1, 2;
`match` must be exhaustive. Add a `_ => …` catch-all. (Lesson 11.)

**A8 — `two`.** `n` is `2`, which falls into the `2 => "two"` arm; the whole `match`
is an expression, so `word` becomes `"two"`. (Lesson 11.)

**A9 — `bool` / any.** An `if` condition must be a `bool`; a `match` compares a value
against patterns, so it works on any type (numbers, `bool`, characters, and — from
Phase 5 — your own types). (Lessons 9 & 11.)

**A10 — `loop`.** A `loop` has exactly one way out — `break` — so there's a single,
well-defined place to hand a value back. `while` and `for` can also stop on their own
(a `false` condition, or running out of items), so there'd be no single value to
return. (Lesson 10.)

---

*How did you do?* Anything you missed points straight at the lesson to reread before
the `likes` kata (`katas/likes.md`), which is the real test of Phase 2 — writing
control flow, not just reading it.

— *Sources:* questions written for this corpus from Lessons 9–11 (BOOK §3.5 + Ch.6,
CR, BLOG); every code snippet compiled and run on **rustc 1.95.0**, edition 2024.
