# Phase 6 Quiz — Generics: `<T>`, Traits & Lifetimes

A self-check for the Phase-6 **Generics** trio (Lessons 24–26: generic type parameters
`<T>` in functions/structs/enums/methods + monomorphization; traits — defining, default vs
required methods, trait bounds `<T: Trait>` / `where`, `impl Trait`; lifetimes — `'a`
annotations, why a returned reference needs one, elision, and the dangling-reference errors).
Same rule as before: **predict each answer before** you look at the **Answers** section.
Don't run the code first; predict, then verify. Fifteen questions.

> Tip: cover the Answers section until you've committed to an answer for every question.
> (This is the *later* Phase-6 slice the Organizing quiz pointed you to — generics, traits,
> lifetimes.)

---

## Questions

**Q1 — concept.** When you compile generic code, Rust performs **monomorphization**. In one
sentence: what does it do, and therefore what does a generic cost you **at runtime** versus
what it costs in **binary size / compile time**?

**Q2 — predict the output.**
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest(&numbers));
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest(&chars));
}
```

**Q3 — does this compile? If so, predict the exact output.** (Watch the second line carefully.)
```rust
struct Point<T> { x: T, y: T }
impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
}
fn main() {
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 4.0 };
    println!("int x = {}", int_point.x());
    println!("float: x = {}, y = {}", float_point.x, float_point.y);
}
```

**Q4 — does this compile? If not, what's the error code?**
```rust
struct Point<T> { x: T, y: T }
impl Point<T> {
    fn x(&self) -> &T { &self.x }
}
fn main() {
    let p = Point { x: 5, y: 10 };
    println!("{}", p.x());
}
```

**Q5 — does this compile? If not, what's the error code?**
```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("{}", largest(&numbers));
}
```

**Q6 — does this compile? If so, predict the output.**
```rust
enum MyOption<T> { Some(T), None }
fn main() {
    let a: MyOption<i32> = MyOption::Some(5);
    let b: MyOption<f64> = MyOption::Some(2.5);
    let c: MyOption<i32> = MyOption::None;
    if let MyOption::Some(i) = a { println!("a holds {i}"); }
    if let MyOption::Some(f) = b { println!("b holds {f}"); }
    if let MyOption::None = c { println!("c holds nothing"); }
}
```

**Q7 — predict the output.** (`SocialPost` supplies only the required method; `NewsArticle`
overrides the default.)
```rust
trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
struct SocialPost { username: String }
impl Summary for SocialPost {
    fn summarize_author(&self) -> String { format!("@{}", self.username) }
}
struct NewsArticle { headline: String, author: String }
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String { self.author.clone() }
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.summarize_author())
    }
}
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    let post = SocialPost { username: String::from("horse_ebooks") };
    let article = NewsArticle {
        headline: String::from("Penguins win the Cup"),
        author: String::from("Iceburgh"),
    };
    notify(&post);
    notify(&article);
}
```

**Q8 — predict the output.** (Three ways to say "any `T` that is `Summary`," plus an
`impl Trait` return.)
```rust
trait Summary { fn summarize(&self) -> String; }
struct Tweet { who: String }
impl Summary for Tweet {
    fn summarize(&self) -> String { format!("tweet from {}", self.who) }
}
fn announce<T: Summary>(item: &T) { println!("Now: {}", item.summarize()); }
fn announce_where<T>(item: &T) where T: Summary { println!("Later: {}", item.summarize()); }
fn make_tweet() -> impl Summary { Tweet { who: String::from("ferris") } }
fn main() {
    let t = make_tweet();
    announce(&t);
    announce_where(&t);
}
```

**Q9 — does this compile? If not, what's the error code?**
```rust
trait Summary { fn summarize(&self) -> String; }
fn notify(item: &impl Summary) { println!("News! {}", item.summarize()); }
fn main() { notify(&5); }
```

**Q10 — does this compile? If not, what's the error code?** (Note the empty `impl` block.)
```rust
trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
struct Photo { caption: String }
impl Summary for Photo {}
fn main() {
    let p = Photo { caption: String::from("hi") };
    println!("{}", p.summarize());
}
```

**Q11 — does this compile? If so, predict the output.**
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
fn main() {
    let s1 = String::from("abcd");
    let s2 = String::from("xyz");
    let result = longest(s1.as_str(), s2.as_str());
    println!("The longest string is {result}");
}
```

**Q12 — does this compile? If not, what's the error code?** (No `main`; compiled as a library.)
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

**Q13 — does this compile? If not, what's the error code?** (No `main`; compiled as a library.)
```rust
struct Holder {
    text: &str,
}
```

**Q14 — does this compile? If not, what's the error code?** (No `main`; compiled as a library.)
```rust
fn make_owned(s: &str) -> &str {
    let owned = String::from(s);
    owned.as_str()
}
```

**Q15 — fill in the blanks (concept).** (a) A lifetime annotation `'a` does **not** change how
long any value lives; it only describes a `____` that the compiler then checks. (b) The
function `fn first_word(s: &str) -> &str` needs **no** written lifetime because the elision
rule for **exactly one input reference** gives that lifetime to `____`. (c) A method whose
only reference parameter is `&self` returns a reference that takes `____`'s lifetime.

---

## Answers

*(Verified on rustc 1.95.0, edition 2024.)*

**A1 — Monomorphization stamps out a separate, specialized copy of the generic code for each
concrete type you actually use**, so the `T` is gone by the time the program runs. Cost at
**runtime: nothing** (a generic is exactly as fast as the hand-written copies). Cost
elsewhere: the duplicated copies make the **binary larger** and the **compile slower**
(matters for tiny targets like embedded or WebAssembly). (Lesson 24.)

**A2 — `largest number: 100` / `largest char: y`.** One generic function serves both lists;
`T` becomes `i32` for the numbers and `char` for the chars. The `: PartialOrd` bound is what
lets the body use `>`. (Lesson 24.)

**A3 — Yes, it compiles. Output:**
```
int x = 5
float: x = 1.5, y = 4
```
The catch is the last value: `4.0` prints as **`4`**, not `4.0` — `Display` for floats drops
a trailing `.0`. `Point<T>` is one definition the compiler specializes for `i32` and `f64`;
note `impl<T> Point<T>` declares `T` after `impl` *and* names it in the type. (Lesson 24.)

**A4 — No: `error[E0425]`** ("cannot find type `T` in this scope"). `impl Point<T>` **uses**
`T` without **declaring** it — the placeholder must be introduced right after `impl`. The
compiler prints the fix verbatim: **`impl<T> Point<T>`**. (Lesson 24.)

**A5 — No: `error[E0369]`** ("binary operation `>` cannot be applied to type `&T`"). A bare
`T` can only do what *every* type can do, and not every type is orderable. The compiler
suggests restricting `T` with **`PartialOrd`** — that trait bound is the fix (and the bridge
to Lesson 25). (Lesson 24.)

**A6 — Yes, it compiles. Output:**
```
a holds 5
b holds 2.5
c holds nothing
```
`MyOption<i32>` and `MyOption<f64>` are two concrete enums built from one generic definition —
exactly how the real `Option<T>` is made. (Lesson 24.)

**A7 — `Breaking news! (Read more from @horse_ebooks...)` / `Breaking news! Penguins win the
Cup, by Iceburgh`.** `post` supplied only the **required** `summarize_author`, so it inherited
the **default** `summarize` (which called *its* author method); `article` **overrode**
`summarize` with its own. One `notify` served both. (Lesson 25.)

**A8 — `Now: tweet from ferris` / `Later: tweet from ferris`.** `<T: Summary>` and
`where T: Summary` compile to exactly the same constraint, so both lines print the same way;
`make_tweet`'s `-> impl Summary` hands back "some type that is `Summary`" (a `Tweet`) without
the caller naming it. (Lesson 25.)

**A9 — No: `error[E0277]`** ("the trait bound `{integer}: Summary` is not satisfied").
`notify` demands a type that implements `Summary`, and a plain integer doesn't — the compiler
names exactly what's missing and points at the bound that required it. This is the trait
bound doing its job at **compile** time. (Lesson 25.)

**A10 — No: `error[E0046]`** ("not all trait items implemented, missing:
`summarize_author`"). The compiler will take the **default** `summarize` off your hands, but
it won't invent the **required** `summarize_author` — that promise is yours to keep. (Lesson
25.)

**A11 — Yes, it compiles. Output: `The longest string is abcd`.** The `<'a>` says "the result
lives as long as the shorter of `x` and `y`." Here both live the whole `main`, so `result` is
valid. The annotation describes the relationship; it doesn't extend anything. (Lesson 26.)

**A12 — No: `error[E0106]`** ("missing lifetime specifier"). With **two** input references the
compiler can't tell whether the returned `&str` borrows from `x` or from `y`, so elision rule
2 (one input → all outputs) doesn't apply and you must write the shared `'a`:
`fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`. (Lesson 26.)

**A13 — No: `error[E0106]`** ("missing lifetime specifier"). A struct that **holds** a
reference must promise it won't outlive what it borrows — the same error, now on a field. Fix
it the `Excerpt<'a>` way: `struct Holder<'a> { text: &'a str }` (or simply **own** it with
`String`). (Lesson 26.)

**A14 — No: `error[E0515]`** ("cannot return value referencing local variable `owned`"). The
`String` is created **inside** the function and dropped at the closing brace, so a reference
to it would dangle. No annotation can fix this — you can return a reference that borrows from
an **input**, never one that borrows from a **local**. Return the owned `String` itself.
(Lesson 26.)

**A15 — (a) relationship** (a promise about how references' lifetimes relate, which the
compiler then checks — it changes nothing about how long values live); **(b) all outputs**
(the single input lifetime is given to every output, which is why `first_word` needs no
annotation); **(c) `self`** (when a parameter is `&self`/`&mut self`, the output gets `self`'s
lifetime). (Lesson 26.)

---

*How did you do?* Anything you missed points at the lesson to reread. You can now write one
generic definition that serves many types, name shared behavior with a trait and demand it
with a bound, and satisfy the borrow checker when a reference crosses a function or struct
boundary — the whole generics-traits-lifetimes toolkit. That closes Phase 6.

— *Sources:* questions written for this corpus from Lessons 24–26 (BOOK Ch.10.1–10.3, CR
Ch.13 & 24); every code snippet compiled (and the `main`-bearing ones run, the library ones
built with `--crate-type lib`) on **rustc 1.95.0**, edition 2024.
