// Sharing a reference-counted value with a thread. `Rc` is fast but not safe to
// move between threads. Run it, read the error, and reach for the thread-safe
// cousin instead.
use std::rc::Rc;
use std::thread;

fn main() {
    let shared = Rc::new(5);
    let in_thread = Rc::clone(&shared);
    let handle = thread::spawn(move || {
        println!("value in the thread: {in_thread}");
    });
    handle.join().unwrap();
    println!("value in main: {shared}");
}
