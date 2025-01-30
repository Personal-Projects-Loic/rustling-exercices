macro_rules! my_macro {
    ($x:expr) => {
        println!("Check out my macro! {}", $x);
    };
}

fn main() {
    // TODO: Fix the macro call.
    my_macro!(8);
}
