# Lesson 29 — `Box<T>`: values on the heap

*(Phase 7 — the smart-pointer finale begins. You know **ownership** (L15): one
value, one owner, cleaned up when the owner goes away. The next four lessons show
the standard-library types that bend that rule *safely*. Each is a "smart
pointer": a value that acts like a pointer to data, but carries extra powers.)*

## 1. Why it exists

A plain reference (`&x`, from L16) only *borrows* — it points at data someone
else owns. But one real situation doesn't fit the fixed-size mould at all: **a
type that contains itself**. A list where each link holds the next link has no
fixed size — the compiler can't tell how big one value is, so it refuses
(part 4 shows the exact error). `Box<T>` is the way out: put the value on the
**heap** and hold a small, fixed-size pointer to it — "the next link lives *over
there*."

## 2. The idea

**`Box<T>`** puts a value on the heap instead of the stack. The box itself is a
small, fixed-size pointer; the data lives elsewhere. You use it like the value
inside, and it cleans up after itself when it goes out of scope — no manual
freeing, ever. Its main beginner use: giving a self-containing type a known
size ("indirection" = *store a pointer to the value instead of the value
itself*).

## 3. Tiny examples to read

**A value on the heap.** You use it like the value inside:

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```

```
b = 5
```

**`Box<T>` enables a type that contains itself.** This list holds an `i32` and
the *rest* of the list. Without the box it has no known size (part 4); the box
makes the "rest" a fixed-size pointer, so it compiles — and we can walk it:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};

fn sum(list: &List) -> i32 {
    match list {
        Cons(value, rest) => value + sum(rest),
        Nil => 0,
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("sum = {}", sum(&list));
}
```

```
sum = 6
```

## 4. Common pitfalls / real compiler errors — infinite size

Without a `Box` (or `Rc`, or `&`), the compiler can't work out the size of one
`List` value:

```rust
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    use List::{Cons, Nil};
    let _list = Cons(1, Cons(2, Nil));
}
```

**Before you scroll — will this compile?**

```
error[E0072]: recursive type `List` has infinite size
 --> main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^
2 |     Cons(i32, List),
  |               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

The fix is right there in the message: wrap the recursive part in `Box<List>`
(the working version is in part 3). The matching exercise below hands you this
wall — **predict the code** before you run.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new boxes` works too.)* **Predict on paper before
each run.**

1. **Box on the heap.** Put a `Box::new(10)` in a variable and print it.
   **Predict** the output.
2. **Build a self-containing type WITHOUT the box first.** Write the `List` enum
   with a bare `Cons(i32, List)`. **Predict the error code**, read the
   compiler's suggested fix, then apply it and walk the list.

*(You write every line here — I won't. The predictions are your answer key.
Next: the two traits that make a box act like a reference and clean up after
itself — `Deref` and `Drop`.)*

## 6. What surprised you?

A sentence or two: did "indirection = store a pointer instead of the value" make
the infinite-size error obvious in hindsight? Tell me, and I'll pitch Lesson 29b
to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§15.1**: `Box<T>` on the heap,
  enabling recursive types, and the `E0072` "infinite size" error.
- **CR** — *Comprehensive Rust* (Google): the `Box` slides.
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 28b — Adapter chains: map, filter, collect](28b-adapter-chains.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 29b — Deref & Drop: the traits underneath →](29b-deref-drop.md)
