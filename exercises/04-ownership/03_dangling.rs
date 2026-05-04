// Patina — exercises/04-ownership/03_dangling.rs
//
// CONCEPT: a function can't return a reference to data it owns —
// when the function returns, its locals are dropped, and any
// reference into them would be left pointing at freed memory.
// Other languages (C, C++) let you do this and explode at runtime;
// Rust's borrow checker rejects it at compile time.
//
// Make this compile WITHOUT changing main(). The fix is in
// `make_greeting`. The failing version produces E0515 ("cannot
// return reference to local variable"). Run `rpro exercise hint`
// for the book sections that explain why.
//
// Hint of last resort: `rpro exercise hint --solution`.

fn main() {
    let s = make_greeting();
    println!("{s}");
}

// FIXME: this returns a reference to a local — the local is dropped
// when the function returns, so the reference would dangle. Rust
// rejects this at compile time. Make the function return ownership
// of the data instead of a borrow.
fn make_greeting() -> &'static str {
    let g = String::from("hello, world");
    &g
}
