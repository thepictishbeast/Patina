// This doubles the numbers, then tries to print the original vector — but it does
// not compile. Fix it so BOTH lines print, without changing what they show. Run
// it and read the compiler error.
fn main() {
    let v = vec![1, 2, 3];
    let doubled: Vec<i32> = v.into_iter().map(|x| x * 2).collect();
    println!("doubled:  {doubled:?}");
    println!("original: {v:?}");
}
