// While a reference to a value is still in use, the value is "frozen": you
// cannot change the original out from under the borrow. Here `watcher` borrows
// `total`, and the code then tries to reassign `total` while `watcher` is still
// going to be read — so it does not compile.

// Run it, read the error (note its code and the three lines it points at), then
// make it compile so it prints what the watcher saw and the new total.

// Hint: a borrow lasts only until its LAST use. If you finish reading through
// `watcher` before you reassign `total`, then nothing is borrowing `total` at
// the moment you change it.
fn main() {
    let mut total = 100;
    let watcher = &total;
    total = 200;
    println!("watcher saw {watcher}, total is now {total}");
}
