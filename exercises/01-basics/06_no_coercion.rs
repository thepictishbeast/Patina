// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing the `let small: i32` annotation.
// Read the "expected i32, found i64" error.

fn main() {
    let big: i64 = 1_000_000;
    let small: i32 = big;
    println!("{small}");
}
