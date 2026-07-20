// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile so it prints what the watcher saw and the new total. Read the
// compiler error.

fn main() {
    let mut total = 100;
    let watcher = &total;
    total = 200;
    println!("watcher saw {watcher}, total is now {total}");
}
