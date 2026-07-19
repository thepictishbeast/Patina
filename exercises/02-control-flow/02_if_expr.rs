// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT deleting the println!. Read the compiler's `help:`.

fn main() {
    let temperature = 18;

    let label = if temperature >= 20 {
        "warm"
    };

    println!("It's {label} today.");
}
