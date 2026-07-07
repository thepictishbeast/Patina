// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `award` is called twice but only works once. Make both calls work, without
// changing what it prints. Run it and read the compiler error.

fn announce(prize: String) {
    println!("and the winner receives: {prize}");
}

fn main() {
    let prize = String::from("a hand-carved Ferris");
    let award = move || announce(prize);
    award();
    award();
}
