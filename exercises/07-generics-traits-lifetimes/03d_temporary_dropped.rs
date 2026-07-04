// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: a borrow must not outlive the value it points to — and a *temporary*
// (a value with no name, like the `String` that `String::from(...)` builds on
// the spot) lives only to the end of the statement that creates it. Here a
// reference INTO that temporary is kept past the statement, so by the time it's
// used the value it borrowed is already gone.

// PREDICT first: what stops this compiling? Then run it and read the error —
// note its code and the "temporary value" it says is dropped while borrowed.

// Fix it WITHOUT changing `first_word`: give the `String` a NAME so it lives long
// enough — bind it to a variable, then pass a reference to that. (The book's
// "lifetimes / the borrow checker" section is the why.)

fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}

fn main() {
    let word = first_word(&String::from("hello world"));
    println!("the first word is: {word}");
}
