// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings

// CONCEPT: `for x in v` CONSUMES the vector — it moves `v` into the loop,
// so you can't use `v` afterward. To keep the vector, iterate over a
// reference: `for x in &v` (then `x` is a `&T`).

// Make this compile WITHOUT removing the final println!. Read which value
// the compiler says was "moved", and borrow it in the loop instead.

fn main() {
    let names = vec![String::from("ada"), String::from("alan")];

    for name in names {
        println!("hi {name}");
    }

    // `names` was moved into the for loop above.
    println!("there were {} names", names.len());
}
