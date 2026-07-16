// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Predict first: will this compile? Run it and read E0382 (use of moved value),
// then make both lines work WITHOUT deleting either.

use std::collections::HashMap;

fn main() {
    let team = String::from("blue");
    let mut scores = HashMap::new();
    scores.insert(team, 10);
    println!("added the {team} team");
}
