# Lesson 3 — Shadowing

## 1. Why it exists

Lesson 2 ended on a wall: a `mut` number can't become text — the *kind* is locked
(`E0308`). But sometimes you genuinely want to reuse a good name while the value's
kind changes — read something as text, turn it into a number, and still call it
`reply`. Rust lets you do that **without** `mut`, by naming it again. That's
**shadowing**.

## 2. The idea

Shadowing means writing a **second `let` with the same name**:

```
let reply = "42";
let reply = reply.len();
```

The second `let` doesn't edit the first name — it makes a **brand-new name** that
*covers* (shadows) the old one from that line on. Because it's a new binding, two
things follow:

1. It can hold a **different kind** of value (text → number) — the thing `mut`
   couldn't do.
2. The new name is still **locked by default** (there's no `mut`); to change it
   again you'd shadow again.

Shadowing is not mutation. `mut` keeps one box and swaps its contents. Shadowing
builds a **new box** that happens to wear the same label.

## 3. Tiny examples to read

**Re-using a name through a little pipeline of changes:**

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x is {x}");
}
```

**Predict the line, then check:**

```
x is 12
```

Each `let x = ...` reads the *current* `x` and makes a new one: `5` → `6` → `12`.

**Changing the kind — text to number (the move `mut` can't make):**

```rust
fn main() {
    let reply = "   yes   ";
    let reply = reply.trim();
    let reply = reply.len();
    println!("reply is {reply} chars");
}
```

```
reply is 3 chars
```

`"   yes   "` (text) → trimmed text `"yes"` → its length `3` (a number), all under
one name, no `mut` anywhere.

## 4. Common pitfalls / real compiler errors

Shadowing obeys **scope** — a shadow made inside a `{ }` block only lasts inside
that block:

```rust
fn main() {
    let n = 5;
    {
        let n = n * 10;
        println!("inside:  {n}");
    }
    println!("outside: {n}");
}
```

**Predict both lines before scrolling:**

```
inside:  50
outside: 5
```

Inside the block, a new `n` (`50`) shadows the outer one. When the block ends, that
inner name is gone and the outer `n` (`5`) is visible again. The trap people fall
into: expecting `outside` to be `50`. It isn't — the inner shadow never touched the
outer name. (That "names live only inside their block" idea is **scope**; it shows
up in every later lesson.)

> No error code this time — shadowing is perfectly legal Rust. The pitfall here is
> *mental*: confusing a new shadow with changing the original value.

## 5. Predict-then-run practice (your turn — write this yourself)

`cargo new shadowing`. Predict before each run:

1. Take a name, then shadow it **twice** with arithmetic (like the `x` example, but
   your own numbers). Predict the final printed value before running.
2. Shadow a **text** value into a **number** (some text, then its `.len()`).
   Predict the number first. This is the move `mut` can't make — prove it to
   yourself.
3. *Contrast rep:* take a `let mut` number and try to assign text to it (no
   shadowing — just `name = "...";`). Predict the **error code** before running.
   (You saw it in Lesson 2.) This pins down the difference: shadow = a new `let`;
   mutate = a bare `=`.

*(Every line is yours. Predictions are your answer key.)*

## 6. What surprised you?

Did the block-scope result (`outside: 5`) match your guess? Is the shadow-vs-`mut`
distinction clear now, or still fuzzy? Tell me — Lesson 4 (`const`) leans on the
"locked" idea one more time before we move on to the *kinds* of values.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.1 "Variables and Mutability →
  Shadowing" (the `x` pipeline and the type-changing example).
- **CR** / **BLOG** — cited for contrast; both treat shadowing more briefly.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 2 — Mutability (`mut`)](02-mutability.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 4 — Constants (`const`) →](04-constants.md)
