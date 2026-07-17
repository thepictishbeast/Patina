// Predict first: does this compile? A `const` is a value fixed forever and known
// at compile time. Read the error, then make it compile WITHOUT changing the 100.

const MAX_SCORE = 100;

fn main() {
    println!("the most you can score is {MAX_SCORE}");
}
