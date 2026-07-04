// Rustlings Pro — exercises/07b-functional-and-smart-pointers/05_refcell_runtime_borrow.rs

// CONCEPT: a `RefCell` lets you change the value inside it even through a shared,
// immutable-looking handle — "interior mutability". To stay safe it still enforces
// the borrowing rules (many readers OR one writer, never both), but it checks them
// while the program RUNS, not at compile time.

// Here two `borrow_mut()` values, `a` and `b`, are alive AT THE SAME TIME — that's
// two writers at once. The compiler does NOT stop you, so this file COMPILES. But
// when it runs, the second `borrow_mut()` finds the cell is still borrowed by `a`,
// and the program PANICS.

// Predict what happens when it RUNS, then run it and read the panic. Make it run
// cleanly — let each change to `tasks` finish before the next one begins, so the
// cell is never borrowed twice at once. Keep both pushes.

// Hint ladder: press Hint (or `rpro exercise hint`).

use std::cell::RefCell;

fn main() {
    let tasks = RefCell::new(vec!["write", "test"]);

    let mut a = tasks.borrow_mut();
    let mut b = tasks.borrow_mut();
    a.push("ship");
    b.push("celebrate");

    println!("{:?}", tasks.borrow());
}
