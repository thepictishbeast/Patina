// Predict first: does this compile AND run cleanly, or panic? Run it, read the
// outcome, then make it run cleanly — an unsigned count can't go below zero.

fn coins_in_pocket() -> u32 {
    0 // the pocket is empty
}

fn main() {
    let coins = coins_in_pocket();
    let after_spending = coins - 1;
    println!("{after_spending} coins left");
}
