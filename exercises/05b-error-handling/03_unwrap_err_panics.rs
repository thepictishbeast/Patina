// Rustlings Pro — exercises/05b-error-handling/03_unwrap_err_panics.rs
//
// CONCEPT: `unwrap()` is the lazy way to get the value out of a `Result`: if it's
// `Ok`, you get the value; if it's `Err`, the program PANICS and stops. It is a
// promise to the compiler that "this can't fail" — and when that promise is wrong,
// there is no graceful handling, just a crash.
//
// Here `raw` is not a number, so `raw.parse()` returns an `Err`, and `.unwrap()`
// turns that error into a panic. The code COMPILES — the failure only shows when it
// RUNS.
//
// Predict what happens when it runs, then run it and read the panic. Then HANDLE the
// error case instead of unwrapping it — decide what `count` should be when the text
// isn't a number — so the program finishes instead of crashing. (You handled errors
// this way in the earlier exercises in this phase.)
//
// Hint ladder: press Hint (or `rpro exercise hint`).

fn main() {
    let raw = "ferris";
    let count: i32 = raw.parse().unwrap();
    println!("count doubled is {}", count * 2);
}
