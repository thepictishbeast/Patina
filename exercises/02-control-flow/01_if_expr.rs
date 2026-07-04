// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: in Rust, `if` is an EXPRESSION — it evaluates to a value.
// When you bind that value with `let`, every branch must produce the
// same type, AND an `if` used as a value needs an `else` (what would
// the value be when the condition is false?).

// Make this compile WITHOUT deleting the println!. Read the compiler's
// `-->` line and its `help:` — it tells you what's missing.

fn main() {
    let temperature = 18;

    // This binds the *value* of the `if` to `label` — but there's no
    // `else`, so on a cold day there's no value to bind.
    let label = if temperature >= 20 {
        "warm"
    };

    println!("It's {label} today.");
}
