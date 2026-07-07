// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `main` adds two `Point`s with `+`, but `Point` doesn't support `+` yet, so this
// won't compile. Make `a + b` produce a `Point` whose fields are the two points'
// fields added together, without changing `main`. Run it and read the compiler error.

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 3, y: 4 };
    let sum = a + b;
    println!("{a:?} + {b:?} = {sum:?}");
}
