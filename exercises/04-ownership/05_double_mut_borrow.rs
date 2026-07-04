// Rust allows at most ONE active mutable reference (`&mut`) to a value at a
// time. That single rule is what guarantees no two parts of a program can
// scribble over the same data at once. This program tries to hold TWO `&mut`
// borrows of `count` alive together, so it does not compile.

// Run it, read the error (note its code), then fix it so the program runs and
// prints the final count.

// Hint: a `&mut` borrow lasts only until its LAST use. If you completely
// finish with the first reference before you create the second, you never
// actually hold two of them at the same time.
fn main() {
    let mut count = 0;
    let first = &mut count;
    let second = &mut count;
    *first += 1;
    *second += 1;
    println!("count = {count}");
}
