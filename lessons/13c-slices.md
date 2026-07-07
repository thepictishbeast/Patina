# Lesson 13c — Slices: a borrowed window

*(Phase 3 — Text & collections, part 4. You have a whole array; a function
only needs the middle of it. Copying the middle out is wasteful — a **slice**
is a borrowed VIEW into part of a sequence, and its type hides something on
purpose: the length.)*

## 1. Why it exists

Functions shouldn't care how big the whole is. A function that sums "some
numbers" shouldn't demand *exactly five* — but an array's length is part of
its type (`[i32; 5]`), so a function taking `[i32; 5]` rejects a `[i32; 6]`.
The **slice** solves this: a borrowed window into part (or all) of a
sequence, whose type **erases the length** — one function fits any size.

## 2. The idea

**`&scores[1..3]` is a slice**: a window onto elements 1 and 2 of `scores`.
`1..3` is Lesson 10c's exclusive range — index 1 up to *but not including* 3.

Its type is **`&[i32]`** — look closely at what's *missing*: no `; N`. The
length dropped out of the type. That's the point: a function taking `&[i32]`
accepts a window of *any* length, from any array (and, later, any `Vec`).

Two more facts to file:

- **You've been using a slice since Lesson 12.** A `&str` *is* a slice of a
  `String` — same idea, character data instead of numbers.
- **The `&` marks it as *borrowed***: the slice is a view, not a copy — the
  data stays where it was. The deeper borrowing rules behind `&` are Phase 4;
  here, "window" is the whole story.

## 3. A tiny example to read

**A view into the array:**

```rust
fn main() {
    let scores = [10, 20, 30, 40, 50];
    let middle: &[i32] = &scores[1..3];   // elements 1 and 2
    println!("{middle:?}");
}
```

```
[20, 30]
```

Read the annotation: `middle` is a `&[i32]` — no length in sight. `scores`
still owns all five numbers; `middle` just frames two of them.

## 4. Common pitfalls / real panics — the window must fit the wall

**A slice range past the end panics.** Slices are bounds-checked exactly like
Lesson 13b's indexing — a window can't hang past the edge of the data. Here
the end index is computed, so the check happens at run time:

```rust
fn main() {
    let scores = [10, 20, 30, 40, 50];
    let end = 7;
    let window = &scores[2..end];
    println!("{window:?}");
}
```

**Before you scroll — five elements, a window asking for `2..7`. Compile
error or panic?**

```
thread 'main' panicked at main.rs:4:25:
range end index 7 out of range for slice of length 5
```

Same family as 13b's rule — the runtime catches what the compiler can't see —
and the message tells you both numbers you need: the length you have and the
end you asked for.

**One more panic to know *about* (not to memorise):** slicing a `String`
counts **bytes**, and one character can be several bytes (Lesson 12's UTF-8
fact) — so a range that cuts through the *middle* of a character panics at
run time, with a message naming the exact boundary. When you start slicing
real text, read **Book Ch.8.2 "Storing UTF-8 Encoded Text"** first; for now,
slice numbers freely and slice strings on obvious edges.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new collections` works too.)* **Predict on paper before
each run.**

1. **The window.** Make an array of five numbers. Take a slice of the **last
   two** (think about the right range) and print it with `{:?}`. **Predict**
   the slice before you run.
2. **Any length fits.** Write a function `fn peek(xs: &[i32])` that just
   prints `xs.len()`. Call it with a slice of two elements, then a slice of
   four, from the *same* array. **Predict** both printed lengths — then say
   in one sentence why one function accepted both, when no `[i32; N]`
   function could.
3. **Past the edge.** Slice with a *computed* end that's too big (a `let end
   = …;` variable, like part 4). **Predict**: compile error or panic — and
   which two numbers will the message report?

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. That's the fixed-shape trio: tuples bundle, arrays row,
slices frame. Next: collections that GROW — `Vec` and `HashMap`.)*

## 6. What surprised you?

A sentence or two: did "the length drops out of the type" land as the reason
slices exist? Did it surprise you that you'd already met a slice — `&str` —
a lesson before this one named the idea? Tell me, and I'll pitch Lesson 14 to
match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §4.3 "The Slice Type": slices
  as references to a run of elements, `&str` as a string slice, and (Ch.8.2)
  the UTF-8 char-boundary rule this lesson points at rather than demonstrates.
- **CR** — *Comprehensive Rust* (Google), §8.3–8.4: the slice idea — the
  length drops out of `&[T]`, so one function fits any size.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), §2.3: the framing that a
  `&str` is a slice of a `String`.
- The "a view / window into a sequence" framing is where CR and BLOG converge.
  Deep borrowing (why the `&` matters) is Phase 4, not taught here. Output
  captured live on **rustc 1.95.0** (edition 2024; temp paths and
  run-specific thread ids normalized).

---

<!-- lesson-nav -->
[← Lesson 13b — Arrays: a fixed row of one type](13b-arrays.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 14 — Vec: the growable list →](14-vec.md)
