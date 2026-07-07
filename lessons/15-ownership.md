# Lesson 15 — Ownership, scope & drop

*(Phase 4 — Ownership begins. This is the idea quietly behind a lot of what
you've already seen — the `&` in `&str`, and the "this value was moved" hints
we kept flagging in Phase 3. It's the conceptual heart of Rust, so it gets
three lessons: the rules (here), moves (15b), and Copy/Clone (15c). Take it
slowly.)*

## 1. Why it exists

Every program has to give memory back when it's done with it. Some languages
do this with a garbage collector that runs in the background; others make you
free it by hand (and punish you with crashes if you free the wrong thing
twice, or use it after freeing). Rust takes a third path: **ownership**. Every
value has exactly one **owner**, and when that owner goes out of scope, Rust
frees the value **automatically** — no collector, no manual `free`, and the
compiler proves at build time that you never use a value after it's gone. You
get safety *and* speed, which is the whole point of Rust.

## 2. The idea

**One sentence of memory model** (all this lesson needs): small fixed-size
values — an `i32`, a `bool`, a `char` — are cheap to store and copy; a
growable value like a `String` keeps its character data elsewhere, and what
the variable holds is a small record saying where to find it. (The fuller
stack-and-heap mechanics — the pointer/length/capacity record, the
stack-of-plates picture — are **Book §4.1**, worth reading once; nothing
below depends on more than the sentence above.)

**The three rules of ownership** (straight from the Book — memorize these):

1. Each value in Rust has an **owner**.
2. There can be only **one owner at a time**.
3. When the owner goes **out of scope**, the value is **dropped** (its memory
   is freed).

**Drop, concretely.** Rule 3 in action: when an owner reaches the end of its
`{ }` block, Rust automatically frees its value right then. You never write
the cleanup, and it never happens twice. You already know from Lesson 3 that
a *name* lives only inside its block — rule 3 adds that the *value* is freed
at the same brace. Name and memory expire together.

(Rule 2 is the one with teeth — what happens when you *assign* a `String` to
a second name, and only one owner is allowed? That's Lesson 15b, and it's the
rule that surprises everyone.)

## 3. A tiny example to read

**Drop at the end of a scope.** Predict both lines:

```rust
fn main() {
    let outer = String::from("outer");
    {
        let inner = String::from("inner");
        println!("inside the block: {inner}");
    } // <- `inner` goes out of scope here; Rust frees it automatically
    println!("after the block: {outer}");
}
```

```
inside the block: inner
after the block: outer
```

After the inner block, `inner` is gone (freed) and unusable — `outer` lives
on until `main` ends. No `free`, no leak.

## 4. Common pitfalls / real compiler errors

**Using a value after its scope closed — `E0425`.** Try to print `inner`
*after* the block:

```rust
fn main() {
    {
        let inner = String::from("inner");
        println!("inside the block: {inner}");
    } // <- `inner` is dropped (freed) right here
    println!("after the block: {inner}");
}
```

**Before you scroll — you met this error code with loop variables in
Lesson 10c. What will it say here?**

```
error[E0425]: cannot find value `inner` in this scope
 --> main.rs:6:33
  |
6 |     println!("after the block: {inner}");
  |                                 ^^^^^
  |
help: the binding `inner` is available in a different scope in the same function
  |
3 |         let inner = String::from("inner");
  |             ^^^^^
```

Read the help line: "available in a **different scope**" — the compiler knows
exactly where `inner` lived and that you're past its brace. Two rules point
at the same spot: Lesson 3's *scope* says the name is gone; ownership rule 3
says the memory went with it. The out-of-scope value isn't just unnameable —
it no longer exists.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new ownership` works too.)* **Predict on paper before
each run.**

1. **Rules on paper first.** Without looking back, write the three rules of
   ownership in your own words. Check against part 2 — the one you fumbled
   tells you what to reread.
2. **Watch a drop.** Make an inner `{ }` block that builds a `String` and
   prints it, then print a *different*, outer `String` after the block (the
   part-3 shape, your own values). **Predict** both lines.
3. **Reach past the brace.** Now print the *inner* `String` after the block
   instead. **Predict the error code** — and read the help line: what does
   "a different scope" tell you about *when* the value was freed?

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next, Lesson 15b: rule 2 with teeth — what assignment
does to a `String`, and the most famous error in Rust.)*

## 6. What surprised you?

A sentence or two: did "no collector, no manual free — the brace does it"
land as a third way of handling memory? Did tying rule 3 to Lesson 3's scope
rule make drop feel familiar rather than new? Tell me, and I'll pitch
Lesson 15b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§4.1 "What Is Ownership?"**:
  the three rules verbatim, drop-at-end-of-scope, the bug ownership prevents
  (freeing memory twice), and the stack/heap mechanics this lesson compresses
  to one sentence + a pointer.
- **CR** — *Comprehensive Rust* (Google), §20: the scope-and-drop framing.
  Its tour of other languages' memory models was dropped per the no-analogy
  rule.
- Compiler output captured live on **rustc 1.95.0** (edition 2024;
  `rustc --edition 2024 FILE.rs`, temp paths normalized to `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 14b — HashMap: the lookup table](14b-hashmap.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 15b — Moves: assignment hands ownership over →](15b-moves.md)
