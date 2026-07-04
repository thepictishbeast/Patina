// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: methods live in an `impl` block. You can only call methods
// that are actually defined — calling one that doesn't exist is a
// compile error (the compiler even suggests close names).

// Make this compile WITHOUT removing the `c.value()` call. The struct
// is missing the method `main` tries to call — define it in the `impl`
// block (it should bump `count` by one; take `&mut self`).

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
    c.increment(); // no such method yet
    c.increment();
    println!("counted {}", c.value());
}
