// Rustlings Pro — exercises/05-types-and-matching/03b_unwrap_none.rs
//
// CONCEPT: some failures don't show up at COMPILE time — they wait until the code
// RUNS. `.max()` on a list returns an `Option`, because the list might be empty and
// then there is no maximum. `.unwrap()` says "I'm certain there's a value inside" —
// and if there isn't, the program PANICS and stops.
//
// This file COMPILES cleanly. Predict what happens when it RUNS, then run it and
// read the panic: the line after `panicked at` tells you what went wrong. `scores`
// is empty, so `.max()` is `None`, and `.unwrap()` on `None` panics.
//
// Make it run without panicking, even when there are no scores yet — decide what the
// "top score" should be when the list is empty, instead of assuming there is one.
//
// Hint ladder: press Hint (or `rpro exercise hint`).

fn main() {
    let scores: Vec<i32> = Vec::new(); // no scores recorded yet
    let top = scores.iter().max().unwrap();
    println!("the top score is: {top}");
}
