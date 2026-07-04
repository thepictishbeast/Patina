// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Predict first: what stops this compiling? Then run it and read the error — note
// which path it says is unresolved. Fix it WITHOUT changing the `mod shapes` block,
// the struct, or `main`.

mod shapes {
    pub struct Circle {
        pub radius: f64,
    }
}

use crate::shape::Circle;

fn main() {
    let c = Circle { radius: 2.0 };
    println!("a circle of radius {}", c.radius);
}
