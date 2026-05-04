// Patina — exercises/04-ownership/02_borrow.rs
//
// CONCEPT: borrow checker — Rust forbids a mutable reference while
// any immutable reference is live. Two people holding immutable refs
// to the same data: fine. One person holding a mutable ref: also
// fine. Both at the same time: not allowed, because the holder of
// the immutable ref might be reading exactly when the mutator
// changes the value out from under them.
//
// Make this compile WITHOUT removing any of the println! calls.
// You can REORDER the references / println!s, or you can change
// the second borrow's mutability. The failing version produces
// E0502 ("cannot borrow ... as mutable because it is also borrowed
// as immutable"). Run `rpro exercise hint` for the book sections
// that explain why.
//
// Hint of last resort: `rpro exercise hint --solution`.

fn main() {
    let mut greeting = String::from("hello");
    let immut_ref = &greeting;
    let mut_ref = &mut greeting;

    mut_ref.push_str(", world");
    println!("immutable: {}", immut_ref);
    println!("mutable:   {}", mut_ref);
}
