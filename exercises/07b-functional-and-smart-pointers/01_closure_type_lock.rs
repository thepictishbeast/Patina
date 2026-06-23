// Rustlings Pro — exercises/07b-functional-and-smart-pointers/01_closure_type_lock.rs
//
// CONCEPT: a closure's parameter and return types are INFERRED from how it is
// FIRST used — and then locked in. A closure is not generic: the compiler picks
// one type from the first call and expects every later call to match.
//
// `echo` is called first with an integer, then with a string. Predict what
// happens, run it, and read which call the compiler points its `-->` at — then
// make the two calls consistent, without changing what `echo` itself does.
//
// Hint ladder: press Hint (or `rpro exercise hint`).

fn main() {
    let echo = |x| x;
    let n = echo(5);
    let s = echo("hello");
    println!("{n} and {s}");
}
