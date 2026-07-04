// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Predict first: does this compile AND run cleanly, or panic? Run it and read the
// outcome, then make it run cleanly — leave the numbers in the call and the
// println! exactly as they are.

fn area(width: u8, height: u8) -> u8 {
    width * height
}

fn main() {
    let a = area(20, 20);
    println!("the area is {a}");
}
