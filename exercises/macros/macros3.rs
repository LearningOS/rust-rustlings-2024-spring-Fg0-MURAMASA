// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}
// #[macro_export]属性 -> 可以使得宏在当前crate的其他模块和外部crate中可用

fn main() {
    my_macro!();
}
