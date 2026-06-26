# Phase 7 Quiz — Functional Features & Smart Pointers

A self-check for the Phase-7 lessons (Lessons 27–29: **closures** — capture & `move`;
**iterators** — laziness, adapters/consumers, `iter`/`iter_mut`/`into_iter`; **smart
pointers** — `Box`, `Rc`, `RefCell`, `Deref`/`Drop`). Same rule as before: **predict each
answer before** you look at the **Answers** section. Don't run the code first; predict, then
verify. Fourteen questions.

> Tip: cover the Answers section until you've committed to an answer for every question.
> (The full `Fn`/`FnMut`/`FnOnce` trait *mechanics* are only sketched in the lessons — these
> questions stay at the level the lessons teach: which capture mode maps to which trait.)

---

## Questions

**Q1 — concept.** A closure can grab a variable from the surrounding code in **three** ways,
mirroring the three ways a function takes a parameter. Name all three, and say **who decides**
which one a given closure uses.

**Q2 — predict the output.**
```rust
fn main() {
    let list = vec![1, 2, 3];
    let only_borrows = || println!("closure: {list:?}");
    println!("before: {list:?}");
    only_borrows();
    println!("after:  {list:?}");
}
```

**Q3 — predict the output.**
```rust
fn main() {
    let add_one = |x| x + 1;
    let nums = [1, 2, 3];
    let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();
    println!("{}", add_one(41));
    println!("{doubled:?}");
}
```

**Q4 — does this compile? If not, what's the error code?**
```rust
fn main() {
    let list = vec![1, 2, 3];
    let owns_it = move || println!("from closure: {list:?}");
    owns_it();
    println!("back in main: {list:?}");
}
```

**Q5 — concept.** This closure makes `sort_by_key` refuse to compile with **`error[E0507]`:
cannot move out of `value`, a captured variable in an `FnMut` closure**. In one sentence: why
does the closure being **`FnMut`** (called many times) clash with what the body does to
`value`?
```rust
list.sort_by_key(|n| {
    log.push(value);   // value: String
    *n
});
```

**Q6 — predict the output.**
```rust
fn main() {
    let v = vec![10, 20];
    let mut it = v.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}
```

**Q7 — predict the output.**
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let result: Vec<i32> = v
        .iter()
        .copied()
        .filter(|x| x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    println!("{result:?}");
}
```

**Q8 — does this compile, and what does it print?** (Mind the laziness rule.)
```rust
fn main() {
    let v = vec![1, 2, 3];
    v.iter().map(|x| x * 2);
    println!("done");
}
```

**Q9 — concept.** A vector `v` gives you three ways to ask for an iterator: `v.iter()`,
`v.iter_mut()`, and `v.into_iter()`. For each, say **what each item hands you** (the type) and
**whether `v` survives** afterward.

**Q10 — does this compile? If not, what's the error code?**
```rust
fn main() {
    let v = vec![1, 2, 3];
    let _doubled: Vec<i32> = v.into_iter().map(|x| x * 2).collect();
    println!("{v:?}");
}
```

**Q11 — predict the output.**
```rust
fn main() {
    let doubled = (1..=4).map(|n| n * 2).collect::<Vec<i32>>();
    println!("{doubled:?}");
}
```

**Q12 — does this compile? If not, what's the error code?**
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

**Q13 — predict the output** (four numbers, in order).
```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("hi"));
    println!("{}", Rc::strong_count(&a));
    let _b = Rc::clone(&a);
    println!("{}", Rc::strong_count(&a));
    {
        let _c = Rc::clone(&a);
        println!("{}", Rc::strong_count(&a));
    }
    println!("{}", Rc::strong_count(&a));
}
```

**Q14 — does this *compile*? And if it compiles, does it *run* cleanly — or panic?** (Be
careful: this one is not about an error *code*.)
```rust
use std::cell::RefCell;

fn main() {
    let balance = RefCell::new(100);
    let mut one = balance.borrow_mut();
    let mut two = balance.borrow_mut();
    *one += 50;
    *two += 50;
}
```

---

## Answers

*(Verified on rustc 1.95.0, edition 2024.)*

**A1 — borrow immutably, borrow mutably, or take ownership** (`move`). The **body decides**:
the compiler reads what the body does with the variable and picks the **least demanding** mode
that works (only reading → immutable borrow; changing it → mutable borrow; needing to own it →
move). You never request a mode — `move` is the one override, forcing ownership. (Lesson 27.)

**A2 — `before: [1, 2, 3]` / `closure: [1, 2, 3]` / `after:  [1, 2, 3]`.** The body only
*reads* `list`, so the closure captures it by **immutable borrow** — `list` stays usable both
before and after the call. (Lesson 27.)

**A3 — `42` then `[2, 4, 6]`.** `add_one(41)` → `42`; `.iter().map(|n| n * 2).collect()`
doubles each element into a new `Vec`. The `map` closure captures *nothing* — it just
transforms its argument. (Lesson 27.)

**A4 — No: `error[E0382]`** ("borrow of moved value: `list`"). `move` forces the closure to
**take ownership** of `list`, so after `owns_it` is built `main` no longer owns it — the last
`println!` tries to use a value that's already moved. (Drop the `move` and the closure borrows
instead, leaving `list` usable.) (Lesson 27.)

**A5 — Because an `FnMut` closure can be called *many* times, but moving `value` out can only
happen *once*.** `log.push(value)` *gives away* the `String` (it's not `Copy`), so the second
call would have nothing left to move — the compiler rejects it up front. The fix is
`value.clone()`, handing each call its own copy. (Lesson 27.)

**A6 — `Some(10)` / `Some(20)` / `None`.** Each `next` advances the cursor one step and returns
`Some(item)`; once the two items are exhausted, the third call returns `None` — the same `None`
a `for` loop watches for. (`it` must be `mut` because `next` takes `&mut self`.) (Lesson 28.)

**A7 — `[4, 8, 12]`.** `filter` keeps the evens (`2, 4, 6`), `map` doubles them (`4, 8, 12`),
and `collect` runs the chain into a `Vec`. `filter`/`map` are lazy adapters that build the
plan; the single `collect` is the consumer that drives it. (Lesson 28.)

**A8 — It compiles (just a warning) and prints only `done`.** The doubling **never happens**:
`map` is a lazy adapter with no consumer on the end, so it does nothing. The compiler warns
*"unused `Map` that must be used … iterators are lazy and do nothing unless consumed."*
(Lesson 28.)

**A9 — `iter()` hands you `&T` (read-only), `v` survives. `iter_mut()` hands you `&mut T`
(change items in place), `v` survives. `into_iter()` hands you `T` by value (takes ownership),
`v` is consumed and gone afterward.** A `for` loop picks one for you: `&v` → `iter`, `&mut v` →
`iter_mut`, plain `v` → `into_iter`. (Lesson 28.)

**A10 — No: `error[E0382]`** ("borrow of moved value: `v`"). Distinct from Q4: here the move
isn't a `move` closure — `into_iter` **takes ownership of its receiver** (`self`), so `v` is
moved by the method call and can't be used on the next line. If you need `v` afterward, use
`.iter()` (which only borrows). (Lesson 28.)

**A11 — `[2, 4, 6, 8]`.** A range like `1..=4` is itself an iterator, so adapters hang straight
off it — no `.iter()` needed. The turbofish `collect::<Vec<i32>>()` puts the target type on
`collect` instead of on the `let` binding; same result either way. (Lesson 28.)

**A12 — No: `error[E0072]`** ("recursive type `List` has infinite size"). A `List` that
contains a bare `List` has no knowable size — the compiler needs **indirection**. The fix it
suggests is `Box<List>` (a fixed-size pointer): *store a pointer to the value instead of the
value itself.* (Lesson 29.)

**A13 — `1` / `2` / `3` / `2`.** `Rc::strong_count` is `1` at creation, `2` after `_b`'s clone,
`3` inside the block after `_c`'s clone — then `_c` goes out of scope at the block's end, so the
count falls back to `2`. The value is cleaned up only when the count hits **zero** (the last
person leaves the room and turns off the TV). (Lesson 29.)

**A14 — It *compiles*, but it *panics at runtime* — there is no error code.** Two live
`borrow_mut` handles break the borrow rule "one mutable borrow at a time," but `RefCell` checks
that rule at **runtime**, not compile time. So instead of a compile error you get a crash:
`thread 'main' panicked … RefCell already borrowed`. (With a plain `&mut` this same mistake
would be a *compile* error you couldn't run past — that's the trade-off `RefCell` makes.)
(Lesson 29.)

---

*How did you do?* Anything you missed points at the lesson to reread. You can now write
behaviour inline with **closures**, transform data streams lazily with **iterators**, and reach
past single ownership with **smart pointers** — heap storage (`Box`), shared owners (`Rc`), and
safe-but-checked-at-runtime mutation (`RefCell`). That closes the Phase-7 toolkit.

— *Sources:* questions written for this corpus from Lessons 27–29 (BOOK Ch.13 & Ch.15, CR
§26 & smart-pointer slides); every code snippet compiled (and the `main`-bearing ones run) on
**rustc 1.95.0**, edition 2024. The two `E0382`s (Q4, Q10) are kept on purpose — they show the
*same* error code from two *different* moves (a `move` closure vs `into_iter` taking its
receiver).
