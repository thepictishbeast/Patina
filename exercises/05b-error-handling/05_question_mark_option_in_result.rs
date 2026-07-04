// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `?` is used on an `Option` inside a function that returns a `Result`. Make this
// compile so it prints the last word. Read the error and the method the compiler
// suggests.

fn last_word(text: &str) -> Result<String, String> {
    let word = text.split_whitespace().last()?;
    Ok(word.to_string())
}

fn main() {
    println!("{:?}", last_word("learning rust offline"));
}
