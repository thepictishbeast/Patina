// Rustlings Pro — exercises/07-generics-traits-lifetimes/02b_trait_in_scope.rs

// CONCEPT: a type can IMPLEMENT a trait in one place, but the trait's methods are
// only callable where the trait itself is IN SCOPE — brought in with a `use`. The
// `Circle` below really does implement `Area`, yet `main` can't call `c.area()`,
// because `main` has not imported the `Area` trait. The compiler reports the method
// as "not found" even though the impl exists (E0599).

// Run it and read E0599 top-to-bottom — the `help:` line names exactly the `use`
// line you need to add. Bring the trait into scope so its method becomes callable;
// don't change the math.

// Hint ladder: press Hint (or `rpro exercise hint`).

mod geometry {
    pub trait Area {
        fn area(&self) -> f64;
    }

    pub struct Circle {
        pub radius: f64,
    }

    impl Area for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
}

fn main() {
    let c = geometry::Circle { radius: 2.0 };
    println!("area = {}", c.area());
}
