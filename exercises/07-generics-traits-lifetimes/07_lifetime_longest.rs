// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `longest` returns whichever slice is longer, but the compiler can't tell how long
// the returned reference may live. Make it compile — read the error.

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("a long string");
    let b = String::from("short");
    println!("the longest is: {}", longest(a.as_str(), b.as_str()));
}
