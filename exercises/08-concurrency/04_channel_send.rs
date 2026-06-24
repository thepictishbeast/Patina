// Rustlings Pro — exercises/08-concurrency/04_channel_send.rs
//
// CONCEPT: besides sharing memory (Arc/Mutex), threads can talk by MESSAGE PASSING —
// one thread SENDS a value down a channel and another RECEIVES it. The key rule:
// sending a value MOVES it. Ownership travels down the channel to the receiver, so
// once you have sent something, it is no longer yours to use on the sending side.
//
// Here the spawned thread sends `message`, then tries to print it again. Because
// `send` moved `message` into the channel, that second use is a use-after-move
// (E0382) — the value is gone from here.
//
// Run it and read E0382. Let the thread send the message OR keep using it, not both:
// print it before the send, or send a copy if both sides need one. The receiver in
// main should still get the message.
//
// Hint ladder: press Hint (or `rpro exercise hint`).

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
