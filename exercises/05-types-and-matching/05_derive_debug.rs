// To print a value with the `{:?}` debug format, its type must implement the
// `Debug` trait — a struct doesn't get it automatically. Predict: does this
// compile? Read the "`Point` doesn't implement `Debug`" error, then make it print.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 7 };
    println!("the point is {p:?}");
}
