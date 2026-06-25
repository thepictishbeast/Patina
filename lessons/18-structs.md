# Lesson 18 — Structs (your own types)

*(Phase 5 — Custom types & matching begins. Until now you've used types Rust gave you.
Now you make your own — bundling related values into one named type, and attaching
behaviour to it.)*

## 1. Why it exists

Real things have several parts that belong together: a user has a name *and* an age *and*
a status; a rectangle has a width *and* a height. Passing those around as loose separate
variables is error-prone. A **struct** groups them into one named type — a template you
stamp out instances from — and `impl` lets you attach **methods** so the behaviour lives
with the data.

> **How the sources frame it:** the **BOOK** is the backbone — the "definition is a
> template, each instance fills it in" model, the `Rectangle` methods, and its signature
> "let-the-compiler-teach-you" walk for `#[derive(Debug)]`; **CR** has the cleanest single
> worked struct and the method example that shows every kind of receiver.

## 2. The idea

**Define and instantiate.** A `struct` names a type and lists its fields:

```
struct Person {
    name: String,
    age: u32,
}
let p = Person { name: String::from("Avery"), age: 30 };
let who = p.name;   // field access with .
```

Notice the fields *own* their data (`String`, not `&str`) — so each `Person` owns
everything it needs (a `&str` field would need a lifetime, which is Phase 6). To change a
field, the **whole instance** must be `mut` (there's no per-field `mut`). And if you
already have variables with the same names as the fields, **field-init shorthand** lets you
drop the repetition: `Person { name, age }`.

**Three handy forms:**
- **Struct update** — fill the rest of a new instance from an existing one with `..`:
  `User { name: new_name, ..old }`. (Heads-up from Phase 4: if a non-`Copy` field comes from
  `old` via `..`, that *moves* `old`.)
- **Tuple structs** — named, but fields are positional: `struct Color(i32, i32, i32);`,
  accessed `c.0`, `c.1`, … Good for giving a meaning to a bare tuple.
- **Unit structs** — no fields at all: `struct Marker;`. Useful later for marker types.

**Methods and `impl`.** Behaviour attaches to a type in an `impl` block. The first
parameter is the **receiver**, and which one you pick mirrors the borrow intent from
Phase 4:

- `&self` — borrow to **read** (most methods).
- `&mut self` — borrow to **change** the instance.
- `self` — **consume** it (the instance is moved into the method; using it afterward is the
  Phase-4 `E0382`).

A function in `impl` with **no** receiver is an **associated function** — called with `::`,
often used to build instances (`Rectangle::square(3)`). `new` is just a *convention* for
this, not a keyword. `Self` is shorthand for the type's own name.

**`#[derive(Debug)]`.** Your own types can't be printed with `{}` or `{:?}` until you say
so. Adding `#[derive(Debug)]` above the struct makes `{:?}` (and pretty `{:#?}`) work — and
if you forget, the compiler tells you the exact line to add. (`dbg!(x)` prints a value to
stderr and hands it back, handy for quick checks.)

## 3. Tiny examples to read

**Build one, with shorthand, and change a field.** Predict the line:

```rust
struct Person {
    name: String,
    age: u32,
}
fn main() {
    let name = String::from("Avery");
    let age = 30;
    let mut p = Person { name, age };   // field-init shorthand
    p.age += 1;                          // whole instance is mut
    println!("{} is {}", p.name, p.age);
}
```

```
Avery is 31
```

**A method and an associated function (30-second rep — you type this).** Predict both
lines:

```rust
struct Rectangle { width: u32, height: u32 }
impl Rectangle {
    fn area(&self) -> u32 {                 // &self: reads the instance
        self.width * self.height
    }
    fn square(size: u32) -> Self {          // associated fn: builds one
        Self { width: size, height: size }
    }
}
fn main() {
    let r = Rectangle { width: 30, height: 50 };
    println!("area = {}", r.area());
    let sq = Rectangle::square(10);         // called with ::
    println!("square area = {}", sq.area());
}
```

```
area = 1500
square area = 100
```

**Print your own type with `#[derive(Debug)]`:**

```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }
fn main() {
    let r = Rectangle { width: 30, height: 50 };
    println!("{r:?}");    // compact
    println!("{r:#?}");   // pretty
}
```

```
Rectangle { width: 30, height: 50 }
Rectangle {
    width: 30,
    height: 50,
}
```

*(That `Rectangle` block was your write-rep; part 5 is the rest.)*

## 4. Common pitfalls / real compiler errors

**Trying to debug-print without deriving it — `E0277`.** Drop the `#[derive(Debug)]` and:

```rust
struct Rectangle { width: u32, height: u32 }
fn main() {
    let r = Rectangle { width: 30, height: 50 };
    println!("{r:?}");
}
```

```
error[E0277]: `Rectangle` doesn't implement `Debug`
 --> main.rs:4:15
  |
4 |     println!("{r:?}");
  |               ^^^^^ `Rectangle` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  …
help: consider annotating `Rectangle` with `#[derive(Debug)]`
  |
1 + #[derive(Debug)]
2 | struct Rectangle { width: u32, height: u32 }
```

The fix is literally printed for you: add `#[derive(Debug)]` above the struct.

**A borrowed field with no lifetime — `E0106`.** It's tempting to make a field a `&str`:

```rust
struct Holder {
    text: &str,
}
```

```
error[E0106]: missing lifetime specifier
 --> main.rs:2:11
  |
2 |     text: &str,
  |           ^ expected named lifetime parameter
```

A borrowed field has to promise it won't outlive what it borrows — that's a *lifetime*,
and it's a Phase-6 topic. For now, the right fix is to **own the data**: make it
`text: String`. (Owning fields is the default for exactly this reason.)

> A quick Phase-4 callback: a method that takes `self` (not `&self`) **consumes** the
> instance — using it after that call is the `E0382` move error you already know.

## 5. Predict-then-run practice (your turn — write this yourself)

Open a fresh playground or `cargo new structs`. **Predict on paper before each run.**

1. **Define and use.** Make a `struct Book` with an owned title (`String`) and a `u32`
   page count. Build one, print both fields. Then make the binding `mut` and change the
   page count. **Predict** the output.

2. **A method + an associated function.** Add an `impl Book` with a method `&self` that
   returns whether the book is "long" (say, over 300 pages), and an associated function
   that builds a default `Book`. Call both. **Predict** the results.

3. **Derive, then break it.** Add `#[derive(Debug)]` and print your `Book` with `{:#?}`.
   Then *remove* the derive and print with `{:?}` — **predict** the error code and the
   exact line the compiler will tell you to add.

*(You write every line here — I won't. The predictions are your answer key; the code is
yours. Next lesson: enums and `match` in depth — the other half of modelling your data.)*

## 6. What surprised you?

A sentence or two: did "the receiver (`&self` / `&mut self` / `self`) is just the borrow
rules again" connect to Phase 4? Was the compiler handing you the `#[derive(Debug)]` line
a surprise? Tell me, and I'll pitch Lesson 19 (enums + matching) to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §5.1 "Defining and Instantiating Structs",
  §5.2 "An Example Program" (the `#[derive(Debug)]` "let the compiler teach you"
  fail-then-fix and `dbg!`), §5.3 "Method Syntax" (`&self`/`&mut self`/`self`, associated
  functions, `Self`, multiple `impl` blocks). The "definition = template" framing and the
  own-your-fields (`String` not `&str`) default.
- **CR** — *Comprehensive Rust* (Google), §10.1–10.2 and §13.1/§13.3. The clean single
  worked struct, the method example showing all receiver kinds, and "why derive" (the macro
  as shorthand for a hand-written impl). Its "no struct inheritance" point is kept as a
  plain fact; the cross-language comparison around it was dropped per the no-analogy rule.
- **BLOG** — punts on structs; the concept is sourced from BOOK/CR.
- Compiler output captured live on **rustc 1.95.0** (edition 2024). Lifetimes (the real fix
  for a borrowed field) are Phase 6.

---

<!-- lesson-nav -->
[← Lesson 17 — Slices in Depth](17-slices-in-depth.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 19 — Enums & Matching →](19-enums-and-matching.md)
