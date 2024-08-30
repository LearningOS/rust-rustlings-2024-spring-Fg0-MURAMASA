// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
    // "milk" 是一个字符串字面量，它的类型是 &'static str。这个类型的 &str 是一个静态生命周期的字符串切片，代表对静态分配内存中的字符串数据的引用。
    // 当你将 "milk" 放入 shopping_list 中时，实际的类型是 &str，因此 shopping_list 应该是 Vec<&str>。
}
