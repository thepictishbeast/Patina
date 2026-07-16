// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT moving `tax_rate`. Read the error — `checkout` can't
// find `tax_rate` in its scope.

mod store {
    fn tax_rate() -> f64 {
        0.08
    }

    pub mod checkout {
        pub fn total(subtotal: f64) -> f64 {
            subtotal * (1.0 + tax_rate())
        }
    }
}

fn main() {
    println!("{:.2}", store::checkout::total(100.0));
}
