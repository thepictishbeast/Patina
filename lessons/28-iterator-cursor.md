# Lesson 28 — Iterators: the `next()` cursor

*(Phase 7 — Functional features & smart pointers, part 3. Back in Lesson 10 you
wrote `for item in collection { … }` and it just worked. This lesson opens that
box: a `for` loop is asking the collection for an **iterator** — a little cursor
that hands out one item at a time.)*

## 1. Why it exists

Every walk over a collection — printing items, summing them, transforming them —
needs the same little machine underneath: something that knows *what comes next*
and *when to stop*. That machine is the iterator, and once you can name it, you
can also choose **how** it accesses the collection: read each item, change each
item, or take each item away. That choice is ownership (Lessons 15–16b) all over
again.

## 2. The idea

**An iterator is a cursor.** It has one job, captured by the `Iterator` trait:
hand back the next item when asked. The method that does it is `next`:

```
fn next(&mut self) -> Option<Self::Item>
```

Each call returns `Some(item)` for the next value, and `None` once there's
nothing left (you know `Option` from Lesson 19b). It takes `&mut self` because
asking for the next item *advances* the cursor — so the iterator has to be `mut`.

**Iterators are lazy.** Making an iterator does *no work* — it's a **plan, not a
result**. Nothing happens until something pulls items out (a `for` loop, or the
consumers of the next lesson). File that rule away now; Lesson 28b makes it
visible.

**Three ways to ask a collection for its iterator** — the difference is *what
you get to touch*:

- **`.iter()`** — borrows each item, hands you `&T` (read-only). The collection
  survives.
- **`.iter_mut()`** — borrows each item mutably, hands you `&mut T` (change
  items in place). The collection survives.
- **`.into_iter()`** — *takes ownership*, hands you each `T` by value. The
  collection is consumed and gone afterward.

And the tie-back to Lesson 10: a `for` loop picks one of these for you based on
how you write it. `for x in &v` uses `iter`, `for x in &mut v` uses `iter_mut`,
and `for x in v` uses `into_iter` (and eats `v`).

## 3. Tiny examples to read

**`next` walks the cursor by hand.** A `for` loop calls `next` for you; here we
do it ourselves so you can see the `Some`/`None`. Note `it` must be `mut`:

```rust
fn main() {
    let small = vec![1, 2, 3];
    let mut it = small.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}
```

```
Some(1)
Some(2)
Some(3)
None
```

Each call moved the cursor forward one step; after the third item it's
exhausted, so the fourth call returns `None` — the same `None` a `for` loop
watches for to know it's done.

**`iter` vs `iter_mut` vs `into_iter` — the three accesses.** Same vector, three
flavours. `iter_mut` lets you change items in place (`*x` to reach through the
`&mut`):

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    for x in v.iter_mut() {
        *x += 10;          // change each item through &mut
    }
    println!("{v:?}");     // v survived, now changed

    let total: i32 = v.iter().sum();  // borrow each &i32 to read
    println!("{total}");

    let owned: Vec<i32> = v.into_iter().collect();  // takes ownership of v
    println!("{owned:?}");
    // v is gone now — into_iter consumed it
}
```

```
[11, 12, 13]
36
[11, 12, 13]
```

(`sum` and `collect` are *consumers* — they drive the cursor to the end. They
get their own lesson next; here just see the three accesses.)

## 4. Common pitfalls / real compiler errors — after `into_iter`, the collection is gone

`into_iter` takes ownership, so touching the original afterward is the Lesson-15
move error:

```rust
fn main() {
    let v = vec![1, 2, 3];
    let _doubled: Vec<i32> = v.into_iter().map(|x| x * 2).collect();
    println!("{v:?}");
}
```

```
error[E0382]: borrow of moved value: `v`
 --> main.rs:4:16
  |
2 |     let v = vec![1, 2, 3];
  |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
3 |     let _doubled: Vec<i32> = v.into_iter().map(|x| x * 2).collect();
  |                                ----------- `v` moved due to this method call
4 |     println!("{v:?}");
  |                ^ value borrowed here after move
  |
note: `into_iter` takes ownership of the receiver `self`, which moves `v`
```

The note names the exact cause. If you need `v` again afterward, you wanted
`.iter()` — a borrow — and the last line compiles fine. Both matching exercises
below are flavours of this wall: **predict each code** before you run.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the two matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new iterators` works too.)* **Predict on paper
before each run.**

1. **Walk the cursor.** Make a `Vec` of three strings. Get an iterator with
   `.iter()`, bind it `mut`, call `.next()` four times printing each with
   `{:?}`. **Predict** all four lines — especially the fourth. Then remove the
   `mut` and **predict** what the compiler says.
2. **The three accesses, side by side.** With a `mut` vector: `.iter_mut()` in a
   `for` loop to add 1 to every element; then a `for x in &v` to print them;
   then `for x in v` (which eats it) — and after that, one more use of `v`.
   **Predict the error code**, and which loop head you'd change so it compiles.

*(You write every line here — I won't. The predictions are your answer key.
Next: stacking steps on the cursor — `map`, `filter`, and the consumers that
drive them.)*

## 6. What surprised you?

A sentence or two: did "an iterator is just a cursor" change how Lesson 10's
`for` loop looks now? Did `iter`/`iter_mut`/`into_iter` land as "borrow / borrow
mutably / take ownership"? Tell me, and I'll pitch Lesson 28b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.13.2**: the `Iterator` trait,
  `next` returning `Some`/`None`, and the laziness rule.
- **CR** — *Comprehensive Rust* (Google), §26.
- Ties back to **Lesson 10** (`for` loops) and **Lesson 19b** (`Option`).
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 27b — Closure capture & move](27b-closure-capture.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 28b — Adapter chains: map, filter, collect →](28b-adapter-chains.md)
