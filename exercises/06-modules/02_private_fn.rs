// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing `main`. The compiler names the private
// function.

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    fn double(x: i32) -> i32 {
        x * 2
    }
}

fn main() {
    println!("{}", math::add(2, 3));
    println!("{}", math::double(5));
}
