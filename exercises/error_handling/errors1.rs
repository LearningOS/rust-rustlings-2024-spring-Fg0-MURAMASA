// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
// Result<String, String> 是 Rust 中用于处理可能失败的操作的一种枚举类型
// Ok(T)：表示操作成功，并包含一个类型为 T 的值。T 是成功时返回的数据类型。在你的例子中，T 是 String，表示成功时返回的名牌文本。
// Err(E)：表示操作失败，并包含一个类型为 E 的错误信息。E 是错误类型。在你的例子中，E 也是 String，表示失败时的错误信息。
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
            Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}",name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
