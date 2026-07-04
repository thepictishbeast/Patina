// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: making a MODULE reachable does not make its contents public.
// Each function opts in to being callable from outside with its own
// `pub`. "Module public" != "contents public".

// Make this compile WITHOUT changing `main`. One function is private;
// the compiler names it. Add `pub` to it.

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    // no `pub` — callable only from inside `math`
    fn double(x: i32) -> i32 {
        x * 2
    }
}

fn main() {
    println!("{}", math::add(2, 3));
    println!("{}", math::double(5));
}
