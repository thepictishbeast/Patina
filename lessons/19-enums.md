# Lesson 19 — Enums: one-of types

*(Phase 5 — Custom types & matching, part 2. Structs (Lesson 18) group values that
go *together*. Enums model a value that is *one of* several shapes.)*

## 1. Why it exists

A struct says "a thing has **all of** these." An **enum** says "a thing is **one
of** these." A message is *either* a quit *or* a move *or* some text; a lookup
*either* found a value *or* didn't. Modelling that with an enum lets the compiler
know exactly which shapes exist — and, as you'll keep seeing all through this
phase, make you handle every one of them.

## 2. The idea

**Enums and their data.** Variants can be empty, or carry data — and the shapes
can mix:

```
enum Message {
    Quit,                        // no data
    Move { x: i32, y: i32 },     // struct-like
    Write(String),               // tuple-like
    ChangeColor(i32, i32, i32),  // tuple-like
}
```

A variant name is really a **constructor**: `Message::Write(String::from("hi"))`
makes one. You attach methods with `impl` — exactly like a struct — and the body
usually `match`es on `self`, with each arm naming one variant's shape. (The full
pattern vocabulary — guards, ranges, `|` — is Lesson 19c; here the arms just name
shapes and bind their data.)

## 3. A tiny example to read

**An enum with a method (`match self`).** Predict the four lines:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn describe(&self) -> String {
        match self {
            Message::Quit => String::from("quit"),
            Message::Move { x, y } => format!("move to ({x}, {y})"),
            Message::Write(text) => format!("write: {text}"),
            Message::ChangeColor(r, g, b) => format!("color ({r}, {g}, {b})"),
        }
    }
}
fn main() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("hi")),
        Message::ChangeColor(255, 0, 0),
    ];
    for m in &msgs {
        println!("{}", m.describe());
    }
}
```

```
quit
move to (1, 2)
write: hi
color (255, 0, 0)
```

## 4. Common pitfalls / real compiler errors — a pattern must match the variant's *shape*

Each arm's pattern has to fit how the variant was declared. Write
`Message::Quit(x)` — as if `Quit` carried data — and the compiler names the
mismatch (`E0532`, "expected tuple struct or tuple variant, found unit variant");
match a data-carrying variant *without* binding its data and it likewise objects.
The matching exercise below hands you exactly this wall — **predict the error
code**, then read which shape the compiler says the variant really has.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new enums` works too.)* **Predict on paper before
each run.**

1. **An enum with a method.** Make an enum `Shape` with at least two data-carrying
   variants (e.g. `Circle(f64)`, `Rectangle { w: f64, h: f64 }`). Add an `impl`
   with an `area(&self)` method that `match`es on `self`. Build a couple and print
   their areas. **Predict** the output.
2. **Mismatch a shape on purpose.** In your `match`, try binding data out of a
   variant that has none. **Predict** the error code first.

*(You write every line here — I won't. The predictions are your answer key. Next:
the most important enum in Rust — `Option<T>`, and why Rust has no null.)*

## 6. What surprised you?

A sentence or two: did "a variant name is a constructor" demystify things? Did
enum-with-a-method feel natural after Lesson 18b? Tell me, and I'll pitch
Lesson 19b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §6.1 (defining enums, data-carrying
  variants).
- **CR** — *Comprehensive Rust* (Google), §10.3 (the three variant kinds). The
  enum-with-a-method (`match self`) example was synthesis-authored to fill a
  catalog gap (no source had one); it was compiled like every other snippet.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 18c — Printing your own types](18c-derive-debug.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 19b — Option: Rust has no null →](19b-option.md)
