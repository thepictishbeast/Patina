// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Predict first: what stops this compiling? Then run it and read the error.
// Reach the value the SAFE way WITHOUT changing the type of `maybe`, so it prints
// "the number is 7".

fn main() {
    let maybe: Option<i32> = Some(7);

    let Some(n) = maybe;

    println!("the number is {n}");
}
