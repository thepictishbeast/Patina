// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT making `area_cache` pub and WITHOUT changing the
// module. Read which field the compiler says is private.

mod shapes {
    pub struct Circle {
        pub radius: f64,
        area_cache: f64,
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
    println!("cache is {}", c.area_cache);
}
