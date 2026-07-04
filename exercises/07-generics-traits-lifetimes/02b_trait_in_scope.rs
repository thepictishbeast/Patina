// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `Circle` implements `Area`, but `main` can't call `c.area()`. Make it compile
// WITHOUT changing the math. Read the error top-to-bottom, including its `help:`.

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
