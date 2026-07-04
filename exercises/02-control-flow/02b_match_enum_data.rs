// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Predict first: will this compile? Then run it and read the error. Make the
// `Rectangle` arm return its area. Don't change the enum or `main`.

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn area(s: Shape) -> f64 {
    match s {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle => 0.0,
    }
}

fn main() {
    let c = Shape::Circle(2.0);
    let r = Shape::Rectangle(3.0, 4.0);
    println!("circle area    = {:.2}", area(c));
    println!("rectangle area = {:.2}", area(r));
}
