// Predict first: does this compile? Read the "mismatched types" error and the note
// about what the body "implicitly returns", then make it compile WITHOUT adding a
// `return` — one character is in the wrong place.

fn double(n: i32) -> i32 {
    n * 2;
}

fn main() {
    println!("double 21 is {}", double(21));
}
