// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing the `let total: u32` annotation or the
// println!. Read the `expected`/`found` types on the error.

fn main() {
    let mut n = 0;

    let total: u32 = loop {
        n += 1;
        if n == 5 {
            break "done";
        }
    };

    println!("counted to {total}");
}
