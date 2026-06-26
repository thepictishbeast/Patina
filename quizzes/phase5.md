# Phase 5 Quiz — Custom Types & Matching

A self-check for all of Phase 5 (Lessons 18–20: structs, methods/`impl`,
`derive(Debug)`, enums, `Option`, `match`, `if let`/`let…else`, and error handling —
`Result`, `?`, `panic!`, `unwrap`/`expect`). Same rule: **predict each answer before**
you look at the **Answers** section. Don't run the code first; predict, then verify.
Fifteen questions (1–10 cover L18/L19; 11–15 cover L20 error handling).

> Tip: cover the Answers section until you've committed to an answer for every question.

---

## Questions

**Q1 — predict the output.**
```rust
struct Point { x: i32, y: i32 }
let p = Point { x: 3, y: 7 };
println!("{}, {}", p.x, p.y);
```

**Q2 — does this compile? If not, what's the error code?**
```rust
struct Rectangle { width: u32, height: u32 }
let r = Rectangle { width: 30, height: 50 };
println!("{r:?}");
```

**Q3 — predict the output.**
```rust
struct Square { side: u32 }
impl Square {
    fn new(side: u32) -> Self { Self { side } }
    fn area(&self) -> u32 { self.side * self.side }
}
let s = Square::new(4);
println!("{}", s.area());
```

**Q4 — predict the output.**
```rust
enum Light { Red, Green, Yellow }
impl Light {
    fn action(&self) -> &str {
        match self {
            Light::Red => "stop",
            Light::Green => "go",
            Light::Yellow => "slow",
        }
    }
}
let l = Light::Green;
println!("{}", l.action());
```

**Q5 — does this compile? If not, what's the error code?**
```rust
let x = None;
println!("{x:?}");
```

**Q6 — does this compile? If not, what's the error code?**
```rust
fn plus_one(n: Option<i32>) -> i32 {
    match n {
        Some(x) => x + 1,
    }
}
```

**Q7 — predict the output.**
```rust
let n = Some(7);
let s = match n {
    Some(x) if x < 0 => "neg",
    Some(0..=5) => "small",
    Some(_) => "big",
    None => "none",
};
println!("{s}");
```

**Q8 — does this compile? If not, what's the error code?**
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y;
```

**Q9 — predict the output.**
```rust
if let Some(v) = Some(5) {
    println!("{v}");
} else {
    println!("none");
}
```

**Q10 — fill in the blanks (concept).** (a) A struct says a value has `____` of its
fields; an enum says a value is `____` of its variants. (b) Rust has no null; the type
that means "a value that might be missing" is `____`.

*(Questions 11–15 cover Lesson 20 — error handling.)*

**Q11 — predict the output.**
```rust
match "100".parse::<i32>() {
    Ok(n) => println!("ok {n}"),
    Err(_) => println!("bad"),
}
```

**Q12 — predict the output.**
```rust
let x: i32 = "abc".parse().unwrap_or(-1);
println!("{x}");
```

**Q13 — does this compile? If not, what's the error code?**
```rust
fn parse_it(s: &str) -> i32 {
    let n: i32 = s.parse()?;
    n
}
```

**Q14 — predict the output.**
```rust
fn triple(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 3)
}
println!("{:?}", triple("4"));
```

**Q15 — does this *compile*? And what happens when you *run* it?**
```rust
let n: i32 = "x".parse().unwrap();
println!("{n}");
```

---

## Answers

*(Verified on rustc 1.95.0, edition 2024.)*

**A1 — `3, 7`.** Field access with `.` reads each field of the instance. (Lesson 18.)

**A2 — No: `error[E0277]`** ("`Rectangle` doesn't implement `Debug`"). Your own types
can't be `{:?}`-printed until you add `#[derive(Debug)]` — which the compiler suggests by
name. (Lesson 18.)

**A3 — `16`.** `Square::new(4)` (an associated function) builds a `Square { side: 4 }`;
`.area()` (a `&self` method) returns `4 * 4`. (Lesson 18.)

**A4 — `go`.** `l` is `Light::Green`, so `match self` takes the `Green` arm. (Lesson 19.)

**A5 — No: `error[E0282]`** ("type annotations needed"). A bare `None` doesn't say what the
`Some` would hold, so the compiler can't infer the type — annotate it
(`let x: Option<i32> = None;`). (Lesson 19.)

**A6 — No: `error[E0004]`** ("non-exhaustive patterns: `None` not covered"). A `match` on an
`Option` must handle `None` too. (Lesson 19.)

**A7 — `big`.** `7` isn't negative and isn't in `0..=5`, so it falls to `Some(_) => "big"`.
(Lesson 19.)

**A8 — No: `error[E0277]`** ("cannot add `Option<i8>` to `i8`"). `y` might be `None`, so you
can't use it as a plain number — get the value out first. (Lesson 19.)

**A9 — `5`.** `if let` binds `v` to the inner `5` from `Some(5)` and runs the first block.
(Lesson 19.)

**A10 — (a) all / one; (b) `Option<T>`** (`Some(value)` or `None`). (Lessons 18 & 19.)

**A11 — `ok 100`.** `parse::<i32>()` returns a `Result`; `"100"` parses, so the `Ok(n)` arm
runs with `n = 100`. (Lesson 20.)

**A12 — `-1`.** `"abc"` is not a number, so `parse()` returns `Err`; `.unwrap_or(-1)` hands
back the fallback `-1` instead of the value. (Lesson 20.)

**A13 — No: `error[E0277]`** ("the `?` operator can only be used in a function that returns
`Result` or `Option`"). `parse_it` returns `i32`, which can't carry the error, so `?` has
nowhere to send an `Err`. Fix: return a `Result`, or handle it here with `match`. (Lesson 20.)

**A14 — `Ok(12)`.** `"4"` parses to `4`; `?` unwraps the `Ok` (no early return), then the
function returns `Ok(4 * 3)`. (Lesson 20.)

**A15 — It *compiles*, then *panics at runtime*.** `"x"` isn't a number, so `parse()` is an
`Err`, and `.unwrap()` panics with `called Result::unwrap() on an Err value: ParseIntError { kind: InvalidDigit }`.
`unwrap` is the blunt "I'm sure it's `Ok`" path — a crash when you're wrong. This compile-vs-run
split is the point: the type said it *could* fail, and `unwrap` chose to ignore that. (Lesson 20.)

---

*How did you do?* Anything you missed points at the lesson to reread. With structs, enums,
`Option`, `match`, and `Result`/`?`, you can now design types that only hold valid states *and*
write functions that fail honestly. That's all of Phase 5. Next: Phase 6 — organizing code with
modules, and generics.

— *Sources:* questions written for this corpus from Lessons 18–20 (BOOK §5–6 + §9, CR §10/§12/§13
+ error handling); every code snippet compiled and run on **rustc 1.95.0**, edition 2024.
