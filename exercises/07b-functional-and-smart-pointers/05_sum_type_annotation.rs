// This totals the vector but never says what numeric type the total is, so it
// won't compile. Fix it without changing the numbers. Run it and read the error.
fn main() {
    let v = vec![1, 2, 3];
    let total = v.iter().sum();
    println!("total: {total}");
}
