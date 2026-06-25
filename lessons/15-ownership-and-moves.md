# Lesson 15 — Ownership & Moves

*(Phase 4 — Ownership begins. This is the idea quietly behind a lot of what you've
already seen — the `&` in `&str`, and the "this value was moved" hints we kept
flagging in Phase 3. It's the conceptual heart of Rust, so take it slowly.)*

## 1. Why it exists

Every program has to give memory back when it's done with it. Some languages do this
with a garbage collector that runs in the background; others make you free it by hand
(and punish you with crashes if you free the wrong thing twice, or use it after
freeing). Rust takes a third path: **ownership**. Every value has exactly one **owner**,
and when that owner goes out of scope, Rust frees the value **automatically** — no
collector, no manual `free`, and the compiler proves at build time that you never use a
value after it's gone. You get safety *and* speed, which is the whole point of Rust.

> **How the sources frame it:** the **BOOK** builds the full model in order (stack/heap →
> the three rules → moves → the `E0382` error) and names the bug it kills (freeing the
> same memory twice); **CR** has the sharpest `Copy`-vs-move contrast and the "owner card"
> picture. (Comparisons to how other languages manage memory are dropped — we explain it
> on its own terms.)

## 2. The idea

**Stack and heap (just enough).** Small fixed-size values — an `i32`, a `bool`, a
`char` — sit on the **stack**: a tidy pile, fast to push and pop (think a stack of
plates). A growable value like a `String` keeps its characters on the **heap** (a big
shared area); what lives on the stack is a little record — a pointer, a length, a
capacity — that says *where* on the heap to look.

**The three rules of ownership** (straight from the Book — memorize these):

1. Each value in Rust has an **owner**.
2. There can be only **one owner at a time**.
3. When the owner goes **out of scope**, the value is **dropped** (its memory is freed).

**Move.** Here's the rule that surprises everyone. For a `String`:

```
let s1 = String::from("hello");
let s2 = s1;
```

This does **not** copy the heap text. It hands the little stack record (the "owner
card") from `s1` to `s2` — and to keep rule 2 true (one owner), `s1` is now treated as
**invalid**. Picture handing your card to someone and tearing up your copy: now only
`s2` can use the value, and only `s2` will free it (so the memory can never be freed
twice). Trying to use `s1` afterward is a compile error — you'll see it in part 4.

**Copy vs Clone.** Then why didn't `let y = x;` break for numbers? Because small
stack-only types — integers, `bool`, `char`, floats, and tuples of those — are **`Copy`**:
assigning them makes a cheap duplicate and **both stay valid**, no move. A `String` isn't
`Copy` (copying heap data isn't free), so it moves instead. If you genuinely want a second,
independent `String`, ask for it out loud with **`.clone()`** — a deliberate, visible cost.
(A type can be `Copy` *or* have custom cleanup on drop, never both.)

**Drop.** Rule 3 in action: when an owner reaches the end of its `{ }` block, Rust
automatically frees its value right then. You never write the cleanup, and it never
happens twice.

## 3. Tiny examples to read

**Numbers are `Copy` — no move.** Predict it:

```rust
fn main() {
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");   // x is STILL valid
}
```

```
x = 5, y = 5
```

**A `String` you want twice — clone it (30-second rep, you type this).** Predict:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();   // deep copy, on purpose
    println!("s1 = {s1}, s2 = {s2}");
}
```

```
s1 = hello, s2 = hello
```

Both are valid because `.clone()` made a second, independent `String` — and you can *see*
the cost, right there in the code. *(That was your write-rep; part 5 is the rest.)*

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

After the inner block, `inner` is gone (freed) and unusable — `outer` lives on until
`main` ends. No `free`, no leak.

## 4. Common pitfalls / real compiler errors

**Using a value after it moved.** This is *the* ownership error, `E0382` — meet it now
and it'll never confuse you again:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 = {s2}, s1 = {s1}");
}
```

**Before you scroll — will this compile?** No. `s1` moved into `s2`, so the later use of
`s1` is using a value that's no longer yours. Real `rustc` (1.95.0):

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

Read it top to bottom — it narrates the whole story: *move occurs because `String` isn't
`Copy`* → *value moved here* → *value borrowed here after move* → and the fix: `.clone()`
if you really need both. (If you didn't need `s1` afterward, you'd just… not use it — no
error.)

**Passing to a function moves, too.** A function parameter takes ownership the same way an
assignment does:

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

Notice the compiler's hint: *borrow instead*. Cloning works, but constantly handing whole
values in and out (or cloning everything) is clumsy. The clean answer — letting a function
**look at** a value without taking it — is **borrowing**, the very next lesson.

> The borrow checker isn't being difficult here. It's pointing at a real bug — a value used
> after it was given away — at build time, with the exact line, instead of letting it crash
> later.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new ownership`. **Predict on paper before each run.**

1. **Make the move error.** Create a `String`, bind it to a second name, then print the
   *first* name. **Predict** the error code and what the compiler will suggest. Run it.

2. **Fix it two ways.** First fix it with the compiler's suggestion (`.clone()`) and print
   both. Then *delete* the clone and instead just don't use the first name after the move —
   **predict** whether *that* compiles too, and run it.

3. **Copy vs move.** Do the same "bind to a second name, use the first" with an `i32`
   instead of a `String`. **Predict**: error or fine? Why is a number different from a
   `String`?

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. Next lesson: borrowing — how to use a value without taking ownership of it.)*

## 6. What surprised you?

A sentence or two: did "assigning a `String` *moves* it" feel strange, or did the
owner-card picture make it click? Was it surprising that numbers behave differently? Tell
me, and I'll pitch Lesson 16 (references & borrowing) to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §4.1 "What Is Ownership?" The whole model: the
  stack/heap split, the three rules verbatim, the `String` move → `E0382`, the
  `Copy`-types list, `Copy`-vs-`Drop`, and drop-at-end-of-scope. The bug it prevents
  (freeing memory twice) and the stack-of-plates / find-your-place-on-the-heap framing.
- **CR** — *Comprehensive Rust* (Google), §20. The sharpest `Copy`-scalar-vs-`String`-move
  contrast, `.clone()` as a visible cost, and the "owner card" picture of a move (hand the
  card over, tear up your copy). Its tour of other languages' memory models was dropped per
  the no-analogy rule.
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Ownership." States the rules in prose;
  it shows no runnable move/error/`Drop`, so the demos come from BOOK/CR.
- Compiler output captured live on **rustc 1.95.0** (edition 2024); the ownership errors are
  edition-independent. The cleaner fix for the function-move case — **borrowing** — is
  Lesson 16.

---

<!-- lesson-nav -->
[← Lesson 14 — `Vec` and `HashMap`](14-vec-hashmap.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 16 — References & Borrowing →](16-references-and-borrowing.md)
