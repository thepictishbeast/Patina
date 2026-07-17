// Predict first: does this compile? Read the "mismatched types" error, then make
// it compile — you may NOT change the "   " or the `.len()`. (Hint: `mut` keeps a
// binding's TYPE fixed; changing a value's type is exactly what shadowing is for.)

fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();
    println!("that is {spaces} spaces");
}
