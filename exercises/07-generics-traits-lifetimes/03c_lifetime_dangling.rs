// A reference must never outlive the value it points to. Here `r` is meant to
// hold a reference, but the value it borrows (`x`) lives only inside the inner
// block — by the time `r` is read, `x` has already been dropped. This is the
// dangling reference the borrow checker exists to prevent, so it does not
// compile. (This is *why* lifetimes exist — no `'a` syntax is needed to hit it.)

// Run it, read the error (note its code and the "dropped here while still
// borrowed" line), then make it compile so it prints the value.

// Hint: the borrow has to stay valid for as long as `r` is used. Either keep
// the borrowed value alive long enough, or do the reading while it is still in
// scope — this is about *when* values are dropped, not about annotations.
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("the value is {r}");
}
