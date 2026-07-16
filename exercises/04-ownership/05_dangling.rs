// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing main() — the fix is in `make_greeting`.
// Read the compiler error.

fn main() {
    let s = make_greeting();
    println!("{s}");
}

fn make_greeting() -> &'static str {
    let g = String::from("hello, world");
    &g
}
