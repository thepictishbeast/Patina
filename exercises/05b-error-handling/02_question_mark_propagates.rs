// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: the `?` operator is a shortcut for dealing with a Result. Written after
// an expression that returns a Result, `?` says: "if this is Err, stop and return
// THAT error from the current function; otherwise, take the Ok value out and keep
// going." It saves you from writing a `match` every time you call something that
// can fail.

// But `?` only works inside a function that itself returns a Result (or Option) —
// it needs somewhere to SEND the error. `first_number` returns a plain `i32`, so a
// propagated error would have nowhere to go, and the compiler stops you (E0277).

// Run it, read the error and its `help:` line, then make `?` legal here: let
// `first_number` return a Result so a failure has somewhere to go, and adjust how it
// hands back the number and how main() uses the result.

fn first_number(text: &str) -> i32 {
    let n = text.parse::<i32>()?;
    n
}

fn main() {
    println!("the first number is {}", first_number("42"));
}
