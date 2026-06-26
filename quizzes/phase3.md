# Phase 3 Quiz — Text & Collections

A self-check for Phase 3 (Lessons 12–14: `String`/`&str`, tuples, arrays, slices,
`Vec`, `HashMap`). Same rule as before: **predict each answer before** you look at the
**Answers** section at the bottom — that prediction is the point. Don't run the code
first; predict, then verify. Ten questions.

> Tip: cover the Answers section until you've committed to an answer for every question.

---

## Questions

**Q1 — predict the output.**
```rust
let mut s = String::from("ab");
s.push_str("cd");
println!("{s}");
```

**Q2 — does this compile? If not, what's the error?**
```rust
let s = String::from("hi");
let c = s[0];
```

**Q3 — predict the output.**
```rust
let t = (1, "two", 3.0);
println!("{}", t.1);
```

**Q4 — predict the output.**
```rust
let a = [7; 3];
println!("{a:?}");
```

**Q5 — does this compile? If not, when does it fail (build time or run time)?**
```rust
let a = [1, 2, 3];
let x = a[5];
```

**Q6 — predict the output.**
```rust
let a = [10, 20, 30, 40];
let s = &a[1..3];
println!("{s:?}");
```

**Q7 — predict the output.**
```rust
let mut v = vec![1, 2, 3];
for n in &mut v { *n *= 2; }
println!("{v:?}");
```

**Q8 — predict the output.**
```rust
let v = vec![5];
println!("{:?}", v.get(9));
```

**Q9 — predict the output.**
```rust
use std::collections::HashMap;
let mut m = HashMap::new();
*m.entry("a").or_insert(0) += 1;
*m.entry("a").or_insert(0) += 1;
println!("{}", m["a"]);
```

**Q10 — short answer (concept).** (a) Indexing a `Vec` out of range does `____`, while
`.get()` on the same index returns `____`. (b) Of `String` and `&str`, which one is owned
and growable, and which is a fixed borrowed view?

---

## Answers

*(Verified on rustc 1.95.0, edition 2024.)*

**A1 — `abcd`.** A `String` is growable; `push_str` appends a `&str` onto the end.
(Lesson 12.)

**A2 — No, it doesn't compile.** `error[E0277]: the type ``str`` cannot be indexed by
``{integer}```. Text is UTF-8, so a single index is ambiguous (which byte? which
character?). Use `s.chars().nth(0)` for a character, or a byte *range* like `&s[0..1]`
that lands on a real boundary. (Lesson 12.)

**A3 — `two`.** `t.1` reaches the second tuple field by position (0-based), which is the
`&str` `"two"`. (Lesson 13.)

**A4 — `[7, 7, 7]`.** `[7; 3]` is the repeat form — three copies of `7`. (Lesson 13.)

**A5 — No; it fails at BUILD time.** The compiler can see the length (3) and the index (5)
right here, so it refuses: `error: this operation will panic at runtime …
#[deny(unconditional_panic)]`. (A *computed* index it can't see would instead panic at run
time — Lesson 13.)

**A6 — `[20, 30]`.** `&a[1..3]` is a slice (a `&[i32]` view) of elements 1 and 2 — the
range is exclusive, so it stops before index 3. (Lessons 10 & 13.)

**A7 — `[2, 4, 6]`.** Looping over `&mut v` gives mutable references; `*n *= 2` reaches the
value each one points at and doubles it. (Lesson 14.)

**A8 — `None`.** `.get(9)` on a 1-element `Vec` is out of range, so it returns `None`
(an `Option`) instead of panicking — that's the safe alternative to `v[9]`. (Lessons 11 & 14.)

**A9 — `2`.** The first `entry("a").or_insert(0)` inserts `0` then `+= 1` makes it `1`; the
second finds the existing entry and `+= 1` makes it `2`. `m["a"]` reads it back. (Lesson 14.)

**A10 — (a) panics / returns an `Option` (`None` when out of range).** (b) `String` is owned
and growable; `&str` is a fixed, borrowed view. (Lessons 12 & 14.)

---

*How did you do?* Anything you missed points at the lesson to reread. Next up is **Phase 4 —
Ownership**, which finally explains the `&` you've been using and the "this value was moved"
hints you saw foreshadowed all through Phase 3.

— *Sources:* questions written for this corpus from Lessons 12–14 (BOOK §3.2/§4/§8, CR, BLOG);
every code snippet compiled and run on **rustc 1.95.0**, edition 2024.
