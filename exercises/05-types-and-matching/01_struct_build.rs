// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: when you build a struct, you must give EVERY field a value.
// Rust has no "uninitialized" fields — a half-built struct can't exist.

// Make this compile WITHOUT changing the struct definition or the
// println!. The compiler names the field you left out.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // One field is missing here.
    let p = Point { x: 3 };

    println!("point is ({}, {})", p.x, p.y);
}
