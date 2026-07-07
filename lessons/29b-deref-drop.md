# Lesson 29b — `Deref` & `Drop`: the traits underneath

*(Phase 7, smart pointers part 2. A `Box` acts like a reference and cleans up
after itself. Neither is magic — each is a trait, and both power every smart
pointer you'll meet.)*

## 1. Why it exists

Two questions Lesson 29 left open: why does `*` work on a box exactly like on a
reference? And who frees the heap data when the box goes away? The answers are
two traits — **`Deref`** (behave like a reference) and **`Drop`** (run cleanup at
scope-end) — and knowing them now pays off immediately: `Drop` is how the next
lesson's `Rc` knows to decrease its owner count.

## 2. The idea

- **`Deref`** is why `*` follows a box just like it follows a reference. (That's
  the one line you need; how `Deref` powers method calls and the `&String`→`&str`
  bridging you met in Lesson 17 is the Book, §15.2.)
- **`Drop`** is the cleanup hook: give a type a `drop` method and Rust calls it
  **automatically** when the value goes out of scope — you never call it
  yourself. Values drop in **reverse creation order** (last made, first
  dropped).

## 3. Tiny examples to read

**`*` follows a box like it follows a reference:**

```rust
fn main() {
    let x = 5;
    let y = &x;          // a reference to x
    let z = Box::new(x); // a box holding a copy of x

    println!("{}", *y);  // follow the reference
    println!("{}", *z);  // follow the box — same `*`
}
```

```
5
5
```

**`Drop` — cleanup runs automatically at scope-end.** Note the **reverse**
order:

```rust
struct Guard {
    name: String,
}

impl Drop for Guard {
    fn drop(&mut self) {
        println!("dropping {}", self.name);
    }
}

fn main() {
    let _a = Guard { name: String::from("a") };
    let _b = Guard { name: String::from("b") };
    println!("both created");
}
```

```
both created
dropping b
dropping a
```

You never call `drop` yourself — Rust inserts the call. `_b` was created last,
so it's dropped first. (This same hook is what makes the next lesson's `Rc`
decrease its count.)

## 4. Common pitfalls — trusting the cleanup order

The trap here isn't an error code — it's *assuming* drop order doesn't matter.
When two values depend on each other (a lock and the data it guards, a file and
a writer into it), reverse-creation order is exactly what makes the dependent
one let go first. Predict the order in your own code before you rely on it —
part 5 has you do precisely that.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu). *(On your own machine, a
playground or `cargo new droporder` works too.)* **Predict on paper before each
run.**

1. **Box vs deref arithmetic.** With `let b = Box::new(10);`, try `*b + 1` and
   then `b + 1`. **Predict** which one the compiler rejects, and read what it
   says the box's type is.
2. **Drop order.** Make a struct with a `Drop` impl that prints a name; create
   three with different names, print `"made all three"` at the end. **Predict
   the full output, in order**, before you run — including which name drops
   first.
3. **A scope changes the order.** Put the *second* Guard inside an inner `{ }`
   block. **Predict** the new output order.

*(You write every line here — I won't. The predictions are your answer key.
Next: `Rc<T>` — one value with many owners, counted by exactly this `Drop`
machinery.)*

## 6. What surprised you?

A sentence or two: did "reverse creation order" match your intuition? Does
knowing `Drop` exists change how you think about the cleanup you never had to
write in 29 lessons? Tell me, and I'll pitch Lesson 29c to match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§15.2** (`Deref` — treating
  smart pointers like references; the depth beyond "`*` works on a box" lives
  there) and **§15.3** (`Drop` — running code on cleanup, reverse drop order).
- Every snippet compiled and run on **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 29 — Box: values on the heap](29-box.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 29c — Rc: one value, many owners →](29c-rc.md)
