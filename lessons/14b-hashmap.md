# Lesson 14b — `HashMap`: the lookup table

*(Phase 3 — Text & collections, part 6 and last. A `Vec` answers "what's the
third item?" Sometimes that's the wrong question — you want "the value *for
this name*": a score for a team, a count for a word. That's a lookup table,
and it comes with the course's most common beginner stumble built in.)*

## 1. Why it exists

Position is the wrong handle for a lot of data. When you tally words, you
don't care which word arrived third — you care what count belongs to `"cat"`.
A **`HashMap<K, V>`** maps **keys** to **values**: hand it a key, get the
value back, no positions anywhere.

## 2. The idea

**It's not in the prelude.** Unlike `Vec` and `String`, `HashMap` isn't in
scope by default, so the first line of any program using it is:

```
use std::collections::HashMap;
```

(That `use` keyword is Lesson 23's topic — bringing names into scope. Until
then, treat this line as a fixed incantation to copy exactly, at the top.
Part 4 shows precisely what happens when you forget it — and how the compiler
hands you the line back.)

**Insert and look up:**

- `map.insert(key, value)` stores a pair.
- `map.get(&key)` hands back an **`Option`** — `Some(&value)` if the key is
  present, `None` if not. The same "might not be there" honesty as `Vec`'s
  `.get()` and `pop`.

**The standout move — the `entry` API for "count or update":**

```
*counts.entry(word).or_insert(0) += 1
```

Read it inside-out: *get the slot for `word`, putting `0` there first if it's
missing, then add 1.* One line, and it's how you tally anything.

**Order note:** a `HashMap` does **not** remember insertion order — printing
one gives the pairs in an arbitrary order. It's a lookup table, not a list.

## 3. A tiny example to read — the word counter (the showpiece)

Predict the *counts* (not the order):

```rust
use std::collections::HashMap;
fn main() {
    let text = "the cat the dog the cat";
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("{counts:?}");
}
```

One possible run (your order may differ — a `HashMap` is unordered):

```
{"dog": 1, "the": 3, "cat": 2}
```

`split_whitespace` hands over each word; `entry(word).or_insert(0)` makes
sure there's a counter (starting at `0` the first time a word appears);
`*… += 1` bumps it.

## 4. Common pitfalls / real compiler errors

**Forgetting the import — `E0433`.** The single most common beginner stumble
with `HashMap`:

```rust
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    println!("{scores:?}");
}
```

**Will this compile?** No — and the compiler hands you the exact fix. Real
`rustc` (1.95.0):

```
error[E0433]: cannot find type `HashMap` in this scope
 --> main.rs:2:22
  |
2 |     let mut scores = HashMap::new();
  |                      ^^^^^^^ use of undeclared type `HashMap`
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |
```

Add the `use std::collections::HashMap;` line it suggests, at the top. (This
is also a preview of how imports fail in general — when Lesson 23 teaches
`use` properly, `E0433` will already be an old acquaintance.)

One more wrinkle to know *about*: inserting an owned `String` as a key
**moves** it into the map. The `&str` and `i32` keys used here don't have
that wrinkle — the why is ownership, Phase 4's topic, and the details are
**Book §8.3** when you get there.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new collections2` works too.)* **Predict on
paper before each run.**

1. **Stumble on purpose.** Write the scores program from part 4 *without* the
   import. **Predict the error code** — then apply the compiler's suggested
   line and confirm it runs.
2. **Count with a `HashMap`.** Count the words in a sentence of your own
   using `*map.entry(word).or_insert(0) += 1`. Print the whole map.
   **Predict** the counts (don't worry about order). Run it twice — does the
   printed *order* stay the same? Should it?
3. **Look one key up.** After counting, print `map.get("the")` and
   `map.get("zebra")` with `{:?}`. **Predict** both — same `Some`/`None`
   honesty as `Vec`'s `.get()`, now keyed by name instead of position.

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. That closes Phase 3 — text and collections. Next comes the
Phase-3 review, then Phase 4: ownership, the idea quietly behind a lot of
what you've seen — including that `String`-key wrinkle above.)*

## 6. What surprised you?

A sentence or two: did the `entry`/`or_insert` counter click, or feel like
magic? Did the unordered printing surprise you the second time you ran it?
Tell me, and I'll shape the Phase-3 review around it.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§8.3** "Storing Keys with
  Associated Values in Hash Maps": the word-count showpiece, `get` returning
  `Option`, and the String-key ownership wrinkle this lesson points at rather
  than teaches.
- **CR** — *Comprehensive Rust* (Google), §17.6–17.7: the clearest `entry`
  explanation and the not-in-the-prelude import gotcha.
- **BLOG** — *Rust for Beginners* explicitly skips hash maps, so this lesson
  is sourced from BOOK/CR.
- Compiler output captured on **rustc 1.95.0** (edition 2024; temp paths
  normalized to `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 14 — Vec: the growable list](14-vec.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 15 — Ownership, scope & drop →](15-ownership.md)
