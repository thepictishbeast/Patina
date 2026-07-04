// Rustlings Pro — exercises/07-generics-traits-lifetimes/02c_trait_object_dyn.rs

// CONCEPT: a `Vec<T>` holds many values of ONE type `T`. Below, `Circle` and
// `Square` are two DIFFERENT types — even though both implement the `Shape`
// trait. So `vec![circle, square]` does not type-check: once Rust sees the first
// element is a `Circle`, it fixes the element type to `Circle` and expects every
// other element to be one too. A `Square` is a different type, so it is rejected
// (E0308: mismatched types — expected `Circle`, found `Square`).

// But "a list of shapes" is exactly what you want here. Rust's answer is a TRAIT
// OBJECT: a value stored behind a shared trait instead of behind its concrete
// type, so one collection can hold a MIX of types that all implement that trait.
// The Book chapter linked from this exercise shows the syntax and why it works.
// Read it, then change `shapes` so it can hold both a `Circle` and a `Square`.
// Leave the trait, the two structs, and the area math exactly as they are.

// Predict first: will this compile? Then run it and read the error before you
// edit — the type the compiler "expected" is the clue.

// Hint ladder: press Hint (or `rpro exercise hint`).

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
