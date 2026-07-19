// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing `Vec<i32>`. Read the "expected i32, found
// &str" error and fix the bad push.

fn main() {
    let mut scores: Vec<i32> = Vec::new();
    scores.push(95);
    scores.push("perfect");
    println!("{scores:?}");
}
