// Following a raw pointer is something Rust makes you ask for explicitly.
// Run it, read the error, and wrap the dereference in the block it names.
fn main() {
    let x = 42;
    let p = &x as *const i32;
    let value = *p;
    println!("value behind the pointer: {value}");
}
