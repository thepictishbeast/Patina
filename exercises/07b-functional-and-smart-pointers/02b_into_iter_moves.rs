// `into_iter()` takes ownership of the vector and hands out *owned* items.
// This program doubles the numbers that way, then tries to print the original
// vector. It does not compile. Run it, read the error, and fix it so BOTH
// lines print.

// Hint: one of the three iterator-makers borrows the collection instead of
// consuming it — which one lets `v` survive the chain?
fn main() {
    let v = vec![1, 2, 3];
    let doubled: Vec<i32> = v.into_iter().map(|x| x * 2).collect();
    println!("doubled:  {doubled:?}");
    println!("original: {v:?}");
}
