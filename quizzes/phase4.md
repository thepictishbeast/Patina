# Phase 4 Quiz — Ownership

A self-check for Phase 4 (Lessons 15–17: ownership & moves, references & borrowing,
slices in depth) — the heart of Rust. Same rule: **predict each answer before** you
look at the **Answers** section at the bottom. Don't run the code first; predict, then
verify. Ten questions.

> Tip: cover the Answers section until you've committed to an answer for every question.

---

## Questions

**Q1 — does this compile? If not, what's the error code?**
```rust
let s1 = String::from("hi");
let s2 = s1;
println!("{s2} {s1}");
```

**Q2 — predict the output.**
```rust
let x = 5;
let y = x;
println!("{x} and {y}");
```

**Q3 — predict the output.**
```rust
let a = String::from("hi");
let b = a.clone();
println!("{a}-{b}");
```

**Q4 — predict the output.**
```rust
fn show(s: &String) { println!("seen: {s}"); }
fn main() {
    let s = String::from("hi");
    show(&s);
    println!("kept: {s}");
}
```

**Q5 — does this compile? If not, what's the error code?**
```rust
let mut s = String::from("hi");
let r1 = &mut s;
let r2 = &mut s;
println!("{r1}{r2}");
```

**Q6 — does this compile? If not, what's the error code?**
```rust
let mut s = String::from("hi");
let r1 = &s;
let r2 = &mut s;
println!("{r1}{r2}");
```

**Q7 — does this compile? If not, what's the error code?**
```rust
fn make() -> &String {
    let s = String::from("hi");
    &s
}
```

**Q8 — does this compile? If not, what's the error code?**
```rust
let mut s = String::from("hi there");
let w = &s[0..2];
s.clear();
println!("{w}");
```

**Q9 — predict the output.**
```rust
let n = 5;
let m = n;
let s = String::from("x");
let t = s;
println!("{n} {m} {t}");
```

**Q10 — fill in the blanks (concept).** (a) At any one time you may have either `____`
mutable reference, or `____` immutable references — never both. (b) A slice borrows the
collection it views, so while the slice is alive you cannot `____` that collection.

---

## Answers

*(Verified on rustc 1.95.0, edition 2024.)*

**A1 — No: `error[E0382]`** ("borrow of moved value: `s1`"). `let s2 = s1;` *moves* the
`String` into `s2`, so `s1` is no longer valid. (Lesson 15. Fix: `.clone()`, or don't
use `s1` after.)

**A2 — `5 and 5`.** An `i32` is `Copy`, so `let y = x;` duplicates it and `x` stays
valid — no move. (Lesson 15.)

**A3 — `hi-hi`.** `.clone()` makes a second, independent `String`, so both `a` and `b`
are valid. (Lesson 15.)

**A4 — `seen: hi` then `kept: hi`.** `show(&s)` *borrows* `s` (it takes `&String`), so
`s` is still owned by `main` afterward. (Lesson 16.)

**A5 — No: `error[E0499]`** ("cannot borrow `s` as mutable more than once at a time").
Rule 1: only one `&mut` at a time. (Lesson 16.)

**A6 — No: `error[E0502]`** ("cannot borrow `s` as mutable because it is also borrowed as
immutable"). Shared **xor** mutable — you can't take a `&mut` while a `&` is still in
use. (Lesson 16.)

**A7 — No: `error[E0106]`** ("missing lifetime specifier"). You can't return a reference
to `s`, because `s` is dropped when `make` returns. Fix: return the owned `String`
(`-> String`, `s`). (Lesson 16.)

**A8 — No: `error[E0502]`.** `w` is a slice borrowing `s`, so `s.clear()` (which needs
`&mut s`) isn't allowed while `w` is still used. The slice *pins* the string — that's
what turns a stale-index bug into a compile error. (Lesson 17.)

**A9 — `5 5 x`.** `n` and `m` are `i32`s (`Copy`), so `n` stays valid after `let m = n;`.
`s` (a `String`) *moves* into `t`, but we never use `s` again, so there's no error — and
`t` prints `x`. (Lesson 15.)

**A10 — (a) one / any number. (b) mutate it** (e.g. `clear`, `push`, reassign). (Lessons
16 & 17.)

---

*How did you do?* Anything you missed points straight at the lesson to reread. Ownership
is the part of Rust that takes the most reps — don't worry if these took thought. Next up
is **Phase 5 — Custom types & matching** (structs, enums, `Option`, error handling), where
you start building types of your own.

— *Sources:* questions written for this corpus from Lessons 15–17 (BOOK §4, CR §9/§20/§23);
every code snippet compiled and run on **rustc 1.95.0**, edition 2024.
