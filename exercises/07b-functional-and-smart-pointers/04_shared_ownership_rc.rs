// Rustlings Pro — exercises/07b-functional-and-smart-pointers/04_shared_ownership_rc.rs

// CONCEPT: a `Box` gives a value exactly ONE owner. Here two lists, `b` and `c`,
// each want the SAME tail `a` as their rest. Putting `a` inside `b` MOVES it, so
// when `c` reaches for `a` too, it is already gone — the value has one owner, not
// two.

// Run it and read the E0382 error top-to-bottom: "use of moved value: `a`". You
// have met moves before — last time you could clone or borrow. But here BOTH lists
// must keep the shared tail alive for as long as they live: they need to share
// OWNERSHIP, not just borrow it. Reach for a smart pointer that lets one value have
// many owners and frees it only once the last owner is done (it keeps a count of
// how many owners there are). Make `a` shareable so both `b` and `c` can hold it.

// Hint ladder: press Hint (or `rpro exercise hint`).

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a)); // `a` was already moved into `b` on the line above
    let _ = (b, c); // pretend the program goes on to use both lists
}
