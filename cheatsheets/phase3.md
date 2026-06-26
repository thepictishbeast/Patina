# Phase 3 Cheatsheet — Text & Collections

Quick reference (pairs with the Phase-3 lessons — String/&str · tuples · arrays · slices · Vec · HashMap). These are introduced **before** ownership (Phase 4), so think "owned vs a borrowed view" at the surface; the deep `&` rules come next phase. Verified on rustc 1.95.0, edition 2024.

## `String` vs `&str`
- `&str` = a **fixed, borrowed view** of text (every string literal is a `&str`). `String` = **owned and growable**, on the heap.
- Make a `String`: `String::from("hi")` or `"hi".to_string()`. Grow it: `push_str(" there")`, `a + &b`, or `format!("{a}-{b}")` (builds a fresh `String`, doesn't consume its inputs).
- Slice a `&str` view back out: `&owned[0..5]`.
- **No integer indexing:** `s[0]` is **`error[E0277]`** "the type `str` cannot be indexed by `{integer}`" (text is UTF-8). Use `s.chars().nth(0)` or a valid byte range.
- (`a + &b` **moves** `a` — a Phase-4 detail; reach for `format!` to keep both.)

## tuples — fixed group of mixed types
- `let t = (500, 6.4, 1);` — fixed length, types can differ (its **arity** is 3).
- Read: destructure `let (x, y, z) = t;` or by position `t.0` / `t.1` (0-based).
- `()` is the **unit** value — what an expression with no real value produces (the `;` from Lesson 6).

## arrays — fixed row of one type
- `let a = [10, 20, 30];` → type `[i32; 3]` (**length is part of the type**). Repeat form: `[0; 3]` → `[0, 0, 0]`.
- Index `a[0]`, count `a.len()`.
- Out-of-bounds, two times it's caught: a **literal** index the compiler can see (`a[5]`) = **compile error** (`#[deny(unconditional_panic)]`); a **computed** index = **runtime panic** "index out of bounds". *(Rule: the compiler catches what it can see; the runtime catches the rest.)*
- If it might need to grow, use a `Vec` instead.

## slices — a borrowed view of part of a sequence
- `&a[1..3]` is a slice; its type is `&[i32]` — **the length drops out**, so one function works on a slice of any size.
- A `&str` is just a slice of a `String`. Slicing **bytes** through the middle of a UTF-8 character **panics** ("not a char boundary") — slice on real boundaries.

## `Vec<T>` — growable list of one type
- `let mut v = vec![1, 2, 3];` (or `Vec::new()`). `v.push(4)`; `v.pop()` → `Option`.
- Read: `v[i]` (panics if out of range) **or** `v.get(i)` (→ `Option`, `None` if out of range) — your choice. `v[100]` panics at **run time** (a `Vec`'s length isn't known at compile time, unlike an array).
- Mutate in place: `for n in &mut v { *n += 50; }` (the `*` reaches the value).

## `HashMap<K, V>` — key → value lookup
- **Not in the prelude:** `use std::collections::HashMap;` (forget it → **`error[E0433]`** "cannot find type `HashMap`" — the compiler suggests the `use` line). The #1 beginner stumble.
- `m.insert(k, v);` · `m.get(&k)` → `Option`. Read with a fallback: `m.get(&k).copied().unwrap_or(0)`.
- Count/tally idiom: `*m.entry(k).or_insert(0) += 1;` (get the slot for `k`, inserting `0` if absent, then bump it).
- A `HashMap` is **unordered** — printing it gives an arbitrary order.

— *Sources:* BOOK §3.2 / §4 / §8 · CR §8–9, §17 · BLOG (String/Tuple/Array/Slice/Vector; it skips hash maps). Snippets verified on rustc 1.95.0, edition 2024. Deep borrowing / move errors (E0382, E0502) are Phase-4 material, foreshadowed only here.
