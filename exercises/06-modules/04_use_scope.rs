// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing the modules. Read the error — it can't resolve
// `vegetables`.

mod garden {
    pub mod vegetables {
        pub fn plant() {
            println!("planted a row");
        }
    }
}

fn main() {
    vegetables::plant();
}
