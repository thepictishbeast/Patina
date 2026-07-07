# Lesson 15c — Copy & Clone: when assignment duplicates

*(Phase 4 — Ownership, part 3. Lesson 15b's error message kept saying "does
not implement the `Copy` trait" — this lesson is that missing piece. Why do
numbers shrug off the assignment that invalidated a `String`, and what's the
honest way to ask for a real second copy?)*

## 1. Why it exists

You've been assigning numbers to second names since Lesson 1 — `let y = x;` —
and nothing ever broke. If assignment *moves*, why didn't `x` get torn up?

Because not every type moves. Small, fixed-size values are so cheap to
duplicate that Rust just… duplicates them. The dividing line between
"assignment copies" and "assignment moves" is a trait — the very one
Lesson 15b's error named — and knowing which side a type is on tells you,
before you run anything, whether the famous error can even happen.

## 2. The idea

- **`Copy` types duplicate on assignment.** Integers, `bool`, `char`, floats,
  and tuples made only of those — the stack-only, fixed-size crowd. Assigning
  one makes a cheap duplicate and **both names stay valid**. No move, no
  error, nothing to think about.
- **Non-`Copy` types move.** A `String` isn't `Copy` (duplicating heap data
  isn't free, and Rust won't hide a cost that big behind an `=` sign). So it
  moves — Lesson 15b's whole story.
- **`.clone()` is the honest deep copy.** If you genuinely want a second,
  independent `String`, ask for it out loud: `s1.clone()`. The cost is real
  but *visible, right there in the code* — grep a file for `.clone()` and
  you've found every deliberate duplication.
- **A type can be `Copy` *or* have custom cleanup on drop — never both.**
  If dropping a value does real work, silently duplicating that value would
  mean the work happens twice. The two capabilities exclude each other by
  design.

## 3. Tiny examples to read

**Numbers are `Copy` — no move.** Predict it:

```rust
fn main() {
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");   // x is STILL valid
}
```

```
x = 5, y = 5
```

The exact shape that was a compile error for a `String` in Lesson 15b — legal
and boring for an `i32`. The type, not the syntax, decides.

**A `String` you want twice — clone it (30-second rep, you type this).**
Predict:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();   // deep copy, on purpose
    println!("s1 = {s1}, s2 = {s2}");
}
```

```
s1 = hello, s2 = hello
```

Both are valid because `.clone()` made a second, independent `String` — and
you can *see* the cost, right there in the code. *(That was your write-rep;
part 5 is the rest.)*

## 4. Common pitfalls — reading the `Copy` line in the error

There's no new error in this lesson; there's a new way to *read* the one you
have. Look again at two lines from Lesson 15b's `E0382`:

```
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
help: consider cloning the value if the performance cost is acceptable
```

The message is this whole lesson in two lines: the move happened *because*
the type isn't `Copy`, and the offered fix is a *clone* — with the cost named
out loud ("if the performance cost is acceptable"). Two habits fall out:

- **Before assigning, ask: is this type `Copy`?** Number-ish → yes, both
  names live. Heap-owning (`String`, `Vec`, `HashMap`) → no, it moves.
- **Don't reflex-clone.** `.clone()` silences the error every time, but
  Lesson 15b showed a move with no use-after is *already fine*, and Lesson 16
  is about looking at a value without owning it at all. Clone when you truly
  need two independent values — not to make a red message go away.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new ownership` works too.)* **Predict on paper before
each run.**

1. **Copy vs move.** Do the bind-to-a-second-name-then-use-the-first
   experiment twice: once with an `i32`, once with a `String`. **Predict**
   each outcome before running — and say which trait made the difference.
2. **The honest fix.** Take the `String` version and fix it with `.clone()`,
   printing both. **Predict** the output — then answer in one sentence: what
   did the fix *cost*, and where in the code can a reader see that cost?
3. **Tuples straddle the line.** Try the experiment with `let a = (1, true);`
   and then with `let a = (1, String::from("hi"));`. **Predict** each: copy
   or move? What rule from part 2 decides?

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. That's the ownership trio — rules, moves, copies. Next:
Lesson 16, the tool that makes most clones unnecessary: borrowing.)*

## 6. What surprised you?

A sentence or two: was it a relief or a surprise that numbers never moved?
Did "clone is visible cost, on purpose" change how you feel about the
compiler suggesting it? And did the tuple experiment in task 3 come out the
way part 2 predicted? Tell me, and I'll pitch Lesson 16 to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§4.1**: the `Copy`-types
  list, `.clone()` as the explicit deep copy, and the
  `Copy`-or-custom-cleanup-never-both rule.
- **CR** — *Comprehensive Rust* (Google), §20: the sharpest
  `Copy`-scalar-vs-`String`-move contrast and `.clone()` as a visible cost.
- Both examples compiled and run on **rustc 1.95.0** (edition 2024); the
  `E0382` excerpt in part 4 is from Lesson 15b's live capture.

---

<!-- lesson-nav -->
[← Lesson 15b — Moves: assignment hands ownership over](15b-moves.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 16 — Shared references →](16-shared-references.md)
