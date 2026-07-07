# Lesson 32 — Trait objects: one collection, many types

*(Phase 9 — Advanced, the opener. Lesson 25 gave you **traits**, and 25b two
ways to demand one: `impl Trait` parameters and bounds (`<T: Summary>`). Both
pin you to **one** concrete type at a time. This lesson lifts that limit: a
**trait object** — `dyn Trait` behind a pointer — lets one collection or
function hold **many different concrete types** and call their shared method,
with the right one found at runtime.)*

## 1. Why it exists

Say you're building a UI. A screen holds a `Button`, a `SelectBox`, a text
field — unrelated structs, but each knows how to **draw itself**. You want one
list of "things on the screen" and one loop that says "draw each of you," not
knowing or caring which kind each one is.

You already know two ways to share behavior, and **neither fits this job:**

- A **generic** with a trait bound (`<T: Draw>`) gets stamped out **once per
  concrete type** (Lesson 24 called this one machine per type). A
  `Vec<T>` where `T: Draw` is therefore locked to **one** type — every element
  must be the *same* kind. It can't hold a `Button` **and** a `SelectBox`.
- Listing every kind in an `enum` works only when you know the **full, fixed
  set** ahead of time. Add a new widget later and you must edit the enum and
  every `match`.

A **trait object** solves the open, mixed case. `Box<dyn Draw>` means "**some**
value — I don't know which type — that implements `Draw`." A
`Vec<Box<dyn Draw>>` can hold a `Button`, a `SelectBox`, and any future widget
side by side, and one loop draws them all. The right method is found at
**runtime**, per element. This is **dynamic dispatch**.

## 2. The idea

**`dyn Trait` never stands alone — it lives behind a pointer.** The two you'll
use:

- **`Box<dyn Trait>`** — an **owned** trait object on the heap. This is what
  goes in a `Vec<Box<dyn Draw>>`.
- **`&dyn Trait`** — a **borrowed** trait object, e.g. a function parameter
  `fn show(item: &dyn Draw)` that accepts a reference to *any* `Draw` type.

**How it works — the fat pointer (CR).** A normal `&Button` is one address. A
trait-object pointer is **two** addresses bundled together — one to the
**data**, one to a **vtable** (a small table of "where is each method for this
concrete type"). When you call `item.draw()`, Rust reads the vtable to find
*this* value's `draw` and jumps there. That's why one `Vec` can hold mixed
types: each value carries its own method table. (Want to see the two pointers
measured, and the machinery underneath? CR's trait-objects slides and Book
Ch.18.2 go deeper — a trait-object reference is literally twice the width of a
plain one.)

**Static vs dynamic dispatch — the trade-off.**

| | generics / `impl Trait` (L25) | `dyn Trait` |
|---|---|---|
| Which method? | decided at **compile** time | looked up at **runtime** (vtable) |
| Copies of the code | **one per type** used | **one**, shared |
| What it can hold | **one** type at a time | **many** types, mixed |
| Cost | none at runtime | a small per-call lookup |

**Which to reach for (CR's caution).** Default to **static dispatch** — it's
faster and the compiler knows more. If your set of types is **closed and
known**, an **`enum`** is usually the better tool. Reach for `dyn` when the set
is **open** or you genuinely need a **mixed collection** — the screen of
widgets. Beginners over-use `dyn`; prefer it last, not first.

> **How the sources frame it:** the **BOOK** Ch.18 §18.2 carries this lesson —
> the `Vec<Box<dyn Draw>>` screen and dynamic-vs-static dispatch. **CR**
> supplies the fat-pointer model and the don't-reach-for-`dyn`-first caution.
> ("OOP" and "inheritance" are names other languages use; you don't need them —
> the mixed-collection problem stands on its own. The other two moves in the
> Book's OOP chapter are Lessons 32b and 32c.)

## 3. Tiny examples to read

**`Vec<Box<dyn Draw>>` — many types, one loop (BOOK).** A `Button` and a
`SelectBox` are different structs; both implement `Draw`. They sit in one vector,
and one loop draws each — the vtable picks the right `draw` per element:

```rust
trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

struct SelectBox {
    options: Vec<String>,
}

impl Draw for Button {
    fn draw(&self) {
        println!("[Button: {}]", self.label);
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("[SelectBox: {} options]", self.options.len());
    }
}

fn main() {
    let screen: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { label: String::from("OK") }),
        Box::new(SelectBox {
            options: vec![
                String::from("yes"),
                String::from("no"),
                String::from("maybe"),
            ],
        }),
    ];

    for component in &screen {
        component.draw();
    }
}
```

```
[Button: OK]
[SelectBox: 3 options]
```

Two different concrete types, side by side in one `Vec`, each drawing itself.
A generic `Vec<T>` could not do this — it would be locked to one type.

**`&dyn Trait` — one function, any implementer.** No `Box`, no heap: a borrowed
trait object as a parameter. `announce` accepts *any* `Greet`:

```rust
trait Greet {
    fn hello(&self) -> String;
}

struct English;
struct Spanish;

impl Greet for English {
    fn hello(&self) -> String {
        String::from("Hello")
    }
}

impl Greet for Spanish {
    fn hello(&self) -> String {
        String::from("Hola")
    }
}

fn announce(g: &dyn Greet) {
    println!("{}!", g.hello());
}

fn main() {
    announce(&English);
    announce(&Spanish);
}
```

```
Hello!
Hola!
```

## 4. Common pitfalls / real compiler errors

**Mixing concrete types in a plain `Vec` — `E0308`.** This is the very problem
trait objects exist to solve. Drop the `Box<dyn …>` and try to put a `Button`
and a `SelectBox` in one vector directly:

```rust
trait Draw {
    fn draw(&self);
}

struct Button;
struct SelectBox;

impl Draw for Button {
    fn draw(&self) { println!("button"); }
}
impl Draw for SelectBox {
    fn draw(&self) { println!("select box"); }
}

fn main() {
    let screen = vec![Button, SelectBox];
    for c in &screen {
        c.draw();
    }
}
```

```
error[E0308]: mismatched types
  --> main.rs:16:31
   |
16 |     let screen = vec![Button, SelectBox];
   |                               ^^^^^^^^^ expected `Button`, found `SelectBox`
```

The first element fixes the vector's type to `Button`; the second doesn't match.
The fix is the lesson: make it `Vec<Box<dyn Draw>>` and `Box::new(...)` each
element (the working version in part 3). "Both implement `Draw`" isn't enough on
its own — you must **ask** for a trait object.

**Not every trait can go behind `dyn` — `E0038`.** To build a vtable, each
method must be callable through the pointer alone; a **generic method** breaks
that (there's no single method to put in the table), so the compiler rejects
the whole trait object: *"error[E0038]: the trait `Draw` is not dyn compatible
… because method `make` has generic type parameters."* The rule of thumb that
keeps you clear of it: keep the methods you want behind `dyn` plain (`&self`,
no generics). The full dyn-compatibility story — with the compiler's complete
explanation — is **Book Ch.18.2** and the reference page the error itself links
to; read it when a real trait of yours trips this.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new traitobjects` works too.)* **Predict on
paper before each run.**

1. **A mixed collection that draws itself.** Define a trait `Shape` with one
   method `area(&self) -> f64`. Make two structs — say `Circle { radius: f64 }`
   and `Square { side: f64 }` — and implement `Shape` for each. Build a
   `Vec<Box<dyn Shape>>` holding **one of each**, then loop over it printing each
   area. **Predict the two numbers** before running. Then try removing the
   `Box<dyn …>` and putting the bare `Circle` and `Square` in one `vec![]` —
   **predict the error code** before you compile, and name *why* the trait
   object was needed.

2. **Borrowed trait object as a parameter.** Keep your `Shape` types. Write a
   function `fn describe(s: &dyn Shape)` that prints the area, and call it once
   with a `Circle` and once with a `Square` (pass `&circle`, `&square`). No
   `Box`, no `Vec` this time. **Predict**: does the *same* function body run for
   both calls, and how does Rust know which `area` to use? Say it in one
   sentence, then run.

3. **Static vs dynamic, side by side.** Rewrite `describe` as a generic instead:
   `fn describe<T: Shape>(s: &T)`. Both versions compile and print the same
   thing — **predict that**, then run to confirm. Now answer in prose (no code):
   which version produces **one** copy of the function in the final program, and
   which produces **one per type** you call it with? Which would you pick if your
   shapes all live in **one mixed `Vec`** — and why does that choice force your
   hand?

*(You write every line here — I won't. The predictions are your answer key.
Next, the Book's other two OOP moves: hiding a struct's insides — Lesson 32b —
and making broken states uncompilable — Lesson 32c.)*

## 6. What surprised you?

A sentence or two: did "`dyn Trait` = one collection, many types, method chosen
at runtime" land — and did the two-pointer picture make the runtime cost
concrete? Did the static-vs-dynamic table change *when* you'd reach for `dyn`
versus a generic or an enum? Tell me, and I'll fold it into the Phase-9 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.18 §18.2**: trait objects,
  `Box<dyn Draw>`, the `Screen`/`Button`/`SelectBox` example, **dynamic
  dispatch** vs the static dispatch of generics, and the dyn-compatibility
  discussion this lesson compresses to a rule of thumb.
- **CR** — *Comprehensive Rust* (Google): the **fat / wide pointer** model of a
  trait object (one pointer to the data, one to the vtable — with the
  `size_of` measurement this lesson points at rather than reproduces), and the
  caution not to reach for `dyn` too quickly.
- Every snippet compiled and run, and every error captured live, on **rustc
  1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`; temp paths normalized
  to `main.rs`). This opens Phase 9 (advanced).

---

<!-- lesson-nav -->
[← Lesson 31b — Futures are lazy: why async needs a runtime](31b-futures-are-lazy.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 32b — Encapsulation: private fields, public methods →](32b-encapsulation.md)
