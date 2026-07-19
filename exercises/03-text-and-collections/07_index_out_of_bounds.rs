// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Predict first: does this compile AND run cleanly, or panic? Run it and read the
// panic (it names the length and the index). Then make the lookup land on a score
// that exists — don't delete the lookup.

fn main() {
    let scores = vec![88, 92, 77];
    let pick = scores[3];
    println!("the score is {pick}");
}
