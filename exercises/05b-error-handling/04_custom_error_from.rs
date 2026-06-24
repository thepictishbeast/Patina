// Rustlings Pro — exercises/05b-error-handling/04_custom_error_from.rs
//
// CONCEPT: real programs define their OWN error type so every way a function can
// fail comes back as one consistent kind. The `?` operator works with this: when you
// apply `?` to a `Result` whose error is some OTHER type, `?` automatically CONVERTS
// that error into your function's error type — but only if you've taught Rust how, by
// implementing the `From` trait that builds your error from theirs.
//
// `parse_count` returns `Result<i32, AppError>`, but `s.parse()` fails with a
// `ParseIntError`. There's no conversion from `ParseIntError` into `AppError`, so `?`
// can't do its job (E0277).
//
// Run it and read E0277 — its `help:` line names the trait you need to implement to
// teach Rust the conversion. Add that conversion so `?` can turn a parse failure into
// your `AppError`. Don't change the function signatures.
//
// Hint ladder: press Hint (or `rpro exercise hint`).

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
