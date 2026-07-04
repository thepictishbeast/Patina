// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile so it prints both. Read the compiler error.

fn main() {
    let v = vec![1, 2, 3];
    let r = &v;
    let v2 = v;
    println!("{r:?} and {v2:?}");
}
