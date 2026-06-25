// `sum()` is a consuming adapter that can total into many numeric types
// (i32, u64, f64, ...). The compiler needs to know which one you mean, and
// this program never says. Run it, read the error, and fix it by telling the
// compiler the total's type.
//
// Hint: the fix goes on the `let` binding (or you can tell `sum` directly).
fn main() {
    let v = vec![1, 2, 3];
    let total = v.iter().sum();
    println!("total: {total}");
}
