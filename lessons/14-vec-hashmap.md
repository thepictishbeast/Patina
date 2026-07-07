# Lesson 14 — `Vec` and `HashMap`

*(Phase 3 — Text & collections, part 3 and last. Lesson 13's tuples and arrays were
**fixed** in size. These two **grow**: a list that adds items as you go, and a lookup
table from keys to values.)*

## 1. Why it exists

An array's length is locked in at compile time (Lesson 13). But most real data arrives
a bit at a time — you read lines, collect results, tally things up — and you don't know
how many there'll be. For that you need a **growable** list: `Vec<T>` ("vector").

And sometimes "the third item" is the wrong question; you want "the value *for this
name*" — a score for a team, a count for a word. That's a **`HashMap<K, V>`**: a table
that maps **keys** to **values**.

> **How the sources frame it:** the **BOOK** owns both — it sells `Vec` as the growable
> answer to the fixed array and makes `[]`-vs-`.get()` a *design choice you make*, and its
> word-count is the canonical `HashMap` showpiece; **CR** adds the clearest `entry`
> explanation and the import gotcha; **BLOG** has a tidy `Vec` demo but skips `HashMap`
> entirely.

## 2. The idea

**`Vec<T>` — a growable list of one type.** Make one and add to it:

```
let mut v = vec![100, 32, 57];   // start with some values
v.push(7);                        // grow it by one
let last = v.pop();               // remove & return the last → an Option
```

Read an element two ways, and **this is a real choice**:

- `v[2]` gives you the value directly — but if the index is out of range it **panics**.
- `v.get(2)` gives you an `Option` (Lesson 11's idea) — `Some(&value)` if it's there,
  `None` if not. No crash; you handle the "not there" case.

So: use `[]` when a bad index is genuinely a bug that *should* stop the program, and
`.get()` when "might not be there" is a normal case to handle. To **change** elements,
loop over `&mut v` and use `*` to reach the value (the `*` is "the thing the reference
points at"):

```
for n in &mut v { *n += 50; }
```

**`HashMap<K, V>` — a lookup table.** It's **not** in the prelude, so the first line is
always:

```
use std::collections::HashMap;
```

Then `insert` a key→value, and `get` a key back (which returns an `Option`, because the
key might not be present). The standout move is the **`entry` API** for "count or
update": `*map.entry(k).or_insert(0) += 1` means *get the slot for `k`, putting `0`
there first if it's missing, then add 1*. One line, and it's how you tally things.

(Order note: a `HashMap` does **not** remember insertion order — printing one gives the
pairs in an arbitrary order. It's a lookup table, not a list.)

## 3. Tiny examples to read

**`Vec` — read, then change in place.** Predict both outputs:

```rust
fn main() {
    let mut v = vec![100, 32, 57];
    for n in &v { println!("{n}"); }   // read each
    for n in &mut v { *n += 50; }       // change each (* reaches the value)
    println!("{v:?}");
}
```

```
100
32
57
[150, 82, 107]
```

**`[]` vs `.get()` — you type this one (30-second rep).** Predict it:

```rust
fn main() {
    let v = vec![10, 20, 30];
    let third: &i32 = &v[2];   // direct — would panic if out of range
    let tenth = v.get(10);      // safe — an Option
    println!("third = {third}, tenth = {tenth:?}");
}
```

```
third = 30, tenth = None
```

**`HashMap` — the word counter (the showpiece).** Predict the *counts* (not the order):

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

`split_whitespace` hands over each word; `entry(word).or_insert(0)` makes sure there's a
counter (starting at `0` the first time a word appears); `*… += 1` bumps it. *(That `[]`
vs `.get()` snippet was your write-rep; part 5 is the rest.)*

## 4. Common pitfalls / real compiler errors

**Forgetting the `HashMap` import.** This is the single most common beginner stumble,
because `HashMap` isn't in scope by default:

```rust
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    println!("{scores:?}");
}
```

**Will this compile?** No — and the compiler hands you the exact fix. Real `rustc`
(1.95.0):

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

Add the `use std::collections::HashMap;` line it suggests, at the top.

**Indexing a `Vec` past the end.** Unlike an array (Lesson 13), a `Vec`'s length isn't
known at compile time, so the compiler *can't* catch a bad index early — the check is
always at run time, and a bad index **panics**:

```rust
fn main() {
    let v = vec![10, 20, 30];
    let x = v[100];
    println!("{x}");
}
```

```
thread 'main' panicked at main.rs:3:14:
index out of bounds: the len is 3 but the index is 100
```

This is exactly why `.get()` exists: `v.get(100)` would calmly return `None` instead of
crashing. Reach for `.get()` whenever the index might legitimately be out of range.

> *(One thing you'll meet in Phase 4: inserting an owned `String` as a key **moves** it
> into the map — `i32`s and `&str`s used here don't have that wrinkle. Why that happens
> is ownership, coming next phase.)*

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new collections2` works too.)* **Predict on paper before each run.**

1. **Build and sum a `Vec`.** Start an empty `Vec<i32>` (`Vec::new()`), `push` three
   numbers onto it, then loop over `&v` and add them into a `total`. Print the `Vec` with
   `{:?}` and the total. **Predict** both.

2. **`[]` vs `.get()`.** On a 3-element `Vec`, print `v.get(1)` and `v.get(9)`. **Predict**
   each (what does an in-range vs out-of-range `.get()` return?). Then try `v[9]` and
   **predict**: compile error or runtime panic?

3. **Count with a `HashMap`.** Import `HashMap`, then count the words in a sentence of your
   own using `*map.entry(word).or_insert(0) += 1`. Print it. **Predict** the counts (don't
   worry about order). Bonus: read one word's count back with `.get(word).copied().unwrap_or(0)`.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. This is the last Phase-3 lesson — next comes the Phase-3 review, then Phase 4:
ownership, the idea quietly behind a lot of what you've seen.)*

## 6. What surprised you?

A sentence or two: did the `[]`-vs-`.get()` choice make sense as *your* decision? Did the
`entry`/`or_insert` counter click, or feel like magic? Tell me, and I'll shape the Phase-3
review around it.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §8.1 "Storing Lists of Values with Vectors"
  and §8.3 "Storing Keys with Associated Values in Hash Maps." Backbone for both: `Vec` as
  the growable array, `[]`-vs-`.get()` as a deliberate choice, and the `entry().or_insert()`
  word-count.
- **CR** — *Comprehensive Rust* (Google), §17.6–17.7. The `entry().or_insert()` vs
  `get().unwrap_or()` distinction (insert-or-update vs read-with-fallback) and the
  not-in-the-prelude import gotcha.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Other types → Vector." The push/get/len
  `Vec` demo; it explicitly skips hash maps, so `HashMap` is sourced from BOOK/CR.
- Compiler/runtime output captured on **rustc 1.95.0** (edition 2024). The `String`-key
  *move* on insert is flagged as a Phase-4 forward reference, not taught here.

---

<!-- lesson-nav -->
[← Lesson 13 — Tuples, Arrays, and Slices](13-tuples-arrays-slices.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 15 — Ownership & Moves →](15-ownership-and-moves.md)
