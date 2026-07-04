// Rustlings Pro — exercises/05-types-and-matching/02_field_privacy.rs

// CONCEPT: a struct can be `pub` while some of its fields stay private.
// Code outside the module can build/use the type through its public API
// but cannot read a private field directly. That's encapsulation.

// Make this compile WITHOUT making `area_cache` pub and WITHOUT changing
// the module. Use only what's public (the `radius` field and `new`).
// Read which field the compiler says is private.

// Hint ladder: press Hint (or `rpro exercise hint`).

mod shapes {
    pub struct Circle {
        pub radius: f64,
        area_cache: f64, // private to the `shapes` module
    }

    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius, area_cache: 0.0 }
        }
    }
}

fn main() {
    let c = shapes::Circle::new(2.0);
    println!("radius is {}", c.radius);
    // This reaches into a private field from outside the module.
    println!("cache is {}", c.area_cache);
}
