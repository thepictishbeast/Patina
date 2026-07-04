// Rustlings Pro — exercises/06-modules/01_private_module.rs

// CONCEPT: modules are PRIVATE by default — including nested modules.
// To reach something across a module boundary, every step of the PATH
// must be public, not just the final item.

// Make this compile WITHOUT changing `main`. Read which module the
// compiler says is private, and add `pub` so the path opens up.

// Hint ladder: press Hint (or `rpro exercise hint`).

mod kitchen {
    // This inner module isn't `pub`, so it's invisible outside `kitchen`
    // even though `preheat` itself is `pub`.
    mod oven {
        pub fn preheat() {
            println!("oven heating");
        }
    }
}

fn main() {
    kitchen::oven::preheat();
}
