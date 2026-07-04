// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile so it prints the final count. Read the compiler error.

fn main() {
    let mut count = 0;
    let first = &mut count;
    let second = &mut count;
    *first += 1;
    *second += 1;
    println!("count = {count}");
}
