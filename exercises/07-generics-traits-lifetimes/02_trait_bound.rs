// `show` accepts any type that can be displayed. `Point` is handed to it,
// but `Point` hasn't earned that yet. Run it, read the error, and fix it.
use std::fmt::Display;

fn show<T: Display>(value: T) {
    println!("here it is: {value}");
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    show(p);
}
