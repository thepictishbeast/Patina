// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing the struct definition or the println!.
// Read the compiler error — it names the field you left out.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3 };

    println!("point is ({}, {})", p.x, p.y);
}
