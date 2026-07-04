// This function PROMISES to return an owned `String`, but its body hands back a
// plain string literal — and a literal is a borrowed `&str`, not a `String`.
// The two are different types, so it does not compile.

// Run it, read the error (note its code and the two types it names), then fix it
// WITHOUT changing the function's signature, so it prints the favorite color.

// Hint: you need to turn the borrowed `&str` literal into an owned `String`.
// There's a method (and a `String::from`) that does exactly that — the book's
// "creating a new string" section names both.

// Adapted from Rustlings (rust-lang/rustlings, MIT) — 09_strings/strings1 —
// rebuilt into Tempered Studio's predict-then-run format.
fn current_favorite_color() -> String {
    "blue"
}

fn main() {
    println!("My current favorite color is {}", current_favorite_color());
}
