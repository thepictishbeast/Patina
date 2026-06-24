// Rustlings Pro — exercises/09-advanced/02_orphan_rule_newtype.rs
//
// CONCEPT: you can implement a trait for a type only if YOU define the trait, or YOU
// define the type (or both) — never a foreign trait on a foreign type. This is the
// coherence rule (often called the "orphan rule"), and it stops two unrelated crates
// from each adding a conflicting impl behind your back.
//
// Here `Display` belongs to the standard library and so does `Vec`, so implementing
// one for the other is not allowed (E0117) — even though the code looks reasonable.
//
// Run it and read E0117. To give a FOREIGN type your own behavior, wrap it in a type
// YOU own — a one-field "newtype" struct — and implement the trait on that wrapper
// instead. Print the list however you like, through your own type.
//
// Hint ladder: press Hint (or `rpro exercise hint`).

use std::fmt;

impl fmt::Display for Vec<String> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.join(", "))
    }
}

fn main() {
    let items = vec![String::from("ferris"), String::from("crab")];
    println!("{items}");
}
