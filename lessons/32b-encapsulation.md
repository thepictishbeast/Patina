# Lesson 32b — Encapsulation: private fields, public methods

*(Phase 9, part 2. Lesson 32 handled mixed types; this lesson is about hiding.
You met privacy in Lesson 22 as a module tool — here it becomes an API tool:
keep a struct's **fields** private, expose **methods**, and the inside of your
type is yours to change forever.)*

## 1. Why it exists

Imagine a collection that always knows its own average. If callers can reach
the fields directly, any of them can push a value into the list and *forget to
update the average* — the struct's one promise, silently broken by code you
don't control.

**Encapsulation** closes that door. Make the **fields private** and expose only
**methods**. Now every change goes through code *you* wrote, so the promise
(list and average always in step) is enforced in exactly one place. And because
callers depend on the methods, not the layout, you can later change the
inside — swap the `Vec` for something else, compute the average differently —
**without breaking any caller**. The hidden field is a promise you're free to
keep however you like.

## 2. The idea

- Fields with no `pub` are **private**: only code in the same module (Lesson
  22's boundary) can touch them.
- Public **methods** are the type's contract: `add` and `average` are promises;
  `list` and the cached `average` field are details.
- A private helper (`update_average` below) keeps the invariant in one place —
  callers can't even see it, let alone skip it.

One honest caveat from Lesson 22: privacy is enforced at **module
boundaries**. Inside one module (like a single-file example where everything
sits together), the compiler lets code touch the fields — the wall appears the
moment the struct lives in its own module, which is where real types live.
Part 4 shows exactly that wall.

> **How the sources frame it:** the **BOOK** Ch.18 §18.1 is this lesson — the
> `AveragedCollection` example and the point that Rust's `pub`/private rules
> give you everything "encapsulation" means elsewhere, no classes needed.

## 3. A tiny example to read

**Private fields, public methods (BOOK).** The average is kept correct
internally, and the *how* is hidden — you could change it later without
touching any caller:

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

## 4. Common pitfalls / real compiler errors — the wall, made real

Put the struct behind a module boundary (as it would be in a real project) and
try to reach past the methods into a field:

```rust
mod stats {
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
}

fn main() {
    let mut c = stats::AveragedCollection::new();
    c.add(10);
    c.list.push(999);   // reach past the methods, straight into the field
    println!("average = {}", c.average());
}
```

**Before you scroll — which line fails, and what does it protect?**

```
error[E0616]: field `list` of struct `AveragedCollection` is private
  --> main.rs:31:7
   |
31 |     c.list.push(999);   // reach past the methods, straight into the field
   |       ^^^^ private field
```

`E0616` is encapsulation working. That `push(999)` would have grown the list
*without* updating the average — the exact bug the type exists to prevent. Note
the struct itself is `pub` (callers can *have* one) while its fields are not
(callers can't *reach inside* one). Making the field `pub` would "fix" the
error and break the promise; the real fix is to go through `add`.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new encapsulation` works too.)* **Predict on paper before
each run.**

1. **A type with one promise.** Build a `Counter` in a `mod counting { … }`:
   one private field `count: u32`, a `pub fn new()`, a `pub fn bump(&mut self)`
   that adds 1, and a `pub fn count(&self) -> u32`. In `main`, make one, bump
   it three times, print the count. **Predict the output.**
2. **Hit the wall.** Add `c.count += 10;` in `main`. **Predict the error code**
   and which word in the message names the protection. Then delete the line —
   the methods are the only door.
3. **Change the inside, keep the promise.** Change the private field to count
   *downward from 100* (store `remaining: u32`, make `bump` subtract, make
   `count()` return `100 - remaining`). Don't touch `main` at all. **Predict**:
   does `main` still compile and print the same number? What does that tell you
   about who depends on what?

*(You write every line here — I won't. The predictions are your answer key.
Task 3 is the whole argument: the inside changed completely and no caller
noticed. Next, Lesson 32c — the sharpest move in the chapter: states the
compiler won't even let you write.)*

## 6. What surprised you?

A sentence or two: did `E0616` feel like a restriction or a protection — and
did task 3 (rebuilding the inside without touching `main`) change your answer?
Would you have caught the `push(999)` bug at runtime in a language where every
field is reachable? Tell me, and I'll fold it into the Phase-9 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.18 §18.1** "Characteristics
  of Object-Oriented Languages": encapsulation via `pub`/private, the
  `AveragedCollection` example (reproduced here), and the point that changing
  hidden internals can't break callers.
- **CR** — *Comprehensive Rust* (Google): touches privacy in its modules
  section; the encapsulation framing is the Book's.
- Every snippet compiled and run, and the error captured live, on **rustc
  1.95.0**, edition 2024 (`rustc --edition 2024 FILE.rs`; temp path normalized
  to `main.rs`).

---

<!-- lesson-nav -->
[← Lesson 32 — Trait objects: one collection, many types](32-trait-objects.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 32c — States as types: broken states won't compile →](32c-states-as-types.md)
