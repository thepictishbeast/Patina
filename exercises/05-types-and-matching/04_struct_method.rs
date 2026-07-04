// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT removing the `c.value()` call. `main` calls a method
// that isn't defined yet. Read the compiler error.

struct Counter {
    count: u32,
}

impl Counter {
    fn value(&self) -> u32 {
        self.count
    }
}

fn main() {
    let mut c = Counter { count: 0 };
    c.increment();
    c.increment();
    println!("counted {}", c.value());
}
