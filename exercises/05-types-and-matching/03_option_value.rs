// Rustlings Pro — exercises/05-types-and-matching/03_option_value.rs

// CONCEPT: `Option<i32>` is NOT an `i32`. It's a box that's either
// `Some(value)` or `None`. You can't use it as the number directly —
// you must get the value OUT first (match, `if let`, `unwrap`, etc.).
// This is Rust's answer to null: the maybe-ness is in the type.

// Make this compile WITHOUT changing `let maybe: Option<i32>`. Pull the
// value out of the Option before binding it to `value`. Read the
// `expected i32, found Option<i32>` on the error.

// Hint ladder: press Hint (or `rpro exercise hint`).

fn main() {
    let maybe: Option<i32> = Some(7);

    // An Option<i32> can't be assigned where an i32 is expected.
    let value: i32 = maybe;

    println!("the value is {value}");
}
