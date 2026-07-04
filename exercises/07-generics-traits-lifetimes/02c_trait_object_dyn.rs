// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Predict first: will this compile? Then run it and read the error. Change `shapes`
// so it can hold BOTH a `Circle` and a `Square`. Leave the trait, the two structs,
// and the area math exactly as they are.

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let shapes = vec![Circle { radius: 1.0 }, Square { side: 2.0 }];
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("total area = {total}");
}
