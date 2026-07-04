// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// This COMPILES. Predict what happens when it RUNS, then run it and read the
// panic. Make it run without panicking, even when there are no scores yet.

fn main() {
    let scores: Vec<i32> = Vec::new();
    let top = scores.iter().max().unwrap();
    println!("the top score is: {top}");
}
