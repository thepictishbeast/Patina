// Variables are immutable by default. `let scores = ...` makes a binding you
// can read but not change — and calling a method that MODIFIES the value (like
// `.push`, which takes `&mut self`) counts as changing it. This program tries
// to add a score to a list that was not declared changeable, so it does not
// compile.
//
// Run it, read the error (note its code) and the compiler's suggestion, then
// make it compile so it prints all three scores.
//
// Hint: this is the *other* side of immutability from reassigning a value —
// here you are not rebinding `scores`, you are mutating it in place. One
// keyword on the `let` line grants permission to do that.
fn main() {
    let scores = vec![10, 20];
    scores.push(30);
    println!("scores: {scores:?}");
}
