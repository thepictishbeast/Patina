// While a value is borrowed, you can't move it away, either — moving it would
// leave the existing reference (`r`) pointing at nothing. Here `r` borrows `v`,
// then the code moves `v` into `v2` while `r` is still used afterward, so it
// does not compile.
//
// Run it, read the error (note its code and the "borrow later used here" line),
// then make it compile so it prints both.
//
// Hint: this is the move-twin of the assign-while-borrowed exercise — a borrow
// lasts only until its LAST use. If you finish using `r` before moving `v`,
// nothing is borrowing `v` at the moment of the move.
fn main() {
    let v = vec![1, 2, 3];
    let r = &v;
    let v2 = v;
    println!("{r:?} and {v2:?}");
}
