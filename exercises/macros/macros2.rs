// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
//这个讲究顺序的
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}


fn main() {
    my_macro!();
}

