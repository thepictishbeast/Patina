// Assignment MOVES a non-`Copy` value (like `String`) — it doesn't duplicate it,
// so the old name can't be used afterward. Predict: does this compile? Read the
// "borrow of moved value" error, then make BOTH names usable — keep the println!.

fn main() {
    let greeting = String::from("hello");
    let echo = greeting;
    println!("{greeting} ... {echo}");
}
