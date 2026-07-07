// A struct definition is a fixed contract: a struct literal may set ONLY the
// fields the struct declares — no inventing new ones. Both real fields here are
// set correctly, but the literal adds one more that `User` never declared.
//
// Make this compile WITHOUT changing the struct definition or the println!.
// Read the compiler error — it names the field that does not belong.

struct User {
    name: String,
    active: bool,
}

fn main() {
    let u = User { name: String::from("Ada"), active: true, admin: false };

    println!("{} is active: {}", u.name, u.active);
}
