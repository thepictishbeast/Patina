// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: a plain `let` binds an IRREFUTABLE pattern — one that ALWAYS
// matches. `Some(n)` is REFUTABLE: an `Option` might be `None`, so the
// pattern can fail. A bare `let` has nowhere to go when it fails, so the
// compiler rejects it rather than let a binding silently not happen.

// PREDICT first: what stops this compiling? Then run it and read the
// error (note its code, and the variant it says "isn't covered").

// Reach the value the SAFE way WITHOUT changing the type of `maybe` — the
// compiler even names the construct you need. There is more than one right
// answer here (`if let`, `match`, or `let ... else`); pick one and make it
// print "the number is 7".

fn main() {
    let maybe: Option<i32> = Some(7);

    // A plain `let` can't destructure something that might be `None`.
    let Some(n) = maybe;

    println!("the number is {n}");
}
