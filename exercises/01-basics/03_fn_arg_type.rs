// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: function parameters have fixed types. Calling a function with
// the wrong type is a compile error — Rust won't silently convert for you.

// Make this compile WITHOUT changing `greet`'s signature. Pass it the
// type it asks for (a `&str`). Read the `expected &str, found {integer}`.

fn greet(name: &str) {
    println!("hi {name}");
}

fn main() {
    let age = 30;
    greet(age); // greet wants a &str, not an integer
}
