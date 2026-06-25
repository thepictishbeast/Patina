# Lesson 29 — Smart Pointers: `Box`, `Rc`, `RefCell`, `Deref`, `Drop`

*(Phase 7 — Functional features & smart pointers, the finale. You already know
**ownership** (L15): one value, one owner, cleaned up when the owner goes away.
That rule is strict on purpose. This lesson shows the handful of standard-library
types that bend it *safely* — putting data on the heap, letting a value have
**several** owners, and letting you change a value that looks frozen. Each is a
"smart pointer": a value that acts like a pointer to data, but carries extra
powers.)*

## 1. Why it exists

A plain reference (`&x`, from L16) only *borrows* — it points at data someone else
owns, and it has no powers of its own. Most of the time that's all you need. But a
few real situations don't fit the one-owner, fixed-size, borrow-only mould:

- **A type that contains itself.** A list where each link holds the next link has
  no fixed size — the compiler can't tell how big one value is, so it refuses
  (you'll see the exact error in part 4). You need a way to say "the next link
  lives *over there*, and here's a pointer to it" so the size is known.
- **Data with more than one owner.** Sometimes two parts of a program genuinely
  share the same value, and *neither* should clean it up until **both** are done.
  Single ownership can't express that.
- **Changing a value that's shared.** Once a value is shared for reading, the
  borrow rules forbid changing it. Occasionally you're certain a change is safe but
  the compiler can't see why.

A **smart pointer** is a struct that acts like a pointer but adds capabilities.
Each one in this lesson solves exactly one of the problems above, and each cleans
up after itself automatically.

> **How the sources frame it:** the **BOOK** Ch.15 is the backbone — it's the only
> source that unifies all five under one story. It defines a smart pointer as "a
> data structure that act[s] like a pointer but also ha[s] additional metadata and
> capabilities," built on two traits: **`Deref`** (behave like a reference) and
> **`Drop`** (run cleanup at scope-end). The TV metaphor in part 2 is quoted from
> the BOOK. **CR** reinforces with clean `Box`/`Rc`/`RefCell` slides and the
> "`Rc::clone` is cheap — it just bumps the count" note. **BLOG** doesn't teach this.

## 2. The idea

Four tools, each a one-line job:

- **`Box<T>`** — put a value on the **heap** instead of the stack. The box itself is
  a small, fixed-size pointer; the data lives elsewhere. Its main beginner use:
  giving a self-containing type a known size.
- **`Rc<T>`** — **multiple owners** of one value (Rc = *reference counted*). It keeps
  a count of how many owners exist. `Rc::clone` makes another owner and adds 1; when
  an owner goes out of scope the count drops by 1. At **zero**, the value is cleaned
  up — not before.
- **`RefCell<T>`** — **interior mutability**: change a value through a *shared,
  immutable-looking* handle. The borrow rules (one mutable **or** many immutable,
  never both) still apply — but `RefCell<T>` checks them at **runtime**. Break them
  and your program **panics** instead of failing to compile.
- **`Deref`** and **`Drop`** are the two traits *underneath* all of these. `Deref`
  is why `*` works on a box just like on a reference. `Drop` is the cleanup hook
  that runs automatically when a value goes out of scope — it's how an `Rc` knows to
  decrease its count.

> **The metaphor (BOOK, Ch.15.4):** *imagine `Rc<T>` as a TV in a family room. When
> one person enters to watch TV, they turn it on. Others can come into the room and
> watch the TV. When the last person leaves the room, they turn off the TV because
> it's no longer being used.* The data is the TV; each owner is a person in the room;
> the value is cleaned up only when the **last** owner leaves.

When do you reach for `RefCell`? Only when you **can't** just add `mut` — because the
value is shared through an `Rc` (so it has no single mutable owner), or it's behind a
`&self` method. For a plain local variable, `let mut` is simpler and better. We'll
see the shared case at the end of part 3.

## 3. Tiny examples to read

**`Box<T>` — a value on the heap.** You use it like the value inside:

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```

```
b = 5
```

**`Box<T>` enables a type that contains itself.** This list holds an `i32` and the
*rest* of the list. Without the box it has no known size (part 4); the box makes the
"rest" a fixed-size pointer, so it compiles — and we can walk it:

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

**`Deref` — `*` follows a box like it follows a reference.** A box behaves like a
reference because it implements `Deref`:

```rust
fn main() {
    let x = 5;
    let y = &x;          // a reference to x
    let z = Box::new(x); // a box holding a copy of x

    println!("{}", *y);  // follow the reference
    println!("{}", *z);  // follow the box — same `*`
}
```

```
5
5
```

**`Drop` — cleanup runs automatically at scope-end.** Give a type a `drop` method and
Rust calls it for you when the value goes out of scope. Note the **reverse** order:

```rust
struct Guard {
    name: String,
}

impl Drop for Guard {
    fn drop(&mut self) {
        println!("dropping {}", self.name);
    }
}

fn main() {
    let _a = Guard { name: String::from("a") };
    let _b = Guard { name: String::from("b") };
    println!("both created");
}
```

```
both created
dropping b
dropping a
```

You never call `drop` yourself — Rust inserts the call. `_b` was created last, so it's
dropped first. (This same hook is what makes an `Rc` decrease its count, below.)

**`Rc<T>` — many owners, counted.** `Rc::clone` adds an owner; the count falls when an
owner goes out of scope. `Rc::strong_count` lets us watch it:

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared text"));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("c reads: {c}");
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("b still reads: {b}");
}
```

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
c reads: shared text
count after c goes out of scope = 2
b still reads: shared text
```

`Rc::clone` is **cheap** — it doesn't copy the string, it just bumps the count. The
value isn't cleaned up while any owner remains: the last person leaves before the TV
goes off.

**`RefCell<T>` — change a value through a shared handle.** Note `balance` is *not*
`mut`, yet we change it — `borrow_mut` hands us a temporary mutable view:

```rust
use std::cell::RefCell;

fn main() {
    let balance = RefCell::new(100);   // not declared `mut`

    *balance.borrow_mut() += 50;       // change it anyway

    println!("balance = {}", balance.borrow());
}
```

```
balance = 150
```

On its own this is pointless — `let mut balance = 100` would be simpler. `RefCell`
earns its place only when you **can't** add `mut`. The classic case is sharing one
mutable value among several owners, by putting a `RefCell` *inside* an `Rc`:

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let balance = Rc::new(RefCell::new(100));

    let owner_a = Rc::clone(&balance);
    let owner_b = Rc::clone(&balance);

    *owner_a.borrow_mut() += 50;             // change through one owner

    println!("a sees {}", owner_a.borrow());
    println!("b sees {}", owner_b.borrow()); // the other owner sees it too
}
```

```
a sees 150
b sees 150
```

There are **two** owners of one value, and a change made through one is visible
through the other — one heap value, many owners, mutable. A plain `let mut` can't give
you two owners, which is exactly why `RefCell` exists. (`Rc<RefCell<T>>` is the combo
worth remembering; the BOOK builds toward it deliberately.)

## 4. Common pitfalls / real compiler errors

**A type that contains itself, with no indirection — `E0072`.** Without a `Box` (or
`Rc`, or `&`), the compiler can't work out the size of one `List` value:

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

The fix is right there in the message: wrap the recursive part in `Box<List>` (the
working version is in part 3). A box is a fixed-size pointer, so the size is knowable
again — "indirection" means *store a pointer to the value instead of the value itself.*

**Breaking the borrow rules with `RefCell` — a runtime panic, not a compile error.**
`RefCell` still enforces "one mutable borrow at a time"; it just checks at **runtime**.
Two live `borrow_mut`s compile fine, then crash when run:

```rust
use std::cell::RefCell;

fn main() {
    let balance = RefCell::new(100);

    let mut one = balance.borrow_mut();
    let mut two = balance.borrow_mut();   // second live borrow_mut

    *one += 50;
    *two += 50;
}
```

```
thread 'main' panicked at main.rs:7:27:
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This is the trade-off `RefCell` makes: it accepts code the compiler would reject, but
moves the borrow check to runtime — so a mistake that would normally be a *compile*
error becomes a *crash* instead. Hold one borrow at a time and you're safe. (With a
plain `&mut` this same mistake is a compile error you can't run past; `RefCell` trusts
you to get it right and panics if you don't.)

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new pointers`. **Predict on paper before each run.**

1. **Box on the heap.** Put a `Box::new(10)` in a variable, then print both `b` and
   `*b`. **Predict** whether they print the same thing and why. Then try to add a plain
   number to the box directly (`b + 1`) versus the dereferenced value (`*b + 1`) — one
   of these the compiler will reject. **Predict which**, then check.

2. **Watch a count rise and fall.** Make an `Rc::new(...)` holding any value. Print
   `Rc::strong_count` after creating it, after one `Rc::clone`, inside an inner `{ }`
   block where you make a *second* clone, and again *after* that block ends. **Predict
   the four numbers** before running. (Re-read the TV metaphor if you get stuck on the
   last one.)

3. **Drop order.** Make a `struct` with a `Drop` impl that prints a name, then create
   three of them in `main` with different names and print `"made all three"` at the end.
   **Predict the full output, in order**, before you run it — including which name's
   `drop` line prints first.

4. **Interior mutability, then break it.** Make a `RefCell::new(0)`, add `5` to it
   through `borrow_mut`, and print it with `borrow`. Once that works, deliberately hold
   **two** `borrow_mut` handles alive at the same time. **Predict**: does it fail to
   *compile*, or compile and *panic* when run? Name which, then confirm — and notice
   the message names the rule you broke.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. With these four types you can now reach past single ownership when a real problem
needs it — heap storage, shared owners, or a safe change behind a shared handle — and let
the type clean up after itself.)*

## 6. What surprised you?

A sentence or two: did "a smart pointer is a value with extra powers, that cleans up
after itself" land? Did the `Rc` count going `1 → 2 → 3 → 2` make the TV metaphor click?
Did it surprise you that breaking `RefCell`'s rule is a *crash* rather than a compile
error — and does that change when you'd reach for it versus a plain `&mut`? Tell me, and
I'll fold it into the Phase-7 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.15** "Smart Pointers": §15.1
  (`Box<T>` on the heap; enabling recursive types; the `E0072` "infinite size" error),
  §15.2 (`Deref` — treating smart pointers like references), §15.3 (`Drop` — running
  code on cleanup, reverse drop order), §15.4 (`Rc<T>`, `Rc::clone`, `Rc::strong_count`,
  and the TV-in-a-family-room metaphor, quoted), §15.5 (`RefCell<T>`, interior
  mutability, runtime borrow checking, and the `Rc<RefCell<T>>` combination).
- **CR** — *Comprehensive Rust* (Google): the `Box`/`Rc`/`RefCell` slides and the
  "`Rc::clone` is cheap — it just bumps the count" framing.
- **BLOG** — not used here; this topic is sourced from BOOK/CR.
- Every snippet compiled and run, and every error/panic captured live, on **rustc
  1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`). One note on the live output:
  the runtime panic prints `RefCell already borrowed` on 1.95.0; older BOOK printings
  show the wording `already borrowed: BorrowMutError` — same failure, captured here from
  the real compiler. This closes the Phase-7 lessons (closures · iterators · smart
  pointers).

---

<!-- lesson-nav -->
[← Lesson 28 — Iterators](28-iterators.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 30 — Threads, Channels & Shared State →](30-threads-and-concurrency.md)
