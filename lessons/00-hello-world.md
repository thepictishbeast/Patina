# Lesson 0 — Hello, World (your first program)

## 1. Why it exists

Before anything else, a program needs two things: a place to **start**, and a way
to **show you something**. This first tiny program gives you both. It proves your
setup actually works, and it introduces the three small pieces that appear in
*every* Rust program you'll ever write. Nothing here is throwaway — you'll use all
of it in the next lesson and forever after.

## 2. The idea

Here is the whole program. Read it top to bottom; we'll name every piece right
after.

```rust
fn main() {
    println!("Hello, world!");
}
```

Four things are happening, and each has a name worth knowing:

1. **`fn main`** — the starting point. When you run a Rust program, Rust looks for
   something called `main` and runs whatever is inside it. `fn` is short for
   *function* — a named group of steps. You can make many functions later and give
   them any name; `main` is the one special name Rust always starts from. (We'll go
   deeper on writing your own functions later — for now, just know `main` is where
   your program begins.)
2. **`{ }`** — the curly braces hold the **body**: the list of steps that belong to
   `main`. Everything between them runs, top to bottom, when the program starts.
3. **`println!`** — this prints a line of text to the screen. Look closely at the
   `!` on the end — that is not a typo. The `!` means `println!` is a **macro**, not
   an ordinary function. A macro is a shortcut the compiler expands into more code
   for you. You don't need to know *how* it expands yet — just that in Rust, a name
   ending in `!` is a macro, and `println!` is the one that prints a line. Leave the
   `!` off and it won't work (you'll see exactly that in part 4).
4. **`;`** — the semicolon ends a **statement** — one complete instruction. Most
   lines inside `main` end with a `;`. Think of it as the full stop at the end of a
   sentence: it tells Rust "this instruction is finished."

That's the skeleton of every Rust program: `fn main() { ... }` with instructions
inside, each ended by `;`.

## 3. Tiny examples to read

Run the program above and it prints exactly this:

```
Hello, world!
```

The text inside the quotes `"..."` is what gets printed — word for word. Change the
words and you change the output. Two instructions, two lines printed:

```rust
fn main() {
    println!("Hello, world!");
    println!("I am learning Rust.");
}
```

prints:

```
Hello, world!
I am learning Rust.
```

Each `println!` prints its own line, in order, top to bottom — because that's the
order the steps sit inside `main`.

## 4. Common pitfalls / real compiler errors

Here's the one to feel in your hands. We forget the `!` on `println`:

```rust
fn main() {
    println("Hello, world!");
}
```

**Before you scroll — will this compile?**

It won't. Here is the *real*, unedited output from `rustc` (1.95.0):

```
error[E0423]: expected function, found macro `println`
 --> c.rs:2:5
  |
2 |     println("Hello, world!");
  |     ^^^^^^^ not a function
  |
help: use `!` to invoke the macro
  |
2 |     println!("Hello, world!");
  |            +
```

Read it slowly — the compiler is *teaching* here:

- `error[E0423]` — every Rust error has a code; you can look any of them up.
- It says `println` is a **macro**, and you tried to use it like a plain function.
- And it hands you the fix under **help**: add the `!` — `println!`.

> A compiler error is not a scolding — it's the program telling you it isn't yet
> doing what you asked. Read it top to bottom; the fix is usually right there.

So: `println!` needs its `!` because it's a macro. That single character is the
whole lesson of this error.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (in the ⋯ menu) — it compiles and runs
right here, offline. *(On your own machine, a playground or `cargo new hello`
works too.)* Then, *before running anything*, predict on paper:

1. Type `main` yourself with one `println!` that prints your own name (not
   "Hello, world!" — make it yours). **Predict the exact line it will print**, then
   run it and check.
2. Now add a **second** `println!` below the first with a different message.
   **Predict the two lines, in order**, then run to confirm.
3. Delete the `!` from one of them. **Predict: will it compile? If not, what error
   code, and what one-character fix will the compiler suggest?** Then run it and
   read the real output.

*(You write every line here — I won't. The predictions above are your answer key;
the code is yours.)*

## 6. What surprised you?

In one or two sentences: did anything about `main`, the `!` on `println!`, or the
`;` catch you off guard? Tell me, and I'll match the next lesson to where you
actually are.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §1.2 "Hello, World!". The program, the
  `fn main` starting-point framing, and the anatomy of a Rust program.
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 1 — Bindings & Immutability →](01-bindings-and-immutability.md)
