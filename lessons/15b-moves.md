# Lesson 15b — Moves: assignment hands ownership over

*(Phase 4 — Ownership, part 2. Lesson 15's rule 2 said one owner at a time.
This lesson is what that rule DOES: assign a `String` to a second name and
the first name is torn up. Meet Rust's most famous error on purpose, today,
and it will never ambush you again.)*

## 1. Why it exists

Rule 2 — one owner at a time — has to survive this innocent-looking line:

```
let s1 = String::from("hello");
let s2 = s1;
```

If both `s1` and `s2` owned the text, both would free it at the end of the
block — the exact double-free bug ownership exists to kill. So Rust does
something decisive instead: the assignment **moves** ownership.

## 2. The idea

**The owner card.** Assigning a `String` does **not** copy the heap text. It
hands the little record (the "owner card") from `s1` to `s2` — and to keep
rule 2 true, `s1` is now treated as **invalid**. Picture handing your card to
someone and tearing up your copy: now only `s2` can use the value, and only
`s2` will free it — so the memory can never be freed twice.

**Functions move, too.** Passing a value to a function hands the card over
exactly like assignment does — the parameter becomes the owner, and your
name is torn up at the call site. Part 4 shows both walls.

Using the *old* name after either kind of move is a compile error — the
famous one. And note what is **not** an error: moving and then simply never
touching the old name. The move itself is fine; it's the *use after* that
the compiler stops.

## 3. A tiny example to read

**A move that compiles fine.** Only the new owner is used afterward:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;               // the owner card changes hands
    println!("s2 = {s2}");     // only the NEW owner is used — fine
}
```

```
s2 = hello
```

No error, no clone, no ceremony — a move is the *normal* way values travel.
The next section is what happens when you reach for the torn-up card.

## 4. Common pitfalls / real compiler errors

**Using a value after it moved — `E0382`.** This is *the* ownership error —
meet it now and it'll never confuse you again:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 = {s2}, s1 = {s1}");
}
```

**Before you scroll — will this compile?** No. `s1` moved into `s2`, so the
later use of `s1` is using a value that's no longer yours. Real `rustc`
(1.95.0):

```
error[E0382]: borrow of moved value: `s1`
 --> main.rs:4:32
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("s2 = {s2}, s1 = {s1}");
  |                                ^^ value borrowed here after move
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++
```

Read it top to bottom — it narrates the whole story: *move occurs because
`String` isn't `Copy`* → *value moved here* → *value borrowed here after
move* → and a suggested fix, `.clone()`. Two threads to pull on next lesson:
what is this `Copy` the message keeps mentioning, and what does `.clone()`
really cost? Both are **Lesson 15c**.

**Passing to a function moves, too.** A function parameter takes ownership
the same way an assignment does:

```rust
fn takes_ownership(text: String) {
    println!("got {text}");
}
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    println!("{s}");   // s was moved INTO the function
}
```

```
error[E0382]: borrow of moved value: `s`
…
note: consider changing this parameter type in function `takes_ownership` to borrow
      instead if owning the value isn't necessary
```

Notice the compiler's hint this time: *borrow instead*. Handing whole values
in and out (or cloning everything) gets clumsy fast. The clean answer —
letting a function **look at** a value without taking it — is **borrowing**,
Lesson 16, right after Copy and Clone complete the picture.

> The borrow checker isn't being difficult here. It's pointing at a real
> bug — a value used after it was given away — at build time, with the exact
> line, instead of letting it crash later.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new ownership` works too.)* **Predict on
paper before each run.**

1. **A move that's fine.** Create a `String`, bind it to a second name, and
   print only the *second* name. **Predict**: does it compile? What single
   fact makes this legal?
2. **Make the famous error.** Now print the *first* name too. **Predict the
   error code** and the three story-beats the message will narrate (why the
   move happened, where, and where the bad use is). Run it and read top to
   bottom.
3. **The function wall.** Write a function taking a `String` parameter, call
   it, then use your variable after the call. **Predict**: same error code
   or different — and what does the compiler suggest *this* time (read its
   note carefully — it's previewing Lesson 16)?

*(You write every line here — I won't. The predictions are your answer key;
the code is yours. Next, Lesson 15c: why numbers never hit this wall — and
the honest way to ask for a real copy.)*

## 6. What surprised you?

A sentence or two: did "assigning a `String` *moves* it" feel strange, or
did the owner-card picture make it click? Did it surprise you that the move
itself is fine — only the use-after is stopped? Tell me, and I'll pitch
Lesson 15c to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§4.1**: the `String` move,
  the `E0382` walkthrough, and functions taking ownership (Listing 4-3
  territory).
- **CR** — *Comprehensive Rust* (Google), §20: the "owner card" picture of a
  move (hand the card over, tear up your copy).
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Ownership": states the
  rules in prose; the runnable demos come from BOOK/CR.
- Compiler output captured live on **rustc 1.95.0** (edition 2024;
  `rustc --edition 2024 FILE.rs`, temp paths normalized to `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 15 — Ownership, scope & drop](15-ownership.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 15c — Copy & Clone: when assignment duplicates →](15c-copy-and-clone.md)
