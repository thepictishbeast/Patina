# Lesson 1 — Bindings & Immutability

## 1. Why it exists

When you give a value a name in Rust, that name **can't be pointed at a new value
later** — not unless you say so up front. This isn't fussiness. It's a safety
guarantee: if one part of your program counts on a name meaning `5`, and some
faraway line quietly changes it to `6`, you get bugs that are miserable to track
down. Rust closes that door by default, so a name you didn't mark as changeable
*really* won't change.

## 2. The idea

A **binding** is the act of attaching a name to a value:

```
let crew_size = 7;
```

Read that left to right: the keyword `let` says "I'm naming something," `crew_size`
is the name, `=` ties it to the value, `7`. From now on, writing `crew_size` means
`7`.

Two things are true the moment you write that line:

1. **Rust figures out the kind of value on its own.** You wrote `7`, a whole
   number, so Rust knows the name holds a whole number. You didn't have to tell
   it. (This is called *inference* — Rust infers the kind.)
2. **The name is locked to that value.** `crew_size` will be `7` for as long as it
   exists. Trying to re-point it later is not allowed — and Rust stops you at
   build time, before the program ever runs.

"Locked by default" is the whole lesson. A name in Rust is a promise that the
value stays put. If you *want* a name you can change, you have to ask for it
explicitly — and you'll see how at the end of part 4.

## 3. Tiny examples to read

**Inference first** — no kind written, Rust works it out:

```rust
fn main() {
    let crew_size = 7;
    println!("The crew size is: {crew_size}");
}
```

Prints:

```
The crew size is: 7
```

> **Read `{crew_size}` as "print the value of `crew_size` here."** Curly braces
> inside `println!`'s quotes drop a binding's value into the text. That's all you
> need for now — printing gets its full lesson in Lesson 8.

**Now a 30-second rep — you type this one.** The kind can also be spelled out by
hand. Type this version yourself; the only new part is `: i32` (Rust's default
whole number):

```rust
fn main() {
    let crew_size: i32 = 7;
    println!("The crew size is: {crew_size}");
}
```

**Predict before you run:** does adding `: i32` change what it prints, compared to
the inference version just above?

It doesn't — both compile to the exact same program and print the identical line:

```
The crew size is: 7
```

Inference isn't a looser mode — Rust still pins down one fixed kind, and `: i32`
just makes it visible. The full story of the kinds themselves is the Book's §3.2
"Data Types" (and Lesson 5). *(That was your first small write-rep — part 5 is
where you write the most.)*

## 4. Common pitfalls / real compiler errors

Here's the one to feel in your hands. We name `x`, then try to re-point it:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

**Before you scroll — will this even compile?**

It won't. Here is the *real* output from `rustc` (1.95.0), unedited:

```
error[E0384]: cannot assign twice to immutable variable `x`
 --> c.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0384`.
```

Read it slowly — the compiler is *teaching* here:

- `error[E0384]` — every Rust error has a code; this one means "assigned twice."
- It points at `let x = 5` as the **first assignment**, then at `x = 6` with
  `^^^^^ cannot assign twice`.
- And it hands you the fix under **help**: `let mut x = 5`.

> A compiler error is not a scolding — it's the program telling you it isn't yet
> safely doing what you asked.

**The fix Rust suggested** — that `help: consider making this binding mutable:
let mut x = 5` line — is the subject of the *next* lesson. For now, take away the
rule: names are locked by default, and the compiler names both the lock (`E0384`)
and the key (`mut`).

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu) and predict before each run.
When you're done, take on the matching exercise — the **Practice this lesson**
link at the bottom jumps straight into it. *(On your own machine, a playground or
`cargo new variables` works too.)* Then, *before running anything*, predict on
paper:

1. Write `main` with a binding of your own — pick any name and any whole-number
   value (not `x`, not `5` — make it yours). Print it once with `println!`.
2. Add a line **below** the print that points the same name at a *different*
   number. Print it again.

**Predict before you run:**
- Will it compile?
- If not — what **error code** will appear, and what exact one-word change will
  the compiler suggest under `help:`?

Then run it and check your prediction against the real output. *(Applying the
compiler's suggested fix is Lesson 2's opening move — save it.)*

*(You write every line here — I won't. The predictions above are your answer
key; the code is yours.)*

## 6. What surprised you?

In one or two sentences: was anything about "locked by default" unexpected? Did
the error message give you *more* than you expected, or less? Tell me, and I'll
adjust the next lesson to match where you actually are.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, §3.1 "Variables and Mutability."
  The failing-reassignment-then-real-`E0384` teaching move, the correctness WHY,
  and the embedded `variables` project repurposed as part 5.
- **CR** — *Comprehensive Rust* (Google), §5.2. Cited for contrast (immutability
  staged as a commented-out reassignment).
- **BLOG** — *Rust for Beginners* (Pablo Aguirre), "Immutable variables." Cited
  for contrast (immutability stated as a rule "for security").
- Compiler output captured live on **rustc 1.95.0** (edition 2024).

---

<!-- lesson-nav -->
[← Lesson 0 — Hello, World (your first program)](00-hello-world.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 2 — Mutability (`mut`) →](02-mutability.md)
