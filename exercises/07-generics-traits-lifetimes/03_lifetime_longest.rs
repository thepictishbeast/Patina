// `longest` returns whichever string slice is longer. The compiler can't tell
// how long the returned reference is allowed to live. Run it, read the error,
// and add what it asks for.
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
