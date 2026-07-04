// Rustlings Pro — exercises/09-advanced/03_unsized_str.rs

// CONCEPT: most types have a fixed SIZE the compiler knows up front, so it can put
// them on the stack and pass them around by value. But `str` (the string data
// itself, not `&str`) is a DYNAMICALLY SIZED type — its length isn't known until
// runtime, so the compiler can't lay it out by value. That's why you always see
// `&str`, never a bare `str`: a reference has a known size (a pointer plus a length)
// even when the data behind it doesn't.

// `first_char` tries to take a `str` BY VALUE, which the compiler rejects (E0277:
// the size of `str` can't be known at compile time). Read the FIRST error — that's
// the real lesson; the second (E0308) is just the call not matching the broken
// signature, and fixing the parameter type fixes both.

// Give the parameter a known size by taking the text behind a reference instead of
// by value. Then the call works.

// Hint ladder: press Hint (or `rpro exercise hint`).

fn first_char(text: str) -> char {
    text.chars().next().unwrap_or('?')
}

fn main() {
    let c = first_char("ferris");
    println!("first char is {c}");
}
