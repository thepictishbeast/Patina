// The `?` operator unwraps a success value or short-circuits on failure — but
// it only works when the failure it would propagate matches what the function
// returns. This function returns a `Result`, yet `?` is used on an `Option`
// (the value from `.last()`). An `Option`'s failure is `None`, which carries no
// error value to put in the `Result`, so Rust refuses to compile it.

// Run it, read the error — note its code AND the method the compiler suggests
// — then make it compile so the program prints the last word.

// Hint: turn the `Option` into a `Result` *before* the `?`, by supplying the
// error value to use when it is `None`. The compiler names the method that
// does exactly that.
fn last_word(text: &str) -> Result<String, String> {
    let word = text.split_whitespace().last()?;
    Ok(word.to_string())
}

fn main() {
    println!("{:?}", last_word("learning rust offline"));
}
