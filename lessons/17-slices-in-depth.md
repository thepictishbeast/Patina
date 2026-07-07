# Lesson 17 — Slices in Depth

*(Phase 4 — Ownership, part 6 and last. Lesson 13 introduced slices as "a view into a
sequence." Now that you know borrowing (Lesson 16), we can see what a slice *really* is —
and the bug it quietly prevents.)*

## 1. Why it exists

A slice (`&s[0..5]`, `&v[1..3]`) isn't a copy of part of a collection — it's a **borrow**
that points *into* it. Put that together with Lesson 16's rules and something powerful
falls out: while a slice is alive, the collection is **borrowed**, so you can't change the
collection out from under the slice. That single fact kills an entire category of bug — the
"stale index," where you remember a position into some data and the data shifts beneath you.

> **How the sources frame it:** the **BOOK** owns the payoff — the `first_word` story, where
> a returned slice makes a would-be silent bug into a compile error ("eliminated an entire
> class of errors at compile time"); **CR** adds the same lesson for `Vec` (a reference into
> a vector blocks a `push` that might move the data).

## 2. The idea

You already know the mechanics from Lesson 13: `&a[1..3]` is a slice, its type is `&[T]`
(the length drops out), and `&str` is a string slice. The new idea is what the leading `&`
*means* — it's a **borrow** (Lesson 16). So:

- A slice **borrows** the collection it points into. It doesn't copy the data.
- Because it's a shared borrow, the rule from Lesson 16 applies: **while the slice is alive,
  the collection is borrowed immutably**, so you cannot mutate it (no `clear`, no `push`).

**Why that's a feature, not a restriction.** Imagine a function that, instead of a slice,
returned a plain number — *"the first word ends at index 5."* If the string later changed
(got cleared or shortened), that `5` would silently point at the wrong place, or past the
end. Nothing would stop you using it — it'd compile, then misbehave at run time. A **slice
can't go stale**: it borrows the data, so the compiler refuses to let you change the data
while the slice is alive. The latent run-time bug becomes a compile error you fix in
seconds.

**Returning slices is safe and cheap.** `fn first_word(s: &str) -> &str` returns a slice
that borrows its argument — no copy, and the borrow checker guarantees the slice can't
outlive (or get out of sync with) the string it views.

> **The picture:** a slice is a *window* into a collection (Lesson 13) — and because the
> window is a borrow, it **pins the collection open** while you're looking. You can't resize
> something you're currently looking into.

This isn't just strings: a reference into a `Vec` pins it too, because growing a `Vec` with
`push` can move its data to a new spot in memory — which would leave any reference into the
old spot dangling. Rust won't allow it.

## 3. Tiny examples to read

**Slices used safely — read them, *then* you're free.** Predict the output:

```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} / {world}");
}
```

```
hello / world
```

**A function that returns a slice.** The Book's signature example is a function that finds
the first word of some text. What matters here is its *shape*:

```
fn first_word(s: &str) -> &str
```

It takes a view and hands back a **slice of that same text** — `first_word(&sentence)`
returns a `&str` that *borrows* `sentence`. (The scanning implementation inside is a lovely
read — Book §4.3 walks it byte by byte; you don't need it for this lesson's point. And note
you can pass `&sentence` — a borrowed `String` — where a `&str` is wanted: Rust quietly
bridges that gap for you, with the machinery explained in Lesson 29.)

The returned `&str` borrows `sentence` — which is exactly what makes the next part safe.

## 4. Common pitfalls / real compiler errors

**Changing a collection while a slice into it is alive — `E0502`.** This is the payoff. Take
a slice, then try to clear the string while the slice is still in use:

```rust
fn main() {
    let mut s = String::from("hello world");
    let word = &s[0..5];   // word borrows s (immutably)
    s.clear();             // clear needs &mut s — not allowed while word is alive
    println!("the first word was: {word}");
}
```

**Before you scroll — will this compile?** No, and that's the whole point — it's catching a
real bug:

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> main.rs:4:5
  |
3 |     let word = &s[0..5];
  |                 - immutable borrow occurs here
4 |     s.clear();
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("the first word was: {word}");
  |                                    ---- immutable borrow later used here
```

If `first_word` had returned a *number* (an index) instead of a slice, `s.clear()` would
have compiled fine — and `word` would now be an index into an empty string: a silent bug
waiting to bite at run time. Because `word` is a **slice** (a borrow), Rust turns that into
the compile error above.

**The same protection for `Vec` — `E0502`.** A reference into a vector blocks a `push`,
because `push` might move the vector's data:

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let elem = &v[1];   // elem borrows v
    v.push(4);          // push could reallocate v, leaving elem dangling
    println!("elem = {elem}");
}
```

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 …
4 |     v.push(4);
  |     ^^^^^^^^^ mutable borrow occurs here
```

> Same rule as Lesson 16 (shared xor mutable), now with real stakes: the borrow checker is
> stopping you from reading through a window into data that's about to move or vanish.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new slices` works too.)* **Predict on paper before each run.**

1. **Safe slices.** Make a `String`, take two `&str` slices of different parts, and print
   both *before* changing anything. **Predict** the output; confirm it compiles.

2. **Trigger the protection.** Take a slice of a `String`, then call `.clear()` on the
   `String` while still using the slice afterward. **Predict** the error code, and which two
   lines the compiler will point at. Run it.

3. **The same for a `Vec`.** Take a reference to an element of a `Vec`, then `push` onto the
   `Vec`, then use the reference. **Predict**: does this compile? Why would pushing be
   dangerous to an existing reference? Run it and read the message.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. That's Phase 4 — ownership, borrowing, and slices. Next is the Phase-4 review, then
Phase 5: building your own types.)*

## 6. What surprised you?

A sentence or two: did "a slice is a borrow, so it pins the collection" click? Did it change
how you think about the `E0502`s from last lesson — as protection rather than nagging? Tell
me, and I'll shape the Phase-4 review around it.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §4.3 "The Slice Type." The `first_word` arc and
  the payoff: a returned slice borrows the string, so mutating the string while the slice is
  live is a compile error (`E0502`) — "eliminating an entire class of errors at compile
  time." (The mechanics — the `view`, the `..` ranges — were covered in Lesson 13.)
- **CR** — *Comprehensive Rust* (Google), §23.3. The same protection for `Vec`: a reference
  into a vector blocks `push`/reallocation (and iterator-invalidation) with the same `E0502`.
- Slice mechanics carried from Lesson 13; the borrow rules from Lesson 16. Compiler output
  captured on **rustc 1.95.0** (edition 2024). Next: the Phase-4 review (quiz + cheatsheet),
  then Phase 5.

---

<!-- lesson-nav -->
[← Lesson 16b — Mutable references & the borrowing rules](16b-mutable-references.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 18 — Structs: bundle your data →](18-defining-structs.md)
