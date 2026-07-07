# Lesson 35 — Capstone I: a single-threaded web server

*(Phase 9 — the capstone begins. Every concept so far has been a tool; the
capstone proves the tools combine into something real. Over two lessons you
build an HTTP server: this one gets a REAL server answering your browser — and
ends by making you feel, in two browser tabs, exactly why it isn't good
enough yet. Unlike every other lesson, this one BINDS a network port — so you
compile-check it here and **run it yourself** with `cargo run`.)*

## 1. Why it exists

A web server is the perfect capstone: it must accept many connections, talk a
real protocol, and stay correct under load. The Book saves it for last for the
same reason we do — if you can read and build this, you can read most real
Rust.

It also teaches one new API surface, and honestly: **`TcpListener`** and
**`TcpStream`** from `std::net`. Everything else in this lesson — loops,
functions, `format!`, byte buffers, `unwrap` — you already own.

> **How the sources frame it:** the **BOOK** Ch.21 "Final Project" is the only
> full treatment, and it builds the server **compiler- and experience-driven**:
> start single-threaded, *feel* one slow request block the rest, then fix it
> properly. This lesson is the first half of that arc; Lesson 35b is the fix.

## 2. The idea

- **Listen.** `TcpListener::bind("127.0.0.1:7878")` claims the port. Looping
  over `listener.incoming()` yields one **`TcpStream`** per connection — your
  two-way pipe to that browser.
- **Read the request.** A browser's request is just bytes; reading some into a
  buffer is enough for now (we're serving, not parsing).
- **Write a response.** HTTP's shape is plain text: a **status line**
  (`HTTP/1.1 200 OK`), any **headers** (`Content-Length` so the browser knows
  when you're done), a **blank line**, then the **body**. `format!` it,
  `write_all` it, done.
- **The flaw, named.** The loop handles each connection **inline** — start to
  finish, one at a time. Requests behind a slow one just wait. You'll *see*
  this in part 5, and it's the entire reason Lesson 35b exists.

## 3. The code to read

This **compiles** as shown; it isn't run here because it would bind a TCP
port — build and run it yourself with `cargo run`, then visit
`http://127.0.0.1:7878`. Nothing in this file fabricates server output.

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let _ = stream.read(&mut buffer);
    let body = "<h1>Hello from Rust</h1>";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);   // handled INLINE — one at a time
    }
}
```

Read the response string carefully: status line, one header, `\r\n\r\n` (the
blank line), body. That's a complete, legal HTTP response — a browser is
satisfied by exactly this. And read `main`'s loop again: nothing happens for
connection *N+1* until `handle_connection` finishes with connection *N*.

## 4. Common pitfalls / real compiler errors

**Forgetting `mut` on the stream — `E0596`.** Reading from and writing to a
`TcpStream` both *change* it (it has internal position and buffers), so the
parameter must be `mut`. Drop it and both uses fail:

```rust
fn handle_connection(stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    let _ = stream.read(&mut buffer);
    // ...
}
```

```
error[E0596]: cannot borrow `stream` as mutable, as it is not declared as mutable
  --> main.rs:4:22
   |
 4 | fn handle_connection(stream: TcpStream) {
   |                      ^^^^^^ not mutable
 5 |     let mut buffer = [0u8; 1024];
 6 |     let _ = stream.read(&mut buffer);
   |             ------ cannot borrow as mutable
...
13 |     stream.write_all(response.as_bytes()).unwrap();
   |     ------ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
 4 | fn handle_connection(mut stream: TcpStream) {
   |                      +++
```

Lesson 2's rule, all the way out here: bindings are immutable unless you say
otherwise — and that includes function parameters. The help line hands you the
one-token fix. (Also worth knowing: if the port is already taken — say, a
previous run still alive — `bind(...).unwrap()` *panics* at startup with
"Address already in use." That's Lesson 20b's blunt path doing exactly what
you asked; kill the old process and rerun.)

## 5. Predict-then-run practice (your turn — write this yourself)

Build it for real: `cargo new hello-server`, type in the server (don't paste),
`cargo run`, and test with your browser. **Predict before each run.**

1. **Serve one page.** Get `http://127.0.0.1:7878` showing *Hello from Rust*.
   **Predict first**: with the loop as written, how many browser tabs can it
   serve *at the same instant*?
2. **Feel the flaw.** Add a route: if the request starts with `GET /sleep`,
   call `thread::sleep(Duration::from_secs(5))` before responding. (Peek at
   `buffer` with `String::from_utf8_lossy(&buffer)` to check the path.) Open
   `/sleep` in one tab, then immediately `/` in another. **Predict**: does `/`
   render instantly, or after ~5 seconds? Watch it happen.
3. **Say why, precisely.** In one sentence, using the word "loop": why did
   the second tab wait? (Your sentence is the design brief for Lesson 35b.)

*(You write every line here — I won't. The predictions are your answer key.
You now have a real server with a real problem you've personally felt — a
single slow request stalls everyone. Lesson 35b fixes it with a thread pool
built from your Phase 7–8 tools.)*

## 6. What surprised you?

A sentence or two: did it surprise you how *little* code a working web server
needs — and how readable the HTTP response format is? Did the `/sleep`
experiment make the single-thread flaw feel real in a way part 2's bullet
didn't? Tell me, and I'll fold it into the capstone review.

## 7. Sources

- **BOOK** — *The Rust Programming Language*, **Ch.21 "Final Project: Building
  a Multithreaded Web Server"**, the single-threaded first half: `TcpListener`
  / `TcpStream`, reading the request, the anatomy of an HTTP response, and the
  slow-request problem (its `/sleep` experiment is the one you run in part 5).
- The threaded second half of the same chapter is Lesson 35b.
- The full program compiles on **rustc 1.95.0**, edition 2024 (built, not
  run — it binds a TCP port); the `E0596` is captured verbatim from the same
  program with `mut` removed. Run the server yourself with `cargo run`.

---

<!-- lesson-nav -->
[← Lesson 34c — Declarative macros: macro_rules!](34c-macro-rules.md) · [↑ Study Guide](../STUDY-GUIDE.md) · [Lesson 35b — Capstone II: the thread pool & graceful shutdown →](35b-thread-pool.md)
