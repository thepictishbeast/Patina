// Rustlings Pro — exercises/07b-functional-and-smart-pointers/03_recursive_box.rs

// CONCEPT: a type that contains ITSELF has no fixed size. Here each `Cons` holds
// the next `List` directly, so to lay `List` out in memory the compiler would
// need infinite space. The fix is to store the inner value behind a pointer of
// KNOWN size — a small handle on the stack that points at the value on the heap.

// Run it and read the E0072 error top-to-bottom: the `help:` line names the exact
// smart pointer the compiler wants. Give `List` a known size without changing
// which values it holds.

// Hint ladder: press Hint (or `rpro exercise hint`).

enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    use List::{Cons, Nil};
    let list = Cons(1, Cons(2, Nil));
    let _ = list; // `List` won't compile until it has a known size
}
