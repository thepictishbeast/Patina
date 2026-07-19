// A slice like `&scores[0..2]` doesn't copy those numbers — it BORROWS the whole
// `scores` vector for as long as the slice is alive. Rust's rule: while something
// is borrowed as shared, you can't also mutate it. Predict — does pushing onto
// `scores` while `window` still points into it compile? Read the E0502 error, then
// reorder so the slice is finished being used *before* the vector changes.
fn main() {
    let mut scores = vec![10, 20, 30];
    let window = &scores[0..2];
    scores.push(40);
    println!("the first two are {window:?}");
}
