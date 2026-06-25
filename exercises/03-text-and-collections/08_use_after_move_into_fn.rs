// Passing a value to a function MOVES it (for types that don't copy, like `Vec`).
// Here `scores` is moved into `fill_vec`, and then the program tries to print
// `scores` again afterward — but it no longer owns the vector, so it does not
// compile.
//
// Run it, read the error (note its code and which value it says was moved), then
// make it compile so BOTH lines print.
//
// Hint: the call hands the whole vector away. To still have it afterward you can
// give the function a *copy* to fill, or rethink what each binding owns. (Reach
// for the obvious method that duplicates a value only once you've read the error.)
//
// Adapted from Rustlings (rust-lang/rustlings, MIT) — 06_move_semantics — rebuilt
// into Tempered Studio's predict-then-run format.
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
