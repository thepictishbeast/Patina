// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: `match` must be EXHAUSTIVE — it has to cover every possible
// value. The compiler checks this for you: if a case is missing, it
// won't compile. That's a feature — you can't forget a case.

// Make this compile WITHOUT changing `light`. Read which pattern the
// compiler says is "not covered" and add an arm for it (or a `_`
// catch-all). Keep the println! for each handled case.

enum Light {
    Red,
    Yellow,
    Green,
}

fn action(light: Light) -> &'static str {
    // One variant is missing here. The compiler will name it.
    match light {
        Light::Red => "stop",
        Light::Green => "go",
    }
}

fn main() {
    println!("{}", action(Light::Yellow));
}
