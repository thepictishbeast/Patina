// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `Sku` implements `Labelled`, but `Labelled` requires more than `Sku` currently
// provides, so this won't compile. Satisfy that requirement without changing the
// `Labelled` trait or `main`. Run it and read the compiler error.

use std::fmt;

trait Labelled: fmt::Display {
    fn labelled(&self) -> String {
        format!("[{self}]")
    }
}

struct Sku {
    code: u32,
}

impl Labelled for Sku {}

fn main() {
    let s = Sku { code: 7 };
    println!("{}", s.labelled());
}
