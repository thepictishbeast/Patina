// This program should greet AND say goodbye — but BOTH functions were named
// `greet`. Within one scope a name can mean only ONE thing, so the compiler
// can't tell them apart and stops with "the name `greet` is defined multiple
// times".
//
// Give the two functions DISTINCT names, and call each one from main. Don't
// delete either body.

fn greet() {
    println!("Hello!");
}

fn greet() {
    println!("Goodbye!");
}

fn main() {
    greet();
}
