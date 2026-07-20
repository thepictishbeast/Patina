// Predict first: does this compile AND run cleanly, or panic? Run it, read the
// outcome, then make it run cleanly — divide only when it is safe to.

fn people_at_the_table() -> u32 {
    0 // nobody has sat down yet
}

fn main() {
    let cookies: u32 = 24;
    let people = people_at_the_table();
    let each = cookies / people;
    println!("each person gets {each} cookies");
}
