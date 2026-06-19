// Five threads each try to add one to a shared counter. Sharing it with `Arc`
// is fine, but `Arc` alone gives you read-only access. Run it, read the error,
// and add the piece that allows safe mutation.
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
