// Adapted from Rustlings (MIT/Apache-2.0) — rust-lang/rustlings, move_semantics1
//
// Make this compile so it returns the vector with 88 added. Read the error and the
// compiler's suggestion.

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let vec = vec;
    vec.push(88);
    vec
}

fn main() {
    let scores = fill_vec(vec![22, 44, 66]);
    println!("scores: {scores:?}");
}
