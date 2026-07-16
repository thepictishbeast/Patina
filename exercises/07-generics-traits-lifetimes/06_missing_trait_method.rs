// Adapted from Rustlings (MIT/Apache-2.0) — rust-lang/rustlings, traits1
//
// The `impl AppendBar for String` block is empty. Make this compile so it prints
// "FooBar". Read the error — note which method is missing.

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
}

fn main() {
    let s = String::from("Foo").append_bar();
    println!("s: {s}");
}
