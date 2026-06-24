// Rustlings Pro — exercises/05b-error-handling/01_result_is_not_t.rs
//
// CONCEPT: some operations can FAIL, so instead of handing back a value directly
// they hand back a `Result`: `Ok(value)` when they succeed, or `Err(reason)` when
// they don't. A `Result<i32, _>` is NOT an `i32` — it's a wrapper holding EITHER an
// `i32` or an error, and you have to open it up and deal with both cases before you
// can use the number inside.
//
// `to_number` returns a `Result` because the text might not be a number. Binding it
// straight to an `i32` makes the types disagree (E0308).
//
// Run it, read the error (the `-->` line and the `help:` line), then get the number
// out and handle the case where parsing fails — decide what to use when the text
// isn't a valid number, instead of assuming it always is.
//
// Hint ladder: press Hint (or `rpro exercise hint`).

use std::num::ParseIntError;

fn to_number(text: &str) -> Result<i32, ParseIntError> {
    text.parse::<i32>()
}

fn main() {
    let n: i32 = to_number("42");
    println!("the number plus one is {}", n + 1);
}
