// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: a `let` binding attaches a NAME to a value — but until you give it a
// value, the name holds nothing, and Rust won't let you READ it. You may write
// `let x;` and assign on a later line, but USING it before that assignment does
// not compile: the compiler proves every read happens after a write.

// PREDICT first: what stops this compiling? Then run it and read the error —
// note its code and which binding it says isn't initialized.

// Fix it WITHOUT removing the `println!`: make sure `count` has a value before
// the line that reads it. (The book's "variables" section introduces binding a
// name to a value.)

fn main() {
    let count: i32;
    println!("the count is {count}");
}
