// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Predict first: what stops this compiling? Then run it and read the error — note
// the "temporary value dropped while borrowed" line. Fix it WITHOUT changing
// `first_word`.

fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}

fn main() {
    let word = first_word(&String::from("hello world"));
    println!("the first word is: {word}");
}
