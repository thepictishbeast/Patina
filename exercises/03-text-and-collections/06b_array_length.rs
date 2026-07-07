// The length is part of an array's TYPE. This binding says the type is
// `[i32; 3]` — an array of exactly three i32s — but the value on the right has
// four. Make it compile WITHOUT changing the `[i32; 3]` annotation. Read the
// error: it names both sizes.

fn main() {
    let scores: [i32; 3] = [88, 92, 77, 65];
    println!("first score: {}", scores[0]);
}
