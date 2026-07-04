// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `to_number` returns a Result because the text might not be a number. Make this
// compile, handling the case where parsing fails. Read the compiler error.

use std::num::ParseIntError;

fn to_number(text: &str) -> Result<i32, ParseIntError> {
    text.parse::<i32>()
}

fn main() {
    let n: i32 = to_number("42");
    println!("the number plus one is {}", n + 1);
}
