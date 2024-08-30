// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Book<'a>{
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
// 由于 name 和 title 的生命周期仅限于 main 函数的作用域，而 book 结构体持有这些引用的生命周期可能超出了它们的实际生命周期
// 这表示 &name 和 &title 的生命周期与 Book 结构体的生命周期不匹配，因为 name 和 title 的生命周期比 Book 结构体的生命周期短。
// 加上注释:示意book, author的引用, title的引用的生命周期是一样的 