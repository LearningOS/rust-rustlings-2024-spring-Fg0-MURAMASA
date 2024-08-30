// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(&mut vec0);

    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

// 在fill_vec函数中创建一个新的Vec<i32>变量vec1：这个新的Vec变量用于存储需要的元素
// 将vec0传递给fill_vec函数时传递引用：vec0 .不再直接传递所有权，而是通过引用传递，这样在fill_vec函数中可以修改vec0
// 这样，vec0的所有权不会被转移，vec1在fill_vec函数中创建并返回，最终可在main函数中同时访问vec0和vec1
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut vec1 = Vec::new();

    vec1.push(22);
    vec1.push(44);
    vec1.push(66);

    vec1
}
