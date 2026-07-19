// `Point<T>` has ONE type parameter `T`, and BOTH fields are declared `T` — so the
// compiler forces `x` and `y` to be the SAME concrete type inside one `Point`. Here
// `x: 5` fixes `T = i32`, then `y: 4.2` (a float) contradicts it. Predict: does this
// compile? Read the E0308 "mismatched types" error, then make both fields one type.
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p = Point { x: 5, y: 4.2 };
    println!("({}, {})", p.x, p.y);
}
