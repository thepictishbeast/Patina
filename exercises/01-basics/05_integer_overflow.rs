// Rustlings Pro — exercises/01-basics/05_integer_overflow.rs

// CONCEPT: every integer type has a FIXED size, and so a fixed range of values. A
// `u8` is an unsigned 8-bit integer — it can only hold 0 through 255. The math
// below, 20 * 20, is 400, which is too big to fit in a `u8`. Rust does NOT silently
// wrap around and hand you a wrong answer (that is a whole class of real-world
// bugs); in a debug build it CHECKS for overflow and PANICS. So this compiles, then
// blows up when you run it.

// Predict first: will it compile AND run cleanly, or will it panic? Run it and read
// the panic. Then make the calculation fit — the numbers are fine, the TYPE is too
// small for their product. (Change the types `area` works with; leave the numbers in
// the call and the println! as they are.)

// Hint ladder: press Hint (or `rpro exercise hint`).

fn area(width: u8, height: u8) -> u8 {
    width * height
}

fn main() {
    let a = area(20, 20);
    println!("the area is {a}");
}
