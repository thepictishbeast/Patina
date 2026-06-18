// Rustlings Pro — exercises/02-control-flow/04_for_scope.rs
//
// CONCEPT: a `for` loop's variable only exists INSIDE the loop body.
// Once the loop ends, that name is gone — using it afterward is an
// error. Control flow and scope go together: a binding lives only in
// the block that introduced it.
//
// Make this compile WITHOUT removing the final println!. Think about
// where to keep a running value that OUTLIVES the loop (declare it
// before the loop, update it inside).
//
// Hint ladder: press Hint (or `rpro exercise hint`).

fn main() {
    for i in 1..=5 {
        println!("tick {i}");
    }

    // `i` belonged to the loop — it doesn't exist out here.
    println!("the loop ran {i} times");
}
