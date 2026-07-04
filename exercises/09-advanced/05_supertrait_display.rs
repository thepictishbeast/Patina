// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: a trait can REQUIRE another trait — written `trait Labelled: Display`,
// which reads "to implement Labelled, a type must ALSO implement Display." That
// required trait is a SUPERTRAIT. Requiring it lets the trait's own methods rely on
// the parent's behaviour: `labelled` below uses `{self}`, which only works because
// every `Labelled` type is guaranteed to also be `Display`.

// `Sku` implements `Labelled` but NOT `Display`, so it breaks the supertrait's
// promise (E0277: `Sku` doesn't implement `Display`).

// Run it and read E0277, then make `Sku` printable so it can satisfy the supertrait:
// implement `Display` for `Sku` (decide how a SKU should print). Don't change the
// `Labelled` trait or `main`.

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
