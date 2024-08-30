// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}//在 macro_rules! 宏定义中，每个模式和其对应的代码块之间需要使用分号;分隔。

fn main() {
    my_macro!();
    my_macro!(7777);
}
