// Adapted from Rustlings (MIT/Apache-2.0) — rust-lang/rustlings, move_semantics
//
// Make this compile so BOTH lines print. Read the error — note which value it says
// was moved.

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(88);
    vec
}

fn main() {
    let scores = vec![22, 44, 66];
    let bigger = fill_vec(scores);
    println!("original: {scores:?}, filled: {bigger:?}");
}
