// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `first_char` takes its text by value as `str`, which won't compile. Fix the
// parameter type so it compiles, without changing what `main` passes or what it
// prints. Run it and read the FIRST compiler error.

fn first_char(text: str) -> char {
    text.chars().next().unwrap_or('?')
}

fn main() {
    let c = first_char("ferris");
    println!("first char is {c}");
}
