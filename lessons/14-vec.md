# Lesson 14 — `Vec`: the growable list

*(Phase 3 — Text & collections, part 5. Lessons 13–13b's tuples and arrays
were **fixed** in size. `Vec` **grows** — and growing moves one safety check
you know from arrays permanently to run time. Its partner, the lookup table,
is Lesson 14b.)*

## 1. Why it exists

An array's length is locked in at compile time (Lesson 13b). But most real
data arrives a bit at a time — you read lines, collect results, tally things
up — and you don't know how many there'll be. For that you need a
**growable** list: `Vec<T>` ("vector"). Same idea as an array — one type, a
row of values, 0-based indexing — but the length lives at run time and can
change.

## 2. The idea

**Make one and add to it:**

```
let mut v = vec![100, 32, 57];   // start with some values
v.push(7);                        // grow it by one
let last = v.pop();               // remove & return the last → an Option
```

`pop` returns an **`Option`** (Lesson 11's idea) — `Some(value)` if there was
a last element, `None` if the `Vec` was empty. The "might not be there" case
is in the type, not left for you to remember.

**Read an element two ways — and this is a real choice:**

- `v[2]` gives you the value directly — but if the index is out of range it
  **panics**.
- `v.get(2)` gives you an `Option` — `Some(&value)` if it's there, `None` if
  not. No crash; you handle the "not there" case.

So: use `[]` when a bad index is genuinely a bug that *should* stop the
program, and `.get()` when "might not be there" is a normal case to handle.

(To **change** elements in place you loop with `&mut` and a `*` dereference —
tools that belong to Phase 4's borrowing lessons. Until then, build a new
`Vec` instead; when you're curious, the in-place loop is in **Book §8.1
"Iterating over the Values in a Vector."**)

## 3. Tiny examples to read

**Grow, then shrink.** Predict both lines — especially the *shape* of `last`:

```rust
fn main() {
    let mut v = vec![100, 32, 57];
    v.push(7);
    println!("{v:?}");
    let last = v.pop();
    println!("last = {last:?}, now {v:?}");
}
```

```
[100, 32, 57, 7]
last = Some(7), now [100, 32, 57]
```

`pop` didn't hand back `7` — it handed back `Some(7)`, because popping an
*empty* `Vec` must hand back something too (`None`), and one return type has
to cover both runs.

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

## 4. Common pitfalls / real panics

**Indexing a `Vec` past the end.** Unlike an array (Lesson 13b), a `Vec`'s
length isn't known at compile time, so the compiler *can't* catch a bad index
early — the check is **always** at run time, and a bad index **panics**:

```rust
fn main() {
    let v = vec![10, 20, 30];
    let x = v[100];
    println!("{x}");
}
```

**Before you scroll — with an array, a literal `[100]` was refused at build
time. Will it be here?**

```
thread 'main' panicked at main.rs:3:14:
index out of bounds: the len is 3 but the index is 100
```

No — it compiles and crashes. Growable length means the compiler has nothing
to check against; 13b's "caught at two times" collapses to one time: run
time. This is exactly why `.get()` exists: `v.get(100)` would calmly return
`None` instead of crashing. Reach for `.get()` whenever the index might
legitimately be out of range.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new collections2` works too.)* **Predict on
paper before each run.**

1. **Build and sum a `Vec`.** Start an empty `Vec<i32>` (`Vec::new()`),
   `push` three numbers onto it, then loop over `&v` (a borrowed view, like
   13c's slices) and add them into a `total`. Print the `Vec` with `{:?}` and
   the total. **Predict** both.
2. **`[]` vs `.get()`.** On a 3-element `Vec`, print `v.get(1)` and
   `v.get(9)`. **Predict** each (what does an in-range vs out-of-range
   `.get()` return?). Then try `v[9]` and **predict**: compile error or
   runtime panic — and why can't it be the array answer?
3. **Pop till empty.** `pop` your 3-element `Vec` **four** times, printing
   each result with `{:?}`. **Predict** all four lines — what does the fourth
   `pop` return, and why is that better than a crash?

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next, Lesson 14b: when "the third item" is the wrong
question and you want "the value for this NAME" — the lookup table.)*

## 6. What surprised you?

A sentence or two: did the `[]`-vs-`.get()` choice make sense as *your*
decision? Did `pop` returning `Some(7)` instead of `7` feel strange or
reassuring? Tell me, and I'll pitch the `HashMap` lesson to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§8.1** "Storing Lists of
  Values with Vectors": `Vec` as the growable array, `[]`-vs-`.get()` as a
  deliberate choice, and the in-place `&mut` iteration this lesson defers to
  Phase 4 (pointed at, not taught).
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Other types → Vector":
  the push/get/len demo.
- Compiler/runtime output captured on **rustc 1.95.0** (edition 2024; temp
  paths and run-specific thread ids normalized).

---

<!-- lesson-nav -->
[← Lesson 13c — Slices: a borrowed window](13c-slices.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 14b — HashMap: the lookup table →](14b-hashmap.md)
