// `collect` can build many kinds of collection, so the compiler needs you to
// say which one you want. Run it, read the error, and give it the type.
fn main() {
    let doubled = (1..=5).map(|n| n * 2).collect();
    println!("doubled: {doubled:?}");
}
