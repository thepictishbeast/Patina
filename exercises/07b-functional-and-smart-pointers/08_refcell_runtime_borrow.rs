// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Predict first: this COMPILES — but does it survive being run? Run it and read
// what happens. Then make it run cleanly and print the list, keeping both pushes.

use std::cell::RefCell;

fn main() {
    let tasks = RefCell::new(vec!["write", "test"]);

    let mut a = tasks.borrow_mut();
    let mut b = tasks.borrow_mut();
    a.push("ship");
    b.push("celebrate");

    println!("{:?}", tasks.borrow());
}
