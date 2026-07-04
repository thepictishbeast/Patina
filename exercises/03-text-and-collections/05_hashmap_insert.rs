// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: a `HashMap` stores key → value pairs and lets you look a value up by its
// KEY (instead of by position the way a `Vec` uses an index). When you `insert` an
// OWNED value — like a `String` — the map TAKES OWNERSHIP of it: the value moves into
// the map, the same way it would move into another variable. After that, the original
// binding no longer owns it, so it can't use it.

// Here `team` (a `String`) is inserted into `scores`, which moves it into the map. The
// `println!` afterward tries to use `team` again, but it's been given away (E0382: use
// of moved value).

// Run it and read E0382, then make both lines work: let the map have its own copy of
// the name, or print the name BEFORE you hand it over — so you're not using a value
// you've already given away.

use std::collections::HashMap;

fn main() {
    let team = String::from("blue");
    let mut scores = HashMap::new();
    scores.insert(team, 10);
    println!("added the {team} team");
}
