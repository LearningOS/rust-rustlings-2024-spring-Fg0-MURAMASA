// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    (1..=num).product()
    //(1..=num) 创建了一个从 1 到 num 的范围，包括 num。
    // product() 是一个迭代器适配器，它会计算所有迭代器元素的乘积。
    // 使用递归:
    // match num {
    //     0 | 1 => 1, // 基本情况：0! 和 1! 都是 1
    //     _ => num * factorial(num - 1), // 递归调用
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
