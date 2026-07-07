// A spawned thread tries to use a vector owned by `main`, but this won't compile.
// Make it compile without changing what the thread prints. Run it and read the
// compiler error.
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(|| {
        println!("vector from the thread: {v:?}");
    });
    handle.join().unwrap();
}
