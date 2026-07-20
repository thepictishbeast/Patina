// Every value is dropped automatically when it goes out of scope — that is how Rust
// frees memory without a garbage collector. Because that cleanup is automatic, Rust
// FORBIDS calling the destructor yourself: `w.drop()` would drop it now, and the
// automatic drop at the end of `main` would drop it AGAIN — a double free. Predict:
// does this compile? Read the E0040 error, then drop it early the allowed way.
struct Widget;

impl Drop for Widget {
    fn drop(&mut self) {
        println!("Widget dropped");
    }
}

fn main() {
    let w = Widget;
    w.drop();
    println!("done");
}
