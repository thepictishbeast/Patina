# Phase 9 Quiz — Advanced (Trait Objects, Patterns, Features, Capstone)

A self-check for the Phase-9 **Advanced** lessons (Lessons 32–35: trait objects &
dynamic dispatch, advanced patterns & refutability, `unsafe`/operator-overloading/
`macro_rules!`, and the multithreaded-web-server capstone). Same rule as before:
**predict each answer before** you look at the **Answers** section. Don't run the code
first; predict, then verify. Sixteen questions.

> Tip: cover the Answers section until you've committed to an answer for every question.
> The **trait-object**, **pattern**, and **feature** snippets here are runnable and their
> outputs are deterministic. The **capstone** snippets (Lesson 35) are **compile-checked
> only** — that program binds a TCP port, so it's never run for output here; a capstone
> question asks *"does this compile / what's the error?"* or is conceptual, never *"what
> does the server print?"*

---

## Questions

**Q1 — concept (one line).** A `Vec<T>` with `T: Draw` (a generic bound) and a
`Vec<Box<dyn Draw>>` both hold "things that can draw." In one sentence: which of the two
can hold a `Button` **and** a `SelectBox` side by side, and what is the name for the
runtime method lookup it uses?

**Q2 — predict the output.**
```rust
trait Draw {
    fn draw(&self);
}

struct Button { label: String }
struct SelectBox { options: Vec<String> }

impl Draw for Button {
    fn draw(&self) { println!("[Button: {}]", self.label); }
}
impl Draw for SelectBox {
    fn draw(&self) { println!("[SelectBox: {} options]", self.options.len()); }
}

fn main() {
    let screen: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { label: String::from("OK") }),
        Box::new(SelectBox {
            options: vec![String::from("yes"), String::from("no"), String::from("maybe")],
        }),
    ];
    for component in &screen {
        component.draw();
    }
}
```

**Q3 — predict the output.**
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
(Assume a 64-bit machine.) Say both numbers and, in a phrase, what the extra width *is*.

**Q4 — does this compile? If not, what's the error code?**
```rust
trait Draw { fn draw(&self); }
struct Button;
struct SelectBox;
impl Draw for Button { fn draw(&self) { println!("button"); } }
impl Draw for SelectBox { fn draw(&self) { println!("select box"); } }

fn main() {
    let screen = vec![Button, SelectBox];
    for c in &screen { c.draw(); }
}
```

**Q5 — does this compile? If not, what's the error code?**
```rust
trait Draw {
    fn draw(&self);
    fn make<T>(&self) -> T;   // a generic method
}

fn main() {
    let _screen: Vec<Box<dyn Draw>> = Vec::new();
}
```

**Q6 — does this compile? If not, what's the error code?**
```rust
struct DraftPost { text: String }
struct PublishedPost { text: String }

impl DraftPost {
    fn new(text: &str) -> DraftPost { DraftPost { text: String::from(text) } }
    fn publish(self) -> PublishedPost { PublishedPost { text: self.text } }
}
impl PublishedPost {
    fn content(&self) -> &str { &self.text }
}

fn main() {
    let draft = DraftPost::new("hello");
    println!("{}", draft.content());
}
```

**Q7 — predict the output.**
```rust
enum Message { Hello { id: i32 } }

fn classify(msg: &Message) {
    match msg {
        Message::Hello { id: id @ 3..=7 } => println!("Found an id in range: {id}"),
        Message::Hello { id } if id % 2 == 0 => println!("Even id: {id}"),
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
}

fn main() {
    classify(&Message::Hello { id: 5 });
    classify(&Message::Hello { id: 8 });
    classify(&Message::Hello { id: 11 });
}
```

**Q8 — predict the output.**
```rust
struct Point { x: i32, y: i32 }

fn main() {
    let shape = Some(Point { x: 3, y: -4 });
    match shape {
        Some(Point { x: 0, y }) => println!("on the y-axis at {y}"),
        Some(Point { x, y: 0 }) => println!("on the x-axis at {x}"),
        Some(Point { x, y }) => println!("at ({x}, {y})"),
        None => println!("no point"),
    }
}
```

**Q9 — predict the output.**
```rust
fn main() {
    let pair = (5, true);
    match pair {
        (4 | 5 | 6, active) if active => println!("matched and active"),
        (4 | 5 | 6, _) => println!("matched but not active"),
        _ => println!("no match"),
    }
}
```

**Q10 — does this compile? If not, what's the error code?**
```rust
fn main() {
    let opt: Option<i32> = Some(5);
    let Some(x) = opt;
    println!("{x}");
}
```

**Q11 — does this compile, and if so what does it print?**
```rust
fn main() {
    if let (a, b) = (1, 2) {
        println!("{a} {b}");
    }
}
```
(Watch for a compiler *message* even if it builds — say which kind.)

**Q12 — predict the output.**
```rust
fn main() {
    let mut num = 5;

    let r1 = &raw const num;   // *const i32
    let r2 = &raw mut num;     // *mut i32

    unsafe {
        *r2 += 10;
        println!("r1 reads: {}", *r1);
    }
}
```

**Q13 — does this compile? If not, what's the error code?**
```rust
fn main() {
    let num = 5;
    let r = &raw const num;   // making a raw pointer
    println!("{}", *r);       // following it
}
```

**Q14 — fill in the blanks (concept).** An `unsafe { }` block unlocks exactly **five**
abilities ("the five superpowers") and nothing else — it does **not** turn off the borrow
checker. Name as many as you can; the canonical five are: (a) **`____`** a raw pointer;
(b) **`____`** an `unsafe` function or method; (c) access or modify a **mutable `____`**;
(d) **`____`** an `unsafe` trait; (e) access the fields of a **`____`**.

**Q15 — predict the output.**
```rust
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($x); )*
            v
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3];
    println!("v = {v:?}");
}
```

**Q16 — capstone, does this compile? If not, what's the error code?** (Compile-check only
— this snippet is from the web-server lesson.)
```rust
use std::sync::mpsc;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn main() {
    let (_sender, receiver) = mpsc::channel::<Job>();
    for _ in 0..4 {
        let handle = thread::spawn(move || {
            let job = receiver.recv().unwrap();   // each thread wants the same receiver
            job();
        });
    }
}
```
Then, in a phrase each (concept): **what is** the type `Box<dyn FnOnce() + Send + 'static>`,
and **why** is the shared receiver wrapped as `Arc<Mutex<Receiver>>`?

---

## Answers

*(Verified on rustc 1.95.0, edition 2024. Trait-object / pattern / feature snippets were
compiled and run; the capstone snippet (Q16) was **compile-checked only** — its program
binds a TCP port, so it is never run for output, and no server output is shown or
fabricated.)*

**A1 — The `Vec<Box<dyn Draw>>` (the trait object) can hold a `Button` and a `SelectBox`
together; the runtime lookup is called **dynamic dispatch**.** A generic `Vec<T>` is
stamped out once per concrete type, so it's locked to **one** type — every element must be
the same kind. A trait object means "some value that implements `Draw`," so the vector can
mix types and the right `draw` is found per element at runtime (via the vtable). (Lesson 32.)

**A2 — `[Button: OK]` then `[SelectBox: 3 options]`.** Two different concrete types sit in
one `Vec<Box<dyn Draw>>`, and the one loop calls each value's own `draw` through its vtable
— exactly the mixed collection a plain generic `Vec<T>` could not hold. (Lesson 32.)

**A3 — `&Button = 8 bytes`, `&dyn Draw = 16 bytes`.** A plain reference is one address. A
trait-object reference is a **fat pointer** — two addresses, one to the data and one to the
**vtable** — so it's twice the width on a 64-bit machine. That extra 8 bytes *is* the
vtable pointer, the cost of asking "which type am I?" at runtime. (Lesson 32.)

**A4 — No: `error[E0308]`** ("mismatched types"). The first element fixes the vector's type
to `Button`; the second is a `SelectBox`, which doesn't match. "Both implement `Draw`" isn't
enough — to mix types you must **ask** for a trait object: `Vec<Box<dyn Draw>>` with
`Box::new(...)` around each element. (Lesson 32.)

**A5 — No: `error[E0038]`** ("the trait `Draw` is not dyn compatible"). To build a vtable,
every method must be callable through the pointer alone; a **generic method** (`make<T>`)
has no single entry to put in the table, so the trait can't be a trait object. Keep methods
behind `dyn` plain (`&self`, no generics) and this won't bite. (Lesson 32.)

**A6 — No: `error[E0599]`** ("no method named `content` found for struct `DraftPost`"). This
is the payoff of encoding states as types: a `DraftPost` simply **has no** `content()`
method, so "read an unpublished draft" isn't a runtime check that might fail — it's a program
that won't compile. The invalid state is unrepresentable. (Lesson 32.)

**A7 — three lines: `Found an id in range: 5` / `Even id: 8` / `Found some other id: 11`.**
`5` hits the first arm — `id @ 3..=7` captures the value *and* confirms the range. `8` fails
the range but the guard `id % 2 == 0` is true. `11` fails both, so the catch-all binds it.
Arm order matters: the range arm is written first, so an in-range even value like `6` would
be claimed by it, never reaching the even-guard. (Lesson 33.)

**A8 — `at (3, -4)`.** The pattern mirrors the data's shape: `Some(...)` peels the `Option`,
then `Point { x: 0, y }` matches *only if* `x` is literally `0`. Here `x` is `3`, so the
first two arms miss and the third binds both fields. (Lesson 33.)

**A9 — `matched and active`.** The first arm needs **both**: the number is `4`, `5`, or `6`
**and** `active` is `true`. With a `|` pattern, the guard applies to the whole alternation.
`5` qualifies and `active` is `true`, so it wins. (Lesson 33.)

**A10 — No: `error[E0005]`** ("refutable pattern in local binding"). `Some(x)` can fail (the
value might be `None`), and a plain `let` has no else-branch to run on a miss — so it
requires an **irrefutable** pattern. The fix is to provide an else path: `if let` or
`let … else`. (Lesson 33.)

**A11 — Yes, it compiles and prints `1 2` — but with a *warning*** ("irrefutable `if let`
pattern"). `(a, b)` always matches, so the `if let` is pointless; the compiler still builds
it and advises replacing the `if let` with a plain `let`. This is the mirror of the rule:
irrefutable → `let`/`for`; refutable → `if let`/`while let`/`match`. (Lesson 33.)

**A12 — `r1 reads: 15`.** Both raw pointers refer to the same `num`, so the write `*r2 += 10`
(5 → 15) is visible when read through `*r1`. Both `*` derefs sit inside the one `unsafe`
block — *making* a raw pointer is safe; *following* one is the superpower that needs
`unsafe`. (Lesson 34.)

**A13 — No: `error[E0133]`** ("dereference of raw pointer is unsafe and requires unsafe
block"). Creating the raw pointer (`&raw const num`) is fine; **dereferencing** it (`*r`) is
one of the five superpowers and must be inside an `unsafe { }` block. The fix is to wrap the
deref in `unsafe`. (Lesson 34.)

**A14 — (a) dereference; (b) call; (c) `static`; (d) implement; (e) `union`.** The five
`unsafe` superpowers: (a) **dereference** a raw pointer (`*const T` / `*mut T`); (b) **call**
an `unsafe` function or method; (c) access or modify a mutable **`static`** variable;
(d) **implement** an `unsafe` trait; (e) access the fields of a **`union`**. Crucially,
`unsafe` unlocks *only* these five — ownership, borrowing, and type-checking stay on (a
borrow violation beside an `unsafe` deref still fails, `E0502`). (Lesson 34.)

**A15 — `v = [1, 2, 3]`.** The pattern `$( $x:expr ),*` captures zero or more
comma-separated expressions; the body's `$( v.push($x); )*` emits one `push` line per
captured expression. So `my_vec![1, 2, 3]` expands — *before* compiling — into "make a
`Vec`, push 1, push 2, push 3, hand it back." The repetition in the body is what loops;
there's no `for`. (Lesson 34.)

**A16 — No: `error[E0382]`** ("use of moved value: `receiver`"). The first `spawn` *moves*
`receiver` into its closure; the next loop iteration has nothing left to move (a `Receiver`
isn't `Copy`). That's why several workers share *one* receiving end as **`Arc<Mutex<Receiver>>`**:
`Arc` gives multiple owners across threads, `Mutex` lets only one worker take a job at a time.
And `Box<dyn FnOnce() + Send + 'static>` is **a heap-boxed closure** — `dyn` because you don't
know its concrete type, `FnOnce` because the job runs once, `Send` so it can cross to another
thread, `'static` so it lives long enough. (Lesson 35.)

---

*How did you do?* Anything you missed points at the lesson to reread. You can now reach for a
trait object when you need a mixed collection behind one shared method (and know its
fat-pointer cost), read dense `match` blocks with guards / `@` / nested patterns and explain
refutability, recognise an `unsafe` block as a small audited hole, see how `macro_rules!`
turns one call into many lines, and reason about the capstone's `Arc<Mutex<Receiver>>` and
boxed `Job` without ever running the server. That closes the lesson series.

— *Sources:* questions written for this corpus from Lessons 32–35 (BOOK Ch.18 "OOP Features",
Ch.19 "Patterns and Matching", Ch.20 "Advanced Features" & the multithreaded-web-server final
project; CR trait-object / pattern / `unsafe` slides). Every trait-object, pattern, and
feature snippet was compiled **and run**, and every error code captured live, on **rustc
1.95.0**, edition 2024; the capstone snippet (Q16) was **compile-checked only** (it binds a
TCP port), so no server output is shown.
