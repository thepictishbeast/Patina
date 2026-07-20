// An or-pattern (`A | B`) lets one `match` arm handle several shapes at once. But a
// single arm body runs no matter which alternative matched — so if the arm binds a
// variable, EVERY alternative must bind that same variable. Here `Msg::Move { x }`
// binds `x` but `Msg::Quit` doesn't. Predict: does this compile? Read the E0408
// error, then give the case that binds `x` its own arm.
enum Msg {
    Move { x: i32 },
    Quit,
}

fn main() {
    let m = Msg::Move { x: 5 };
    match m {
        Msg::Move { x } | Msg::Quit => println!("handled a message"),
    }
}
