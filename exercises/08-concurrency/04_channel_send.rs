// Inspired by Rustlings (MIT/Apache-2.0) — github.com/rust-lang/rustlings
//
// The spawned thread sends `message` down the channel, then tries to use it
// again — so it won't compile. Make it compile so main still receives the
// message. Run it and read the compiler error.

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let message = String::from("hello from the spawned thread");

    thread::spawn(move || {
        tx.send(message).unwrap();
        println!("just sent: {message}");
    });

    println!("main got: {}", rx.recv().unwrap());
}
