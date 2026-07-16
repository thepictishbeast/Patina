// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `?` is used in `first_number`, but it isn't allowed there yet. Make this compile
// so `?` is legal and main prints the number. Read the compiler error and its `help:`.

fn first_number(text: &str) -> i32 {
    let n = text.parse::<i32>()?;
    n
}

fn main() {
    println!("the first number is {}", first_number("42"));
}
