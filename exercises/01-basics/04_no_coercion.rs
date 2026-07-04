// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: Rust does NOT implicitly convert between number types. An
// `i64` is not an `i32`, even though both are integers — you convert
// explicitly (e.g. `as i32`, or `i32::try_from`).

// Make this compile WITHOUT changing the `let small: i32` annotation.
// Convert `big` to an `i32` explicitly. Read the `expected i32, found
// i64` on the error.

fn main() {
    let big: i64 = 1_000_000;
    let small: i32 = big; // no implicit i64 -> i32 conversion
    println!("{small}");
}
