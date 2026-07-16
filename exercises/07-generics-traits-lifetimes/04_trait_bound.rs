// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `show` accepts any type that can be displayed; `Point` hasn't earned that yet.
// Make it compile — read the error.

use std::fmt::Display;

fn show<T: Display>(value: T) {
    println!("here it is: {value}");
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    show(p);
}
