// This reads the value behind a raw pointer, but Rust won't compile it as-is.
// Make it compile without changing what it prints. Run it and read the compiler
// error.
fn main() {
    let x = 42;
    let p = &x as *const i32;
    let value = *p;
    println!("value behind the pointer: {value}");
}
