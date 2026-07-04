// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: a `Vec` is indexed with `[i]`, and the indexes are ZERO-BASED — the
// three scores below live at index 0, 1, and 2. There is no fourth element, so
// `scores[3]` asks for something that isn't there. Rust does NOT read past the end
// and hand you whatever happens to be in memory (that would be a safety hole) — it
// CHECKS the bound at run time and PANICS instead. So this code compiles fine; it
// only blows up when you run it.

// Predict first: will it compile AND run cleanly, or will it panic? Run it and read
// the panic — it tells you the length and the index you asked for. Then make the
// lookup land on a score that actually exists (don't delete the lookup).

fn main() {
    let scores = vec![88, 92, 77]; // three scores, at index 0, 1, and 2
    let pick = scores[3];
    println!("the score is {pick}");
}
