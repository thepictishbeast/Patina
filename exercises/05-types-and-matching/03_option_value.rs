// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing `let maybe: Option<i32>`. Read the
// "expected i32, found Option<i32>" error.

fn main() {
    let maybe: Option<i32> = Some(7);

    let value: i32 = maybe;

    println!("the value is {value}");
}
