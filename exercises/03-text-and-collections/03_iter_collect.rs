// Rustlings Pro — exercises/03-text-and-collections/03_iter_collect.rs
//
// CONCEPT: iterators are LAZY. `.map(...)` doesn't produce a Vec — it
// returns an iterator that does nothing until you consume it. To get a
// `Vec` back, you `.collect()` it.
//
// Make this compile WITHOUT changing `let doubled: Vec<i32>`. The error
// says it found a `Map<...>` where a `Vec<i32>` was expected — finish
// the chain so it actually produces the Vec.
//
// Hint ladder: press Hint (or `rpro exercise hint`).

fn main() {
    let nums = vec![1, 2, 3];
    // map() returns an iterator, not a Vec — it's never collected.
    let doubled: Vec<i32> = nums.iter().map(|n| n * 2);
    println!("{doubled:?}");
}
