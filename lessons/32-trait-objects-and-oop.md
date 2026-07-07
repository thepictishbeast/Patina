# Lesson 32 — Trait Objects & OOP in Rust: `dyn Trait`, Encapsulation, States as Types

*(Phase 9 — Advanced. Lesson 25 gave you **traits** — a named set of methods a
type promises to provide — and two ways to use them as a constraint: `impl
Trait` parameters and trait bounds (`<T: Summary>`). Both pin you to **one**
concrete type at a time. This lesson lifts that limit: a **trait object**
(`dyn Trait` behind a pointer) lets a single collection or function hold **many
different concrete types** and call their shared method — the compiler decides
which one to run at runtime. We then use Rust's privacy and type system to hide
internals and to make broken states impossible.)*

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

> **How the sources frame it:** the **BOOK** Ch.18 ("OOP Features of Rust") is
> the backbone and carries the whole arc this lesson follows — encapsulation via
> privacy, trait objects for dynamic dispatch (the `Vec<Box<dyn Draw>>` screen),
> and the move from a runtime *state pattern* to encoding states **as types**.
> **CR** supplies the mechanical model: a trait-object pointer is a **fat
> pointer** (two pointers — data + vtable), plus the rule that not every trait
> can be a trait object, and the warning *not to reach for `dyn` too quickly*.
> ("OOP" and "inheritance" are just names other languages use; you don't need
> them — the problems above stand on their own.)

## 2. The idea

**`dyn Trait` never stands alone — it lives behind a pointer.** The two you'll
use:

- **`Box<dyn Trait>`** — an **owned** trait object on the heap. This is what
  goes in a `Vec<Box<dyn Draw>>`.
- **`&dyn Trait`** — a **borrowed** trait object, e.g. a function parameter
  `fn show(item: &dyn Draw)` that accepts a reference to *any* `Draw` type.

**How it works — the fat pointer (CR).** A normal `&Button` is one address: it
points at the data. A trait-object pointer is **two** addresses bundled
together — one to the **data**, one to a **vtable** (a small table of "where is
each method for this concrete type"). When you call `item.draw()`, Rust reads
the vtable to find *this* value's `draw` and jumps there. That's why it can hold
mixed types: each value carries its own method table. You can see the size
difference directly (part 3): a trait-object reference is **twice** the width of
a plain one.

**Static vs dynamic dispatch — the trade-off.**

| | generics / `impl Trait` (L25) | `dyn Trait` |
|---|---|---|
| Which method? | decided at **compile** time | looked up at **runtime** (vtable) |
| Copies of the code | **one per type** used | **one**, shared |
| What it can hold | **one** type at a time | **many** types, mixed |
| Cost | none at runtime | a small per-call lookup |

**Which to reach for (CR's caution).** Default to **static dispatch** — it's
faster and the compiler knows more. If your set of types is **closed and known**,
an **`enum`** is usually the better tool. Reach for `dyn` when the set is
**open** or you genuinely need a **mixed collection** — the screen of widgets.
Beginners over-use `dyn`; prefer it last, not first.

**Encapsulation — hide the inside.** You met module privacy in Lesson 22: items
are private by default; `pub` opens them. The OOP use is to make a struct's
**fields private** and expose only **methods**. Callers depend on the methods,
not the layout — so you can change the internals (swap a `Vec` for a `HashSet`,
recompute a cached value differently) **without breaking any caller**. The
hidden field is a promise you're free to keep however you like.

**States as types — make broken states impossible.** A value that moves through
stages (a draft post → a published post) *could* be modelled with a trait object
(`Box<dyn State>`, the classic "state pattern"). But Rust often prefers a sharper
move: give **each state its own type**, and put on each type only the methods
that are valid in that state. A `DraftPost` simply **has no** `content()` method,
so "read an unpublished draft" isn't a bug you catch at runtime — it's a program
that **won't compile** (part 4). The type system rules the invalid state out.

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

**The fat pointer, measured.** A trait-object reference carries **two** pointers
(data + vtable), so it's twice the width of a plain reference on a 64-bit
machine:

```rust
use std::mem::size_of;

trait Draw {}
struct Button;
impl Draw for Button {}

fn main() {
    println!("&Button   = {} bytes", size_of::<&Button>());
    println!("&dyn Draw = {} bytes", size_of::<&dyn Draw>());
}
```

```
&Button   = 8 bytes
&dyn Draw = 16 bytes
```

That extra 8 bytes **is** the vtable pointer — the price of asking "which type
am I?" at runtime.

**Encapsulation — private fields, public methods (BOOK).** The fields are
private; callers only touch the methods. The average is kept correct internally,
and the *how* is hidden — you could change it later without touching any caller:

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection { list: Vec::new(), average: 0.0 }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut c = AveragedCollection::new();
    c.add(10);
    c.add(20);
    c.add(30);
    println!("average = {}", c.average());
}
```

```
average = 20
```

`update_average` is private — a detail. Callers say `add` and `average`; they
can't reach in and leave `list` and the cached `average` out of step.

**States as types (BOOK).** Two types, two stages. A draft can only `publish`;
a published post can be read. A draft *has no* `content()` at all:

```rust
struct DraftPost {
    text: String,
}

struct PublishedPost {
    text: String,
}

impl DraftPost {
    fn new(text: &str) -> DraftPost {
        DraftPost { text: String::from(text) }
    }

    // A draft cannot be read — there is no content() here.
    fn publish(self) -> PublishedPost {
        PublishedPost { text: self.text }
    }
}

impl PublishedPost {
    fn content(&self) -> &str {
        &self.text
    }
}

fn main() {
    let draft = DraftPost::new("hello world");
    let post = draft.publish();        // draft is consumed, becomes published
    println!("{}", post.content());
}
```

```
hello world
```

`publish` takes `self` **by value**, so it *consumes* the draft and hands back a
`PublishedPost`. The draft no longer exists to misuse — and reading it before
publishing is a compile error, next.

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

**A trait that can't be a trait object — `E0038`.** Not every trait works behind
`dyn`. To build a vtable, each method must be callable through the pointer alone.
A **generic method** breaks that (there's no single method to put in the table),
so the trait is *not dyn compatible*:

```rust
trait Draw {
    fn draw(&self);
    fn make<T>(&self) -> T;   // generic method -> no vtable possible
}

fn main() {
    let _screen: Vec<Box<dyn Draw>> = Vec::new();
}
```

```
error[E0038]: the trait `Draw` is not dyn compatible
 --> main.rs:7:30
  |
7 |     let _screen: Vec<Box<dyn Draw>> = Vec::new();
  |                              ^^^^ `Draw` is not dyn compatible
  |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
 --> main.rs:3:8
  |
1 | trait Draw {
  |       ---- this trait is not dyn compatible...
2 |     fn draw(&self);
3 |     fn make<T>(&self) -> T;   // generic method -> no vtable possible
  |        ^^^^ ...because method `make` has generic type parameters
  = help: consider moving `make` to another trait
```

The message names the exact method and why. For a beginner the rule of thumb is
simple: keep the methods you want behind `dyn` plain (`&self`, no generics), and
this won't bite you.

**Using a value in the wrong state — `E0599`.** This is the **payoff** of
encoding states as types. Try to read a `DraftPost` before publishing it:

```rust
struct DraftPost {
    text: String,
}

struct PublishedPost {
    text: String,
}

impl DraftPost {
    fn new(text: &str) -> DraftPost {
        DraftPost { text: String::from(text) }
    }
    fn publish(self) -> PublishedPost {
        PublishedPost { text: self.text }
    }
}

impl PublishedPost {
    fn content(&self) -> &str {
        &self.text
    }
}

fn main() {
    let draft = DraftPost::new("hello");
    println!("{}", draft.content());   // a draft has no content() — wrong state
}
```

```
error[E0599]: no method named `content` found for struct `DraftPost` in the current scope
  --> main.rs:26:26
   |
 1 | struct DraftPost {
   | ---------------- method `content` not found for this struct
...
26 |     println!("{}", draft.content());   // a draft has no content() — wrong state
   |                          ^^^^^^^ method not found in `DraftPost`
```

There's nothing to handle at runtime — the invalid state simply **can't be
written**. That's the whole idea: instead of a method that checks "am I
published?" and maybe fails, the wrong call is a compile error. Invalid states
become **unrepresentable**.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new traitobjects` works too.)* **Predict on paper before
each run.**

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

4. **A state that can't be misused.** Model a two-stage value with **two types**.
   For example `LockedDoor` and `OpenDoor`: a `LockedDoor` has an `unlock(self)
   -> OpenDoor` method and **nothing else**; an `OpenDoor` has a
   `walk_through(&self)` that prints something. In `main`, make a locked door,
   `unlock` it, then `walk_through`. **Predict the output.** Then add a line that
   calls `walk_through` on the **locked** door (before unlocking). **Predict the
   error code** before compiling — and explain in one sentence why this is
   *better* than a single `Door` type with a runtime "is it open?" check.

*(You write every line here — I won't. The predictions are your answer key; the
code is yours. With trait objects you can hold and handle **mixed** types behind
one shared method; with privacy you can hide internals so they're free to change;
and with states-as-types you can hand the compiler the job of rejecting broken
states for you.)*

## 6. What surprised you?

A sentence or two: did "`dyn Trait` = one collection, many types, method chosen
at runtime" land — and did the fat-pointer size (8 → 16 bytes) make the runtime
cost concrete? Did the static-vs-dynamic table change *when* you'd reach for
`dyn` versus a generic or an enum? And did the `E0599` on a draft feel different
from a runtime check — a broken state that **can't be written** rather than one
you catch later? Tell me, and I'll fold it into the Phase-9 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.18** "OOP Features of Rust":
  §18.1 (encapsulation via privacy — the `AveragedCollection` example, private
  fields behind public methods), §18.2 (trait objects, `Box<dyn Draw>`, the
  `Screen`/`Button`/`SelectBox` example, and **dynamic dispatch** vs the
  static dispatch of generics), §18.3 (the state pattern and its Rust-native
  rewrite — **encoding states as types**, the `DraftPost`/`PublishedPost`
  example where "invalid states are now impossible because of the type system").
- **CR** — *Comprehensive Rust* (Google): the **fat / wide pointer** model of a
  trait object (one pointer to the data, one to the vtable), trait
  **dyn-compatibility**, and the caution not to reach for `dyn` too quickly —
  prefer static dispatch or an enum for a closed set of types.
- **BLOG** — not used here; this topic is sourced from BOOK/CR.
- Every snippet compiled and run, and every error captured live, on **rustc
  1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`; temp paths normalized
  to `main.rs`). The fat-pointer sizes (8 and 16 bytes) are from a 64-bit
  target. This continues the Phase-9 advanced lessons (trait objects · OOP
  patterns).

---

<!-- lesson-nav -->
[← Lesson 31 — Async / Await: `async fn`, `.await`, `Future`](31-async-await.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 33 — Advanced Patterns & Matching →](33-advanced-patterns.md)
