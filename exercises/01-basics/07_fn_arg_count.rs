// Every argument a function declares must be supplied when you call it — no
// more, no fewer. This `area` function needs two (a width and a height), but the
// call passes only one, so it does not compile.

// Run it, read the error (note its code — it even names which argument is
// missing), then make it compile so it prints the area.

// Hint: look at `area`'s definition line — it lists the parameters it expects.
// The call has to supply that many. (You choose the second number.)
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    let a = area(5);
    println!("area = {a}");
}
