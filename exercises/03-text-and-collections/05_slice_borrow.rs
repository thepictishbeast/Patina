// A slice `&[T]` is a borrowed WINDOW into a sequence — you hand over a reference,
// not the owned collection. Predict: does this compile? Read the "expected `&[i32]`,
// found `Vec`" error, then make it compile WITHOUT changing `first_three`.

fn first_three(window: &[i32]) -> i32 {
    window[0] + window[1] + window[2]
}

fn main() {
    let scores = vec![10, 20, 30, 40, 50];
    println!("sum of first three: {}", first_three(scores));
}
