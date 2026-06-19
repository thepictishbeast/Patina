// A spawned thread tries to print a vector owned by `main`. The compiler
// can't prove the vector outlives the thread. Run it, read the error, fix it.
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(|| {
        println!("vector from the thread: {v:?}");
    });
    handle.join().unwrap();
}
