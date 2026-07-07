// Five threads each try to add one to a shared counter, but this won't compile.
// Make all five increments happen safely so the final count is 5. Run it and read
// the compiler error.
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(0);
    let mut handles = vec![];
    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            *c += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("final count: {}", *counter);
}
