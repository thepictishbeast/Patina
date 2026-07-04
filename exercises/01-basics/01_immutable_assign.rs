// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: in Rust, `let` bindings are IMMUTABLE by default — you can't
// reassign them. If a value needs to change, opt in with `let mut`.

// Make this compile WITHOUT removing the reassignment. The compiler tells
// you exactly which binding needs to be mutable.

fn main() {
    let count = 0;
    count = count + 1; // reassigning an immutable binding
    println!("count is {count}");
}
