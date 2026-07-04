// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: move semantics — when assigning one variable to another
// transfers ownership instead of copying.

// Make this compile WITHOUT removing the println! call. The runner
// expects E0382 ("borrow of moved value") to appear in the failing
// version; if you see a different error, you may have changed too
// much. Run `rpro exercise hint` for the book sections that explain
// this.

// Hint of last resort: `rpro exercise hint --solution`.

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{} {}", s1, s2);
}
