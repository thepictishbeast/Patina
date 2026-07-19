// Rust has no "truthy" numbers — an `if` condition must be an actual `bool`.
// Predict: does this compile? Read the "expected `bool`, found integer" error,
// then make it compile by asking a real yes/no question.

fn main() {
    let temperature = 30;
    if temperature {
        println!("warm out");
    } else {
        println!("chilly");
    }
}
