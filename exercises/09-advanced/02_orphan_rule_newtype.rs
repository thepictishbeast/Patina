// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// This tries to give `Vec<String>` a custom `Display`, but Rust won't let you
// implement a foreign trait on a foreign type. Make it compile so the list still
// prints. Run it and read the compiler error, including its `help:` line.

use std::fmt;

impl fmt::Display for Vec<String> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.join(", "))
    }
}

fn main() {
    let items = vec![String::from("ferris"), String::from("crab")];
    println!("{items}");
}
