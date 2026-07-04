// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile so it prints the area. Read the error — it names what's missing.

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    let a = area(5);
    println!("area = {a}");
}
