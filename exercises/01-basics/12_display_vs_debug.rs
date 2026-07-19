// `{}` uses the Display format, which structs do NOT get automatically — and,
// unlike Debug, Display can't be derived. Predict: does this compile? Read the
// "doesn't implement `Display`" error, then print the point a way that works.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("the point is {p}");
}
