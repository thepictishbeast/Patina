// Rustlings Pro — exercises/02-control-flow/03_loop_break_value.rs

// CONCEPT: a `loop` can RETURN a value — you pass it to `break`, and the
// whole `loop { ... }` evaluates to it. But the type you break with must
// match how you use the result.

// Here `total` is declared `u32`, but the loop breaks with the wrong
// type. Make it compile WITHOUT changing the `let total: u32` annotation
// or the println!. Read the `expected`/`found` types on the error.

// Hint ladder: press Hint (or `rpro exercise hint`).

fn main() {
    let mut n = 0;

    let total: u32 = loop {
        n += 1;
        if n == 5 {
            // The loop should yield the running total — but this breaks
            // with the wrong type.
            break "done";
        }
    };

    println!("counted to {total}");
}
