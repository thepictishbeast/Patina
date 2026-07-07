// This builds a doubled collection but never says what KIND, so it won't compile.
// Fix it without changing the numbers. Run it and read the compiler error.
fn main() {
    let doubled = (1..=5).map(|n| n * 2).collect();
    println!("doubled: {doubled:?}");
}
