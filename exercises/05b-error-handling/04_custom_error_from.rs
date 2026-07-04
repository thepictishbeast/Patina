// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `parse_count` uses `?` on a value whose error type differs from its own. Make this
// compile WITHOUT changing the function signatures. Read the error and its `help:`.

use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    BadInput,
}

fn parse_count(s: &str) -> Result<i32, AppError> {
    let n = s.parse::<i32>()?;
    Ok(n)
}

fn main() {
    match parse_count("42") {
        Ok(n) => println!("count is {n}"),
        Err(e) => println!("failed: {e:?}"),
    }
}
