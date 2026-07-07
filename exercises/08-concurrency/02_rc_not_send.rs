// This shares a reference-counted value with a spawned thread, but it won't
// compile. Make it compile so both the thread and main can read the value. Run it
// and read the compiler error.
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
