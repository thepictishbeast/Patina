# Lesson 25b — Trait bounds: demanding behaviour from a generic `T`

*(Phase 6, part 4 — the payoff. Lesson 24 ended on a wall: `largest<T>` couldn't
use `>` because a bare `T` promises nothing. Lesson 25 taught types to make
promises. Now you connect them.)*

## 1. Why it exists

A generic function's body must be valid for *every* `T` — that was the rule, and
`E0369` was its teeth. A **trait bound** narrows the deal: "`T` can be any type
**as long as** it implements this trait." The body may then use everything the
trait promises, and the compiler rejects any caller whose type can't keep the
promise. This is the mechanism behind half the standard library's signatures.

## 2. The idea — one bound, three spellings

- **`impl Trait` parameter** — `fn notify(item: &impl Summary)`: "a reference to
  *anything* that implements `Summary`."
- **The generic bound** — `fn notify<T: Summary>(item: &T)`: the same thing in
  Lesson 24's generic form. `impl Trait` is just shorthand for this.
- **A `where` clause** — for longer or multiple bounds, move them off the
  signature line:

```
fn notify<T>(item: &T) where T: Summary { /* ... */ }
```

All three compile to exactly the same thing — pick whichever reads best.
(A fourth spelling, `-> impl Trait` as a *return* type, exists too — one line's
worth of knowledge for now; Book Ch. 10.2 when you need it.)

## 3. Tiny examples to read

**First: closing Lesson 24's wall.** The un-bounded `largest<T>` failed with
`E0369` because not every type can be ordered. Add the promise, and the exact
same body compiles:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("{} {}", largest(&numbers), largest(&chars));
}
```

```
100 y
```

`PartialOrd` is the standard library's "can be compared with `<`/`>`" trait —
exactly the promise the compiler asked you to add last time. The wall is closed.

**The three spellings, side by side.** All say "any type that is `Summary`":

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    who: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("tweet from {}", self.who)
    }
}

fn announce(item: &impl Summary) {         // impl Trait shorthand
    println!("Now: {}", item.summarize());
}

fn announce_bound<T: Summary>(item: &T) {  // the generic bound
    println!("Soon: {}", item.summarize());
}

fn announce_where<T>(item: &T)             // the where clause
where
    T: Summary,
{
    println!("Later: {}", item.summarize());
}

fn main() {
    let t = Tweet { who: String::from("ferris") };
    announce(&t);
    announce_bound(&t);
    announce_where(&t);
}
```

```
Now: tweet from ferris
Soon: tweet from ferris
Later: tweet from ferris
```

## 4. Common pitfalls / real compiler errors — the bound doing its job

Pass a type that doesn't implement the trait, and `E0277` names everything:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

fn notify(item: &impl Summary) {
    println!("News! {}", item.summarize());
}

fn main() {
    notify(&5);
}
```

```
error[E0277]: the trait bound `{integer}: Summary` is not satisfied
  --> main.rs:10:12
   |
10 |     notify(&5);
   |     ------ ^^ the trait `Summary` is not implemented for `{integer}`
   |     |
   |     required by a bound introduced by this call
   |
note: required by a bound in `notify`
```

The message names the type, the missing trait, and the bound that demanded it.
This is the same compile-time safety `Option` and `Result` gave you — now for
*behaviour*. (Two richer walls wait in the exercises: a bound on a *trait* — a
supertrait — and the orphan rule's `E0117`, with the newtype way around it.)

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then work through the matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new bounds` works too.)* **Predict on paper
before each run.**

1. **Close the wall yourself.** Take your Lesson-24 `largest<T>` (the one that
   failed) and add the bound the compiler suggested. **Predict**: does the same
   body now work on numbers AND chars?
2. **Three spellings.** Write `fn introduce(item: &impl Describe)` (from
   Lesson 25's practice), then rewrite it as `<T: Describe>` and with `where`.
   **Predict** whether the output changes. (It shouldn't — convince yourself
   *why*.)
3. **Trigger `E0277` on purpose.** Call `introduce(&5)`. **Predict the error
   code**, read which bound the compiler blames, fix by passing a real
   implementer.

*(You write every line here — I won't. The predictions are your answer key. With
bounds you can now name a behaviour, demand it, and let the compiler reject
anything that can't keep the promise. Next: lifetimes — the last of Phase 6's
three pillars.)*

## 6. What surprised you?

A sentence or two: did closing the `E0369` wall with one bound feel like the
loop it is? Did `notify(&5)` being rejected at *compile* time land as the same
safety `Option`/`Result` gave you, now for behaviour? Tell me, and I'll fold it
into the Phase-6 review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.10.2**: traits as parameters
  (`impl Trait`), trait bounds (`<T: Summary>`, `where`), `impl Trait` returns,
  and the `largest<T: PartialOrd>` resolution (Listing 10-15).
- **CR** — *Comprehensive Rust* (Google), §13.2.
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 25 — Traits: declare and implement](25-traits-declare-implement.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 26 — Lifetime annotations: the longest function →](26-lifetime-annotations.md)
