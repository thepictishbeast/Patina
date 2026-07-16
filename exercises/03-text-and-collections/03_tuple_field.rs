// A tuple's fields are numbered from ZERO — a three-element tuple has `.0`,
// `.1`, and `.2`, and that's all. This reaches for one past the end. Fix it so
// it prints the LAST element (30) WITHOUT changing the tuple. Read the error:
// it lists the fields that actually exist.

fn main() {
    let point = (10, 20, 30);
    println!("last = {}", point.3);
}
