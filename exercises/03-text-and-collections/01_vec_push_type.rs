// Rustlings Pro — exercises/03-text-and-collections/01_vec_push_type.rs

// CONCEPT: a `Vec<T>` is homogeneous — every element is the SAME type.
// You declared a `Vec<i32>`, so it can only hold `i32`s.

// Make this compile WITHOUT changing `Vec<i32>`. Read the
// `expected i32, found &str` on the error and fix the bad push.

// Hint ladder: press Hint (or `rpro exercise hint`).

fn main() {
    let mut scores: Vec<i32> = Vec::new();
    scores.push(95);
    scores.push("perfect"); // not an i32
    println!("{scores:?}");
}
