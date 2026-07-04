// A trait can REQUIRE methods that every implementer must provide. `AppendBar`
// requires `append_bar`, but this `impl AppendBar for String` block is empty — it
// claims to implement the trait without actually supplying the method, so it does
// not compile.

// Run it, read the error (note its code and which method it says is missing),
// then implement that method inside the `impl` block so the program prints
// "FooBar".

// Hint: the trait already fixes the method's shape — it takes `self`, returns
// `Self`, and (per the trait's name) should append "Bar". You write only the
// body. The book's "implementing a trait on a type" section shows the form.

// Adapted from Rustlings (rust-lang/rustlings, MIT) — 15_traits/traits1 — rebuilt
// into Tempered Studio's predict-then-run format.
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
}

fn main() {
    let s = String::from("Foo").append_bar();
    println!("s: {s}");
}
