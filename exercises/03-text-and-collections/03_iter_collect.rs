// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing `let doubled: Vec<i32>`. The error found a
// `Map<...>` where a `Vec<i32>` was expected.

fn main() {
    let nums = vec![1, 2, 3];
    let doubled: Vec<i32> = nums.iter().map(|n| n * 2);
    println!("{doubled:?}");
}
