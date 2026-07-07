// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Both `b` and `c` want to hold the same tail `a`, but this won't compile. Make
// `a` shareable so both lists can hold it, without changing the values. Run it and
// read the compiler error top-to-bottom, including its `help:` line.

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
    let _ = (b, c);
}
