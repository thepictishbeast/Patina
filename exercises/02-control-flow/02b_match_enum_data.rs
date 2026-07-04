// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: an enum variant can CARRY DATA. `Shape::Circle(f64)` holds a radius;
// `Shape::Rectangle(f64, f64)` holds a width and a height. When you `match` such a
// value, the pattern has to BIND that data so the arm can use it — look at the
// `Circle` arm below: `Shape::Circle(r)` names the radius `r`, then uses it.

// The `Rectangle` arm is wrong: it writes `Shape::Rectangle` as if the variant
// carried nothing, so this won't compile. Run it and read the error — the compiler
// even shows you the shape of the pattern you need. Bind the two numbers (give them
// names, the way `Circle` binds `r`) and return the rectangle's area. Don't change
// the enum or `main`.

// Predict first: will this compile? Then run it and read the error before you edit.

enum Shape {
    Circle(f64),         // a circle stores its radius
    Rectangle(f64, f64), // a rectangle stores its width and height
}

fn area(s: Shape) -> f64 {
    match s {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle => 0.0, // this ignores the width and height it carries
    }
}

fn main() {
    let c = Shape::Circle(2.0);
    let r = Shape::Rectangle(3.0, 4.0);
    println!("circle area    = {:.2}", area(c));
    println!("rectangle area = {:.2}", area(r));
}
