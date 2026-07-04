// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// A generic function that finds the largest item in a slice. Make it compile —
// read the error and fix the signature.

fn largest<T>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let numbers = [34, 50, 25, 100, 65];
    println!("the largest is {}", largest(&numbers));
}
