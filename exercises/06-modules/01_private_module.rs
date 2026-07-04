// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing `main`. Read which module the compiler says
// is private.

mod kitchen {
    mod oven {
        pub fn preheat() {
            println!("oven heating");
        }
    }
}

fn main() {
    kitchen::oven::preheat();
}
