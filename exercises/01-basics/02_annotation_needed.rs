// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: Rust infers most types, but sometimes it genuinely can't.
// `.parse()` can produce many types (i32, f64, ...), so you must say
// which one you want — with a type on the binding or a turbofish.

// Make this compile by telling the compiler the type, e.g.
// `let n: i32 = ...` or `"42".parse::<i32>()`. Read the
// "type annotations needed" error.

fn main() {
    // parse() is generic — the compiler can't tell what to parse into.
    let n = "42".parse().unwrap();
    println!("doubled: {}", n * 2);
}
