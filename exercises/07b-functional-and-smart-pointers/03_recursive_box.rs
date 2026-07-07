// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `List` contains itself, so it has no known size and won't compile. Give it a
// known size without changing which values it holds. Run it and read the compiler
// error top-to-bottom, including its `help:` line.

enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    use List::{Cons, Nil};
    let list = Cons(1, Cons(2, Nil));
    let _ = list;
}
