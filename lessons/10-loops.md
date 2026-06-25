# Lesson 10 — Loops: `loop`, `while`, and `for`

*(Phase 2 — Control flow, part 2. Lesson 9 let the program *choose* a path; now
you make it *repeat* one.)*

## 1. Why it exists

Lots of work is the same step done over and over: count down from 3, add up a
list, try again until something succeeds. Writing the step out by hand each time
is tedious and error-prone. A **loop** says "do this block again and again,"
and Rust gives you three loop tools — each fits a different *shape* of repetition:

- **`loop`** — repeat forever, until you explicitly `break` out.
- **`while`** — repeat *while* a yes/no condition stays true.
- **`for`** — walk through each item of a collection or a range. The everyday one.

> **How the sources frame it:** the **BOOK** earns each form in order and shows the
> two things only `loop` has — a `break` that *carries a value* and loop *labels*;
> **CR** gives the crispest reason `loop` can carry a value; **BLOG** adds one sharp
> off-by-one trap. We follow BOOK and keep BLOG's trap.

## 2. The idea

**`loop` — repeat until you break.** A bare `loop` runs its block forever; `break`
is the exit:

```
loop {
    // runs again and again
    break; // ...until this fires
}
```

Because a `loop` has exactly **one** way out — `break` — it's the one loop that can
hand back a *value*: write `break <value>;` and that value becomes the value of the
whole `loop` (the same "a block produces a value" idea you met in Lessons 6 and 9,
now for loops). `while` and `for` can't do this, because they can also stop on their
own (a false condition / running out of items), so there'd be no single value to
hand back.

**`while` — repeat while true.** Give it a `bool` condition; it runs the block,
re-checks the condition, and stops the moment it's `false`:

```
while number != 0 {
    // runs while the condition holds
}
```

`while` is really just the common `loop { if !cond { break } … }` pattern with the
plumbing built in.

**`for` — walk each item.** Point it at a collection or a **range** and it visits
each value in turn:

```
for item in collection {
    // runs once per item
}
```

Ranges make counting easy: `1..4` is **exclusive** (1, 2, 3 — stops before 4),
`1..=4` is **inclusive** (1, 2, 3, 4), and `.rev()` walks a range backwards. `for`
is the one you'll reach for most: it can't run off the end, which (as you'll see in
part 4) is exactly the bug `while` invites.

## 3. Tiny examples to read

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

The loop counts `counter` up; when it hits 10, `break counter * 2` exits *and* hands
`20` back as the loop's value, so `result` is `20`:

```
result is 20
```

**`while` countdown — you type this one (30-second rep).** Predict the output before
you run it:

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

It prints each number while `number != 0` holds, then falls out and prints liftoff:

```
3!
2!
1!
LIFTOFF!!!
```

**`for` over a range, three ways.** Predict each line:

```rust
fn main() {
    for n in 1..4 { print!("{n} "); }   // exclusive
    println!();
    for n in 1..=4 { print!("{n} "); }  // inclusive
    println!();
    for n in (1..4).rev() { print!("{n} "); } // backwards
    println!();
}
```

```
1 2 3 
1 2 3 4 
3 2 1 
```

*(That `while` rep was your write practice for this part; part 5 is the rest.)*

## 4. Common pitfalls / real compiler errors

**The off-by-one that `for` exists to kill.** Here's a `while` walking an array by
index — with the bound one too far (`<=` where it should be `<`):

```rust
fn main() {
    let a = [10, 20, 30];
    let mut index = 0;
    while index <= a.len() {
        println!("{}", a[index]);
        index += 1;
    }
}
```

**Before you scroll — will this compile, and if it runs, does it finish cleanly?**

It compiles fine. Then it prints `10`, `20`, `30`… and crashes, because `a.len()`
is `3` and there is no `a[3]`. Real output (rustc 1.95.0), unedited:

```
10
20
30

thread 'main' panicked at main.rs:5:24:
index out of bounds: the len is 3 but the index is 3
```

The fix isn't to fiddle with `<=` vs `<` until it works — it's to stop indexing by
hand. `for` walks the items themselves, so it *cannot* run off the end:

```rust
fn main() {
    let a = [10, 20, 30];
    for value in a {
        println!("{value}");
    }
}
```

```
10
20
30
```

> A panic is a *runtime* crash, not a compile error — the program built fine and
> only blew up when it actually reached the bad index. `for` removes the whole
> class of bug by never letting you name an index at all.

**Breaking out of nested loops — labels.** A plain `break` only exits the loop it's
in. When loops nest and you want to leave the *outer* one, give it a label (a name
starting with `'`) and `break` that:

```rust
fn main() {
    let mut pairs = 0;
    'outer: for a in 1..=3 {
        for b in 1..=3 {
            if a + b == 4 {
                break 'outer; // leaves BOTH loops, not just the inner one
            }
            pairs += 1;
        }
    }
    println!("pairs before stopping: {pairs}");
}
```

```
pairs before stopping: 2
```

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new loops`. **Predict on paper before each run.**

1. **`for` + range.** Print the numbers `1` through `5`, each on its own line, using
   a `for` and a range. **Predict**: do you need `1..5` or `1..=5`? Run and check.

2. **`while` accumulator.** Start `let mut total = 0;` and a counter at `1`. Use a
   `while` to add the counter into `total` and step the counter up, stopping after
   it passes `5`. Print `total`. **Predict** the final number before you run.

3. **`loop` that carries a value.** Write a `loop` that counts up from `0` and, when
   the count reaches `5`, does `break count * count;` into a `let answer = loop {…};`.
   **Predict** `answer`, then run.

4. **Cause and fix the off-by-one.** Walk a 3-element array with a `while` and an
   index bound that's one too far. **Predict** the error (compile or runtime? which
   message?). Run it, then rewrite it as a `for` so it can't happen.

*(You write every line here — I won't. The predictions are your answer key; the
code is yours.)*

## 6. What surprised you?

A sentence or two: did "only `loop` can carry a value" make sense given how it has a
single exit? Did the off-by-one panic land the way you predicted? Tell me and I'll
tune Lesson 11 (`match`) to where you are.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.5 "Control Flow → Repetition with
  Loops." Backbone: all three loop forms in order, `break` *with a value* (`loop`
  counter → `20`, Listing parallels `if`-as-expression), loop **labels**, and the
  fragile manual `while index < len` walk that *earns* `for` as the safe fix.
- **CR** — *Comprehensive Rust* (Google), §6.4–6.5. Cited for the crispest reason
  `loop` alone can carry a value (one exit = one returned value) and for `..` vs
  `..=`.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Loops & iterators." Cited for
  the `<` vs `<=` off-by-one caveat repurposed as part 4. (It omits break-with-value
  and labels — those come from BOOK.)
- Compiler/runtime output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 9 — `if` / `else if` / `else` (as an expression)](09-if-else-expressions.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 11 — `match` (intro) →](11-match-intro.md)
