// Rustlings Pro — exercises/06-modules/04_super_path.rs

// CONCEPT: a child module can reach an item in its PARENT module with
// `super::` (like `..` in a file path). Without it, a name from the
// parent isn't in the child's scope.

// Make this compile WITHOUT moving `tax_rate`. Inside `checkout`, reach
// the parent's function with `super::`. The error says it can't find
// `tax_rate` in the child's scope.

// Hint ladder: press Hint (or `rpro exercise hint`).

mod store {
    fn tax_rate() -> f64 {
        0.08
    }

    pub mod checkout {
        pub fn total(subtotal: f64) -> f64 {
            // tax_rate lives in the PARENT module, not here.
            subtotal * (1.0 + tax_rate())
        }
    }
}

fn main() {
    println!("{:.2}", store::checkout::total(100.0));
}
