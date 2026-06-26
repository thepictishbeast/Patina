# Phase 7 Cheatsheet — Functional Features & Smart Pointers

Quick reference (pairs with the Phase-7 lessons — L27 closures · L28 iterators · L29 smart
pointers). The arc: **closures** are inline functions that capture their surroundings,
**iterators** are lazy cursors you stack named steps on, and **smart pointers** are values that
act like pointers but carry extra powers and clean up after themselves. Verified on rustc
1.95.0, edition 2024. *(The full `Fn`/`FnMut`/`FnOnce` trait mechanics are only sketched here —
the lessons stay on capture modes.)*

## Closures — anonymous functions that capture
- **Syntax:** bars hold the params, then the body. `|x| x + 1` · `|a, b| a + b` · `|| println!("hi")` (empty bars). Save in a variable, call with `()`: `let f = |x| x + 1; f(41);`.
- **Types are inferred** — you usually skip param/return types; the compiler reads them off the body and first use. (A named `fn` always spells them out; a closure needn't.)
- **Capture = the whole point.** A closure can use surrounding variables without them being passed in. *Three modes*, mirroring how a function takes a parameter: **borrow immutably** (body only reads) · **borrow mutably** (body changes it — both the value *and* the closure variable need `mut`) · **take ownership** (value moved in). **The body decides**; the compiler picks the **least demanding** mode. You never request one.
- **`move`** before the bars **forces ownership** of everything captured, even if a borrow would have done. Main use: handing the closure somewhere that outlives the scope — most often a **new thread** (`thread::spawn(move || …)`).
- **`Fn`/`FnMut`/`FnOnce`** (sketch): read-only capture → callable many times (`Fn`); mutates capture → `FnMut`; moves a captured value *out* / consumes it → can run only **once** (`FnOnce`).

## Closure pitfalls (real error codes)
- **Move a captured value *out* of a closure called repeatedly → `error[E0507]`** "cannot move out of `value`, a captured variable in an `FnMut` closure". `sort_by_key` calls the closure per element, so it must be reusable — but moving a non-`Copy` value out can happen only once. Fix: `value.clone()` inside.
- **Use a value after `move` took it → `error[E0382]`** "borrow of moved value". `move` transferred ownership into the closure; the old name is dead. Drop the `move` (borrow instead) or clone before moving.

## Iterators — lazy cursors
- **An iterator is a cursor.** The `Iterator` trait's one required method: `fn next(&mut self) -> Option<Self::Item>` — `Some(item)` for the next value, `None` when exhausted. Takes `&mut self` (advancing the cursor changes it), so the iterator must be **`mut`**.
- **Laziness rule (memorise):** making an iterator or stacking steps does **no work**. It's a *plan*, not a result — nothing runs until something pulls items out.
- **Adapters** return a new lazy iterator: **`map`** (transform each item) · **`filter`** (keep items that pass a test). Stacking them just builds a bigger plan.
- **Consumers** drive the iterator to the end (call `next` until `None`): **`sum`** · **`collect`** · a **`for`** loop. **A chain with no consumer does nothing** → `warning: unused 'Map' that must be used … iterators are lazy and do nothing unless consumed`.
- **Three ways to ask a collection for its iterator** (the box `for` hides — difference is *what you touch*): **`.iter()`** → `&T` read-only, collection survives · **`.iter_mut()`** → `&mut T` change in place (`*x` to reach through), collection survives · **`.into_iter()`** → `T` by value, **takes ownership**, collection consumed. A `for` picks one: `&v`→`iter`, `&mut v`→`iter_mut`, `v`→`into_iter`.
- **`collect`** runs the iterator and gathers results (usually a `Vec`); it builds several types so it needs to know which — type on the binding (`let v: Vec<i32> = …`) **or** turbofish (`.collect::<Vec<i32>>()`). A range (`1..=4`) is already an iterator — hang adapters straight off it.
- **Use a collection after `into_iter` ate it → `error[E0382]`** "borrow of moved value" ("`into_iter` takes ownership of the receiver `self`"). Want it again? Use `.iter()`, which only borrows.

## Smart pointers — values that act like pointers, with extra powers
- **`Box<T>`** — put a value on the **heap**; the box is a small fixed-size pointer. Beginner use: give a **self-containing type** a known size. A bare recursive type → `error[E0072]` "recursive type … has infinite size" (fix: wrap the recursive part in `Box<…>` — "insert some indirection").
- **`Rc<T>`** — **multiple owners** (reference counted). `Rc::clone(&a)` adds an owner and bumps the count by 1 (**cheap** — no deep copy); an owner leaving scope drops it by 1. Cleaned up only at **zero**. Watch it with `Rc::strong_count(&a)`. *(TV-in-a-family-room: last person out turns off the TV.)*
- **`RefCell<T>`** — **interior mutability**: change a value through a *shared, immutable-looking* handle. `*cell.borrow_mut() += 50` to write; `cell.borrow()` to read. The borrow rules (one mutable **or** many immutable) **still apply — but checked at runtime**: two live `borrow_mut`s **compile**, then **panic** (`RefCell already borrowed`). Reach for it only when you **can't** add `mut` (e.g. behind an `Rc` or a `&self` method) — else `let mut` is simpler.
- **`Rc<RefCell<T>>`** — the combo worth remembering: **many owners of one mutable value**. A change through one owner is visible through the others (one heap value, shared, mutable). A plain `let mut` can't give two owners — that's why this pairing exists.
- **`Deref`** & **`Drop`** — the two traits *underneath* all of these. **`Deref`** is why `*` follows a box just like a reference (`*y` on `&x`, `*z` on `Box::new(x)` — same `*`). **`Drop`** runs cleanup automatically at scope-end (you never call it yourself); values drop in **reverse** creation order; it's the hook that lets `Rc` decrease its count.

— *Sources:* BOOK Ch.13 (closures, iterators) · Ch.15 (smart pointers) · CR §26 + smart-pointer
slides. Snippets verified on rustc 1.95.0, edition 2024. Corpus metaphors (the iterator as a
**cursor**, `Rc` as a **TV in a family room**) are the lessons' own, carried across L27–29.
