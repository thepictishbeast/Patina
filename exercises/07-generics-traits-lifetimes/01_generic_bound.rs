// A generic function that tries to find the largest item in a slice.
// It does not compile yet. Run it, read the error, and fix the signature.
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
