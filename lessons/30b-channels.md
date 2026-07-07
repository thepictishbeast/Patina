# Lesson 30b — Channels: passing data between threads

*(Phase 8, part 2. Two threads run side by side — how does one hand a result to
the other without them ever touching the same value at the same time? Through a
one-way pipe that MOVES ownership.)*

## 1. Why it exists

The safest way for threads to cooperate is to never share at all: one thread
*sends* a value, the other *receives* it, and ownership travels with the value.
A **channel** is exactly that pipe — and because `send` **moves** the value, the
type system (not your discipline) guarantees the sender can't touch it
afterwards. No sharing, no race.

## 2. The idea

`mpsc::channel()` gives you a pair: a **transmitter** `tx` and a **receiver**
`rx` (mpsc = *multiple producer, single consumer*).

- One thread calls `tx.send(value)` — the value **moves** into the channel.
- Another calls `rx.recv()` to wait for and take it — or simply loops
  `for received in rx { … }`, which ends on its own when every sender has been
  dropped.
- Clone `tx` to get **several** senders feeding one receiver (that's the
  "multiple producer"; Book §16.2 shows it).
- With a single sender, order is preserved.

## 3. A tiny example to read

The spawned thread `send`s four strings; `main` receives them by looping over
`rx`. The loop ends when the sending thread finishes (dropping `tx`):

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for m in messages {
            tx.send(m).unwrap();
        }
    });

    for received in rx {
        println!("got: {received}");
    }
}
```

```
got: hi
got: from
got: the
got: thread
```

## 4. Common pitfalls / real compiler errors — you can't keep what you sent

`send` **moves** the value into the channel. Touch it afterwards and the
Lesson-15 rule fires:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("hello");
        tx.send(msg).unwrap();
        println!("sent: {msg}");
    });
    println!("got: {}", rx.recv().unwrap());
}
```

**Before you scroll — does this compile?**

```
error[E0382]: borrow of moved value: `msg`
 --> main.rs:9:26
  |
7 |         let msg = String::from("hello");
  |             --- move occurs because `msg` has type `String`, which does not implement the `Copy` trait
8 |         tx.send(msg).unwrap();
  |                 --- value moved here
9 |         println!("sent: {msg}");
  |                          ^^^ value borrowed here after move
```

The whole safety story in one error: once sent, the value belongs to the other
side. If the sender needs a copy, clone *before* sending — but usually the move
is exactly what you want. The matching exercise below is this wall.

## 5. Predict-then-run practice (your turn — write this yourself)

Type these in the app's **🧪 Sandbox** (⋯ menu), then take on the matching
exercise via the **Practice this lesson** link at the bottom. *(On your own
machine, a playground or `cargo new channels` works too.)* **Predict on paper
before each run.**

1. **A channel.** Spawn a thread that `send`s three numbers down `tx`; receive
   by looping over `rx` in `main`, printing each. **Predict** the output *and*
   its order (single sender — is order preserved?).
2. **Use a value after sending it.** Send a `String`, then try to print it in
   the sender. **Predict the error code**, and say in one sentence why the move
   *is* the safety.

*(You write every line here — I won't. The predictions are your answer key.
Next: when threads genuinely must share ONE value — Arc and Mutex.)*

## 6. What surprised you?

A sentence or two: did "ownership travels with the value" make channels feel
safer than sharing, or just different? Tell me, and I'll pitch Lesson 30c to
match.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **§16.2**: `mpsc::channel`,
  `tx.send`/`rx.recv`, receiving by iterating `rx`, and multiple producers via
  `tx.clone()` (Listings 16-8/16-11).
- **CR** — *Comprehensive Rust* (Google): the Channels slides.
- Every snippet compiled and run, and every error captured live, on
  **rustc 1.95.0**, edition 2024.

---

<!-- lesson-nav -->
[← Lesson 30 — Spawning threads](30-spawning-threads.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 30c — Shared state: Arc & Mutex →](30c-shared-state.md)
