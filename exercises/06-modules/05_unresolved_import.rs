// Rustlings Pro — exercises/06-modules/05_unresolved_import.rs

// CONCEPT: a `use` brings a path from the module tree into scope under a short
// name — but the path has to point at something that ACTUALLY EXISTS. The `use`
// below names a module that isn't in the tree (look closely at the spelling),
// so the import can't be resolved and the program does not compile.

// PREDICT first: what stops this compiling? Then run it and read the error —
// note its code, and exactly which path it says is unresolved.

// Fix it WITHOUT changing the `mod shapes` block, the struct, or `main`: correct
// the path in the `use` so it points at the real module. (The book's "use"
// section shows how a path names its way down the module tree.)

// Hint ladder: press Hint (or `rpro exercise hint`).
mod shapes {
    pub struct Circle {
        pub radius: f64,
    }
}

use crate::shape::Circle;

fn main() {
    let c = Circle { radius: 2.0 };
    println!("a circle of radius {}", c.radius);
}
