// Rustlings Pro — exercises/09-advanced/04_operator_overload_add.rs
//
// CONCEPT: operators like `+` aren't magic built-ins reserved for numbers — each one
// is backed by a TRAIT. `a + b` is really `a.add(b)`, from the `std::ops::Add` trait.
// Number types come with `Add` already implemented; your own types don't, so `+`
// isn't available on them until you implement that trait yourself.
//
// `Point` has no `Add` impl, so `a + b` has no `add` method to call (E0369: cannot
// add `Point` to `Point`).
//
// Run it and read E0369, then teach `Point` how to add: implement the operator's
// trait so `a + b` returns a new `Point` whose fields are the two points' fields added
// together. Don't change `main`.
//
// Hint ladder: press Hint (or `rpro exercise hint`).

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 3, y: 4 };
    let sum = a + b;
    println!("{a:?} + {b:?} = {sum:?}");
}
