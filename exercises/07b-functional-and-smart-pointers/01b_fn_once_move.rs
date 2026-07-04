// Rustlings Pro — exercises/07b-functional-and-smart-pointers/01b_fn_once_move.rs

// CONCEPT: a closure can use a value it captured in one of three ways — just READ
// it, CHANGE it, or GIVE IT AWAY (move it out). A closure that gives a captured
// value away can run only ONCE: after the first call, the value it handed off is
// gone, so there is nothing left to give a second time.

// Here `award` captures `prize` by value (`move`) and then hands it to announce(),
// giving it away. Calling `award()` a second time tries to award a prize that has
// already been given to someone else.

// Run it and read the E0382 error: "use of moved value: `award`" — the closure was
// used up by its first call. Make `award` callable twice: let it LEND the prize
// instead of giving it away (or hand over a copy), without changing what it prints.

// Hint ladder: press Hint (or `rpro exercise hint`).

fn announce(prize: String) {
    println!("and the winner receives: {prize}");
}

fn main() {
    let prize = String::from("a hand-carved Ferris");
    let award = move || announce(prize);
    award();
    award(); // we try to award the same prize a second time
}
