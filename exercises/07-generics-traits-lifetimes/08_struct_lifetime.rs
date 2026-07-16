// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// `Excerpt` stores a `&str` but declares no lifetime. Make it compile — only the
// struct definition needs a change. Read the error and its `help:`.

struct Excerpt {
    part: &str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let e = Excerpt { part: first_sentence };
    println!("excerpt: {}", e.part);
}
