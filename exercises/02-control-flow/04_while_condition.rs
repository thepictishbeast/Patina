// A `while` loop repeats as long as its condition is `true` — and that condition
// must be a real `bool`, not a number. Predict: does this compile? Read the error,
// then make the countdown actually stop.

fn main() {
    let mut remaining = 3;
    while remaining {
        println!("{remaining}...");
        remaining -= 1;
    }
    println!("liftoff!");
}
