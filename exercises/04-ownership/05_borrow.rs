// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT removing any println!. Read the compiler error.

fn main() {
    let mut greeting = String::from("hello");
    let immut_ref = &greeting;
    let mut_ref = &mut greeting;

    mut_ref.push_str(", world");
    println!("immutable: {}", immut_ref);
    println!("mutable:   {}", mut_ref);
}
