// Rustlings Pro — exercises/03-text-and-collections/02_string_vs_str.rs

// CONCEPT: `String` (owned, growable, heap) and `&str` (a borrowed view
// into existing text) are different types. A `"..."` literal is a `&str`.
// To get an owned `String`, convert it: `.to_string()` or `String::from`.

// Make this compile WITHOUT changing `let greeting: String`. Read the
// `expected String, found &str` on the error.

// Hint ladder: press Hint (or `rpro exercise hint`).

fn main() {
    // "hello" is a &str literal — not an owned String.
    let greeting: String = "hello";
    println!("{greeting}, world");
}
