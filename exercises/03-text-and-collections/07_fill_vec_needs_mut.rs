// `fill_vec` rebinds its parameter with `let vec = vec;` — an IMMUTABLE binding —
// and then calls `vec.push(88)`. But `.push` needs to mutate the vector, and an
// immutable binding won't allow that. Fix the function so it compiles and returns
// the vector with 88 added on the end.

// Run it, read the error (note its code AND the compiler's suggestion), then fix
// it so the program prints the full vector.

// Hint: the rebinding line is where the vector loses its "changeable" permission.
// One keyword there grants it back.

// Adapted from Rustlings (rust-lang/rustlings, MIT) — 06_move_semantics/
// move_semantics1 — rebuilt into Tempered Studio's predict-then-run format.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let vec = vec;
    vec.push(88);
    vec
}

fn main() {
    let scores = fill_vec(vec![22, 44, 66]);
    println!("scores: {scores:?}");
}
