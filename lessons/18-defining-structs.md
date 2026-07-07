# Lesson 18 — Structs: bundle your data

*(Phase 5 — Custom types & matching begins. Until now you've used types Rust gave
you. Now you make your own.)*

## 1. Why it exists

Real things have several parts that belong together: a user has a name *and* an age
*and* a status; a rectangle has a width *and* a height. Passing those around as
loose separate variables is error-prone. A **struct** groups them into one named
type — a template you stamp out instances from.

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
everything it needs. To change a field, the **whole instance** must be `mut`
(there's no per-field `mut`). And if you already have variables with the same names
as the fields, **field-init shorthand** lets you drop the repetition:
`Person { name, age }`.

**Three handy forms:**
- **Struct update** — fill the rest of a new instance from an existing one with
  `..`: `User { name: new_name, ..old }`. (One ownership subtlety about `..` moving
  fields is in the Book, §5.1 — read it when you reach for this.)
- **Tuple structs** — named, but fields are positional:
  `struct Color(i32, i32, i32);`, accessed `c.0`, `c.1`, … Good for giving a
  meaning to a bare tuple.
- **Unit structs** — no fields at all: `struct Marker;`. Useful later for marker
  types.

## 3. A tiny example to read

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

## 4. Common pitfalls / real compiler errors

**A borrowed field with no lifetime — `E0106`.** It's tempting to make a field a
`&str`:

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

A borrowed field has to promise it won't outlive what it borrows — that's a
*lifetime*, and it's a Phase-6 topic. For now, the right fix is to **own the
data**: make it `text: String`. (Owning fields is the default for exactly this
reason.)

And two walls the exercises below hand you on purpose: build an instance while
*forgetting a field* (the compiler lists every missing one), and reach for a field
that isn't yours to touch (struct fields have privacy too — the error names it
plainly). Predict each error code before you run.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the two matching
exercises via the **Practice this lesson** links at the bottom. *(On your own
machine, a playground or `cargo new structs` works too.)* **Predict on paper
before each run.**

1. **Define and use.** Make a `struct Book` with an owned title (`String`) and a
   `u32` page count. Build one, print both fields. Then make the binding `mut` and
   change the page count. **Predict** the output.
2. **Leave a field out** of the `Book { … }` you build. **Predict**: does it
   compile, and does the compiler name what's missing?

*(You write every line here — I won't. The predictions are your answer key. Next:
attaching behaviour to your struct with methods and `impl`.)*

## 6. What surprised you?

A sentence or two: did "the whole instance must be `mut`" feel odd or sensible?
Did owning fields (`String`, not `&str`) connect back to Phase 4? Tell me, and
I'll pitch Lesson 18b to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §5.1 "Defining and Instantiating
  Structs." The "definition = template" framing, field-init shorthand, struct
  update, tuple/unit structs, and the own-your-fields (`String` not `&str`)
  default.
- **CR** — *Comprehensive Rust* (Google), §10.1. The clean single worked struct.
  Its "no struct inheritance" point is kept as a plain fact; the cross-language
  comparison around it was dropped per the no-analogy rule.
- Compiler output captured live on **rustc 1.95.0** (edition 2024). Lifetimes (the
  real fix for a borrowed field) are Phase 6.

---

<!-- lesson-nav -->
[← Lesson 17 — Slices in Depth](17-slices-in-depth.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 18b — Methods & impl blocks →](18b-methods-and-impl.md)
