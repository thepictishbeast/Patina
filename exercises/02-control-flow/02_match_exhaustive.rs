// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT changing `light`. Read the compiler error — it names
// the case you're missing.

enum Light {
    Red,
    Yellow,
    Green,
}

fn action(light: Light) -> &'static str {
    match light {
        Light::Red => "stop",
        Light::Green => "go",
    }
}

fn main() {
    println!("{}", action(Light::Yellow));
}
