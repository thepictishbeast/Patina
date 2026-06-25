# Lesson 12 — `String` vs `&str`

*(Phase 3 — Text & collections begins. We delay the deep "who owns what" rules to
Phase 4; here you just need to tell the two kinds of text apart and build one.)*

## 1. Why it exists

Text in Rust comes in two shapes, and beginners trip on this constantly because most
of the time they look the same on screen.

A **string literal** — the `"hello"` you type in your code — is baked right into your
compiled program. It's fixed: you can read it, but you can't grow it or change it. That's
perfect for labels and messages you know ahead of time.

But programs usually need to *build* text while running — join a first and last name,
add to a message in a loop, assemble a line of output. For that you need text you **own
and can grow**: a `String`. This lesson is about telling the two apart — `&str` (a fixed,
borrowed view) versus `String` (owned and growable) — and building a `String`.

> **How the sources frame it:** the **BOOK** sells the split as a *trade-off* (a literal
> is "fast, fixed, baked in"; a `String` is "growable, owned, on the heap"); **CR** gives
> the single clearest runnable example (below); **BLOG** has the one-liner we'll borrow:
> a `&str` is "a **view into a `String`**." (We drop CR's comparisons to other languages'
> string types, per the no-analogy rule.)

## 2. The idea

**`&str` — a borrowed view of text.** Every string literal is a `&str`. The `&` is the
clue: you're looking *at* some text, not owning it. A `&str` is fixed — you can read it
and slice it, but you can't push more onto it. (The deeper meaning of that `&` — borrowing
— is Phase 4; for now, read `&str` as "a window onto text someone else holds.")

**`String` — owned, growable text.** A `String` lives on the heap, and it's yours: you can
add to it and change it. You make one from a literal in either of these ways:

```
let owned = String::from("hello");
let owned = "hello".to_string();
```

…and then grow it: `push_str` adds a `&str` onto the end, `format!` builds a fresh one from
pieces (you met `format!` in Lesson 8), and `+` joins two.

**They connect.** Slicing a `String` gives you a `&str` view back out: `&owned[0..5]` is a
`&str` looking at the first five bytes of `owned`. So the two aren't rivals — a `String`
owns the text; a `&str` is a window into some text (a literal, or a slice of a `String`).

## 3. Tiny examples to read

**All three at once** — a literal `&str`, an owned `String` we grow, and a `&str` sliced
back out. Read it, then predict the three lines:

```rust
fn main() {
    let greeting: &str = "Hello";          // a literal — a fixed, borrowed view
    let mut owned = String::from("Hello");  // owned and growable
    owned.push_str(", world");              // grow it (push_str takes a &str)
    let view: &str = &owned[0..5];          // slice a &str view back out
    println!("greeting = {greeting}");
    println!("owned    = {owned}");
    println!("view     = {view}");
}
```

```
greeting = Hello
owned    = Hello, world
view     = Hello
```

`greeting` never changes (it's a fixed `&str`); `owned` grew because it's a `String`; and
`view` is a `&str` peeking at the first five bytes of `owned`.

**Now a 30-second rep — you type this one.** Build a `String` from pieces with `format!`,
and predict the output:

```rust
fn main() {
    let a = "tic".to_string();
    let b = "tac";
    let c = "toe";
    let combined = format!("{a}-{b}-{c}");
    println!("{combined}");
}
```

```
tic-tac-toe
```

`format!` reads its pieces (a `String` and two `&str`s, mixed freely) and hands back a brand
new `String`. *(That was your write-rep for this part; part 5 is the rest.)*

## 4. Common pitfalls / real compiler errors

**You can't index a string with a number.** It's tempting to reach for "the first character"
with `s[0]`:

```rust
fn main() {
    let s = String::from("hello");
    let first = s[0];
    println!("{first}");
}
```

**Before you scroll — will this compile?** No — and the message explains why text is special.
Real `rustc` (1.95.0), trimmed to the part that teaches:

```
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> main.rs:3:19
  |
3 |     let first = s[0];
  |                   ^ string indices are ranges of `usize`
  |
  = note: you can use `.chars().nth()` or `.bytes().nth()`
```

Rust text is UTF-8, where one "character" can take several bytes, so `s[0]` is ambiguous —
do you mean the first byte or the first character? Rust refuses to guess. The note hands you
the honest tools: `s.chars().nth(0)` for the first character, or slice a *range* of bytes you
know is valid (`&s[0..1]`). You'll meet slices properly in the next lesson.

**A heads-up you'll understand fully in Phase 4.** Joining with `+` works, but it *consumes*
the left string:

```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;   // s1 is moved into s3 here
    println!("{s3}");     // prints: Hello, world!
}
```

This compiles and prints `Hello, world!`. The thing to file away: after `s1 + &s2`, you can't
use `s1` anymore — `+` took ownership of it. *Why* that happens is the whole point of Phase 4;
for now, just know `+` is "use the left one up to build the result," and reach for `format!`
(which doesn't consume its inputs) when you want to keep them.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new strings`. **Predict on paper before each run.**

1. **Build and grow.** Make a `String` from `"Rust"` (your choice of `String::from` or
   `.to_string()`). Then `push_str` a space and another word onto it. Print it. **Predict** the
   final text before you run.

2. **Slice a view.** From your `String`, take a `&str` slice of just the first word
   (`&name[0..4]` style — pick the right range). Print the slice. **Predict** what it shows.

3. **`format!` vs `+`.** Build one sentence two ways: once with `format!` and once with `+`.
   **Predict**: do both print the same thing? Run and confirm.

4. **Trigger the index error.** Try to read `s[0]` on a `String`. **Predict**: compile or
   runtime? which **error code**? Run it, read the note, then get the first character a way the
   compiler actually allows.

*(You write every line here — I won't. The predictions are your answer key; the code is yours.)*

## 6. What surprised you?

A sentence or two: was "a literal can't grow, a `String` can" a clear line, or fuzzy? Did the
"you can't index a string by number" rule surprise you? Tell me, and I'll pitch Lesson 13
(tuples, arrays, slices) to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §4.1 (the literal-vs-`String` trade-off) and §8.2
  "Storing UTF-8 Encoded Text with Strings" (the full build-a-`String` set: `from`/`to_string`/
  `push_str`/`+`/`format!`, and the no-integer-indexing rule with the UTF-8 reason).
- **CR** — *Comprehensive Rust* (Google), §9.4. Backbone of the part-3 example (literal →
  `String::from` → `push_str` → slice a `&str` back out). Its comparisons to other languages'
  string types were dropped per the no-analogy rule.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), §1.4 "String." Cited for the framing a `&str`
  is "a view into a `String`."
- Compiler output captured live on **rustc 1.95.0** (edition 2024); the `+`-moves-`s1` detail is
  flagged as a Phase-4 forward reference, not taught here.

---

<!-- lesson-nav -->
[← Lesson 11 — `match` (intro)](11-match-intro.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 13 — Tuples, Arrays, and Slices →](13-tuples-arrays-slices.md)
