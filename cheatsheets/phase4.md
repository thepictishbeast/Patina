# Phase 4 Cheatsheet — Ownership

Quick reference (pairs with the Phase-4 lessons — ownership & moves · references & borrowing · slices in depth). This is the heart of Rust: no garbage collector, no manual free — the compiler tracks who owns each value and frees it automatically, and proves you never use a value after it's gone. Verified on rustc 1.95.0, edition 2024.

## Ownership — the three rules
- Each value has **one owner**; only **one owner at a time**; when the owner goes **out of scope**, the value is **dropped** (freed automatically — no `free`, no double-free).
- Small fixed-size values (`i32`, `bool`, `char`, `f64`, tuples of those) live on the **stack**; growable ones (`String`, `Vec`) keep their data on the **heap**, with a little record (pointer/len/cap) on the stack — the "owner card."

## Move vs Copy vs Clone
- **Move:** `let s2 = s1;` on a `String` hands the owner card to `s2`; `s1` is now **invalid**. Using it → **`error[E0382]`** "borrow of moved value". (Passing to a function moves it the same way.)
- **Copy:** stack-only types duplicate instead of moving — `let y = x;` leaves `x` valid. (`Copy` and custom-`Drop` are mutually exclusive.)
- **Clone:** `let s2 = s1.clone();` makes a second independent `String` — a deliberate, **visible** cost. Reach for it (or borrowing) when a move gets in your way.

## References & borrowing — `&` lets you use without taking
- `&T` = **shared** (read-only) borrow — any number at once. `&mut T` = **exclusive** (mutable) borrow — change through it.
- **The two rules:** (1) at any time, **either one `&mut` or any number of `&`** — never both (*shared **xor** mutable*); (2) references must **always be valid**.
- A borrow lasts only until its **last use** (non-lexical lifetimes), so non-overlapping borrows don't conflict.
- Borrowing instead of moving lets a function read/modify a value and leave it with its owner.

## The borrow errors (the compiler catching real bugs)
- **`E0499`** — two `&mut` at once ("cannot borrow `x` as mutable more than once").
- **`E0502`** — a `&mut` while a `&` is still live ("cannot borrow `x` as mutable because it is also borrowed as immutable").
- **`E0106`** — returning a reference to a local (dangling); fix = **return the owned value** instead.

## Slices pin their collection
- A slice (`&s[0..5]`, `&v[1..3]`) is a **borrow** into the collection (type `&[T]` / `&str` — the length drops out).
- Because it's a borrow, **while the slice is alive you can't mutate the collection** — `s.clear()` or `v.push()` while a slice/reference is live → **`E0502`**.
- That's a feature: it turns a stale-index bug (a remembered position into data that then shifts) into a **compile error**. A returned slice borrows its source, so it can never go out of sync.

— *Metaphors (sourced):* a move = hand over the owner card and tear up your copy · borrowing = real-life borrowing (use it, give it back) · a slice = a window that **pins** the collection open while you look. *Sources:* BOOK §4 · CR §9/§20/§23. Snippets verified on rustc 1.95.0, edition 2024.
