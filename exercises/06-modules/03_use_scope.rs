// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: you call items by PATH. A name from inside a module isn't in
// scope at the crate root unless you spell out its full path or bring it
// in with `use`.

// Make this compile WITHOUT changing the modules. Either call the full
// path (`garden::vegetables::plant()`) or add a `use` line. The error
// says it can't resolve `vegetables` — because it lives under `garden`.

mod garden {
    pub mod vegetables {
        pub fn plant() {
            println!("planted a row");
        }
    }
}

fn main() {
    // `vegetables` isn't in scope here.
    vegetables::plant();
}
