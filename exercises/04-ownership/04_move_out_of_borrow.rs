// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `first` only borrows the vector. Make it compile so it returns the first word.
// Read the compiler error.

fn first(words: &Vec<String>) -> String {
    words[0]
}

fn main() {
    let v = vec![String::from("alpha"), String::from("beta")];
    println!("first word: {}", first(&v));
}
