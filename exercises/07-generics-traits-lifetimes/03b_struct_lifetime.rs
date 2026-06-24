// Rustlings Pro — exercises/07-generics-traits-lifetimes/03b_struct_lifetime.rs
//
// CONCEPT: when a struct holds a REFERENCE instead of an owned value, Rust needs a
// promise that the reference won't outlive the data it points to. You make that
// promise with a lifetime parameter on the struct: it ties "how long an `Excerpt`
// lives" to "how long the string its `part` borrows from lives", and the compiler
// checks the two stay consistent.
//
// `Excerpt` stores a `&str` but declares no lifetime, so the compiler can't reason
// about how long that borrow is valid (E0106: missing lifetime specifier).
//
// Run it and read E0106 — the `help:` line shows the exact shape. Give the struct a
// lifetime parameter and tie the reference field to it. The body in `main` is
// already fine; only the struct definition needs the annotation.
//
// Hint ladder: press Hint (or `rpro exercise hint`).

struct Excerpt {
    part: &str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let e = Excerpt { part: first_sentence };
    println!("excerpt: {}", e.part);
}
