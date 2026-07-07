# Lesson 10 — `loop`: repeat until you `break`

*(Phase 2 — Control flow, part 3. Lesson 9 let the program *choose* a path; the
next three lessons make it *repeat* one. Rust has three loop tools — `loop`,
`while`, and `for`, one lesson each — and this one starts with the simplest and
most surprising: the loop that can hand a value back.)*

## 1. Why it exists

Lots of work is the same step done over and over: count down from 3, add up a
list, try again until something succeeds. Writing the step out by hand each
time is tedious and error-prone. A **loop** says "do this block again and
again."

The bluntest form is **`loop`**: repeat *forever*, until you explicitly
`break` out. That sounds like the least useful of the three — but its one
strange property, a single guaranteed exit, gives it a power the others don't
have. That power is this lesson.

## 2. The idea

A bare `loop` runs its block forever; `break` is the exit:

```
loop {
    // runs again and again
    break; // ...until this fires
}
```

Because a `loop` has exactly **one** way out — `break` — it's the one loop
that can hand back a *value*: write `break <value>;` and that value becomes
the value of the whole `loop` (the same "a block produces a value" idea you
met in Lessons 6 and 9b, now for loops). The other two loop forms can't do
this, because they can also stop on their own — a condition turning false, a
collection running out — so there'd be no single value to hand back. Part 4
shows the compiler enforcing exactly that.

One small piece of new vocabulary rides along, because loops are where it
earns its keep: **`counter += 1` is shorthand for `counter = counter + 1`** —
"add and put back." Every arithmetic operator has this form (`-=`, `*=`, …);
you'll see `-=` in Lesson 10b.

## 3. A tiny example to read

**`loop` that carries a value out.** Read it, then predict `result`:

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {result}");
}
```

The loop counts `counter` up; when it hits 10, `break counter * 2` exits *and*
hands `20` back as the loop's value, so `result` is `20`:

```
result is 20
```

(One more `loop` trick exists — **labels** like `'outer:` for breaking out of
*nested* loops. It's an edge tool, not a baby step: read Book §3.5 "Loop
Labels" when you first need it.)

## 4. Common pitfalls / real compiler errors

**`break <value>` only works in a `loop` — `E0571`.** Try to make a `while`
hand a value back and the compiler explains the whole design:

```rust
fn main() {
    let mut number = 3;
    let _result = while number != 0 {
        number -= 1;
        break number;
    };
}
```

**Before you scroll — part 2 told you why this can't work. What will rustc
say?**

```
error[E0571]: `break` with value from a `while` loop
 --> main.rs:5:9
  |
3 |     let _result = while number != 0 {
  |                   ----------------- you can't `break` with a value in a `while` loop
4 |         number -= 1;
5 |         break number;
  |         ^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
  |
help: use `break` on its own without a value inside this `while` loop
  |
5 -         break number;
5 +         break;
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0571`.
```

"Can only break with a value inside `loop`" — the compiler states the rule
from part 2 verbatim. A `while` can end because its condition turned false,
and *that* ending carries nothing; so Rust refuses to let any of its exits
carry something. One guaranteed exit is the price of a returned value, and
only `loop` pays it.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new loops` works too.)* **Predict on paper
before each run.**

1. **`loop` that carries a value.** Write a `loop` that counts up from `0`
   and, when the count reaches `5`, does `break count * count;` into a
   `let answer = loop {…};`. **Predict** `answer`, then run.
2. **Shorthand check.** In the same program, replace your counting line with
   the long form (`count = count + 1`). **Predict**: does anything about the
   output change? What is `+=` *for*, if not correctness?
3. **The wall, on purpose.** Rewrite task 1 as a `while count < 5` that tries
   to `break count * count;`. **Predict the error code** and the phrase the
   compiler will use about where break-with-a-value is allowed.

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next: Lesson 10b — the loop that watches a condition.)*

## 6. What surprised you?

A sentence or two: did "only `loop` can carry a value" make sense given the
single exit — and did the `E0571` message say it better than part 2 did? Tell
me, and I'll tune the next two loop lessons to where you actually are.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.5 "Control Flow → Repetition
  with Loops": the `loop` form, `break` *with a value* (the counter → `20`
  example, reproduced here), and loop **labels** (pointed at, not taught).
- **CR** — *Comprehensive Rust* (Google), §6.4–6.5: the crispest reason `loop`
  alone can carry a value (one exit = one returned value).
- Compiler output captured live on **rustc 1.95.0** (edition 2024;
  `rustc --edition 2024 FILE.rs`, temp paths normalized to `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 9b — if is an expression: branching that produces a value](09b-if-as-expression.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 10b — while: repeat while a condition holds →](10b-while.md)
