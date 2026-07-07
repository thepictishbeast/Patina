// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `echo` is called first with a number, then with text. Make the two calls
// consistent, without changing `echo` itself. Run it and read the compiler error.

fn main() {
    let echo = |x| x;
    let n = echo(5);
    let s = echo("hello");
    println!("{n} and {s}");
}
