// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// Make this compile WITHOUT removing the final println!. Read which value the
// compiler says was "moved".

fn main() {
    let names = vec![String::from("ada"), String::from("alan")];

    for name in names {
        println!("hi {name}");
    }

    println!("there were {} names", names.len());
}
