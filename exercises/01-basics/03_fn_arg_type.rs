// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing `greet`'s signature. Read the error.

fn greet(name: &str) {
    println!("hi {name}");
}

fn main() {
    let age = 30;
    greet(age);
}
