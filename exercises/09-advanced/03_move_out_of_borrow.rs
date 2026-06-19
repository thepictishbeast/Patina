// `first` only borrows the vector, but tries to hand back something it doesn't
// own. Run it, read the error, and return a value you're allowed to give away.
fn first(words: &Vec<String>) -> String {
    words[0]
}

fn main() {
    let v = vec![String::from("alpha"), String::from("beta")];
    println!("first word: {}", first(&v));
}
