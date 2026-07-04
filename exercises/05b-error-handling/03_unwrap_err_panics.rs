// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Predict what happens when this runs, then run it and read the panic. HANDLE the
// error instead of unwrapping it, so the program finishes instead of crashing.

fn main() {
    let raw = "ferris";
    let count: i32 = raw.parse().unwrap();
    println!("count doubled is {}", count * 2);
}
