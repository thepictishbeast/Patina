// The `*` operator DEREFERENCES a reference: given a `&T`, `*` follows the
// reference back to the `T` it points at. It only applies to references — there
// is no reference here to follow. Look at the type of `count`, then read the
// compiler error: it names the type that cannot be dereferenced.
//
// Make this compile WITHOUT removing the println!.

fn main() {
    let count = 10;
    let doubled = *count * 2;

    println!("doubled is {doubled}");
}
