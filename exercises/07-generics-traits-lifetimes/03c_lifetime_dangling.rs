// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `r` borrows `x`, but `x` lives only inside the inner block. Make this compile so
// it prints the value. Read the error — note the "dropped here while borrowed" line.

fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("the value is {r}");
}
